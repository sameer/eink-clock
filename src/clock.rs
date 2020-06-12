use cairo::Context;
use chrono::prelude::*;
use metar::{Data, Metar, SpeedUnit, Clouds, CloudType, CloudLayer};

use crate::render::set_font;
use crate::{TemperatureUnits, WindSpeedUnits, DPI, FONT, HEIGHT, WIDTH};

pub fn draw_clock(ctx: &Context, date_time: &DateTime<Local>, current_metar: Metar<'_>) {
    ctx.set_source_rgb(1.0, 1.0, 1.0);
    draw_date(ctx, date_time.date());
    draw_time(ctx, &date_time);
    draw_current_weather(ctx, current_metar);
}

fn draw_date(ctx: &Context, date: Date<Local>) {
    let date = format!("{}", date.format("%A %B %_d, %Y"));

    set_font(ctx, FONT);
    ctx.set_font_size(DPI * 0.50);
    let extents = ctx.text_extents(&date);
    ctx.move_to((WIDTH as f64 - extents.width) / 2.0, extents.height);
    ctx.show_text(&date);
}

fn draw_time(ctx: &Context, date_time: &DateTime<Local>) {
    let (_, hour) = date_time.hour12();
    let time = format!("{}{}", hour, date_time.format(":%M %p"));

    set_font(ctx, FONT);
    ctx.set_font_size(DPI * 1.5);
    let extents = ctx.text_extents(&time);
    ctx.move_to((WIDTH as f64 - extents.width) / 2.0, HEIGHT as f64);
    ctx.show_text(&time);
}

fn draw_current_weather(ctx: &Context, current_metar: Metar<'_>) {
    // if let None = current_observation.weather {
    //     return;
    // }
    // let weather = current_observation.weather.unwrap();

    let mut concise_observation = String::new();
    if let Data::Known(temp) = &current_metar.temperature {
        use uom::si::f32::ThermodynamicTemperature;
        use uom::si::thermodynamic_temperature::degree_celsius;
        let temp_celsius = ThermodynamicTemperature::new::<degree_celsius>(*temp as f32);
        let temp_fahrenheit = temp_celsius.get::<TemperatureUnits>();
        concise_observation += &format!("{:.1}", temp_fahrenheit);
        concise_observation += "°F";
    }
    if let Data::Known(wind_speed) = &current_metar.wind.speed {
        if let Data::Known(_) = &current_metar.temperature {
            concise_observation += " ";
        }
        use uom::si::f32::Velocity;
        use uom::si::velocity::{knot, meter_per_second};
        let velocity = match wind_speed.unit {
            SpeedUnit::Knot => Velocity::new::<knot>(wind_speed.speed as f32).get::<WindSpeedUnits>(),
            SpeedUnit::MetresPerSecond => Velocity::new::<meter_per_second>(wind_speed.speed as f32).get::<WindSpeedUnits>()
        };
        concise_observation += &format!("{:.1}", velocity);
        concise_observation += " mph";
    }
    eprintln!("{:?}", current_metar);

    set_font(ctx, FONT);
    ctx.set_font_size(DPI * 0.75);
    let extents = ctx.text_extents(&concise_observation);
    ctx.move_to((WIDTH as f64 - extents.width) / 2., HEIGHT as f64 / 4.);
    ctx.show_text(&concise_observation);

    let weather_emoji = if let Data::Known(Clouds::CloudLayers) = current_metar.clouds {
        "\u{263c}"
    } else if let Data::Known(Clouds::SkyClear) =  current_metar.clouds {
        "\u{263c}" // sunny
    } else {
        "\u{003f}"
    };

    // let weather_emoji = match weather.as_str() {
    //     "Thunderstorm" => "⛈️",
    //     "Thunderstorm in Vicinity" => "🌦️",
    //     "Overcast" => "☁️",
    //     "Mostly Cloudy" => "🌥️",
    //     "Partly Cloudy" => "⛅",
    //     "A Few Clouds" => "🌤️",
    //     "Fair" => "☀️",
    //     other => other,
    // }
    // .to_owned();
    // set_font(ctx, "OpenMoji-f");
    ctx.set_font_size(DPI * 1.5);
    let extents = ctx.text_extents(&weather_emoji);
    ctx.move_to((WIDTH as f64 - extents.width) / 2., HEIGHT as f64 * 0.6);
    ctx.show_text(&weather_emoji);
}
