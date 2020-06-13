use cairo::Context;
use chrono::prelude::*;
use metar::{
    CloudLayer, CloudType, Clouds, Data, Metar, SpeedUnit, WeatherCondition, WeatherIntensity,
};

use crate::render::set_font;
use crate::{DPI, EMOJI_FONT, FONT, HEIGHT, TEMPERATURE_UNITS, WIDTH, WIND_SPEED_UNITS};

pub fn draw_clock(ctx: &Context, date_time: &DateTime<Local>, current_metar: Metar<'_>) {
    ctx.set_source_rgb(1.0, 1.0, 1.0);
    ctx.rectangle(0.0, 0.0, WIDTH as f64, HEIGHT as f64);
    ctx.fill();
    ctx.set_source_rgb(0.0, 0.0, 0.0);

    // ctx.set_source_rgb(0.0, 0.0, 0.0);
    draw_date(ctx, date_time.date());
    draw_time(ctx, &date_time);
    draw_current_weather(ctx, current_metar);
    draw_art(ctx, &date_time);
}

fn draw_date(ctx: &Context, date: Date<Local>) {
    let date = format!("{}", date.format("%A %B %_d, %Y"));

    set_font(ctx, FONT);
    ctx.set_font_size(DPI * 0.50);
    let extents = ctx.text_extents(&date);
    ctx.move_to((WIDTH as f64 - extents.width) / 2.0, extents.height);
    ctx.show_text(&date);
    ctx.stroke();
}

fn draw_time(ctx: &Context, date_time: &DateTime<Local>) {
    let (_, hour12) = date_time.hour12();
    let time = format!("{}{}", hour12, date_time.format(":%M %p"));

    set_font(ctx, FONT);
    ctx.set_font_size(DPI * 1.5);
    let extents = ctx.text_extents(&time);
    ctx.move_to(
        (WIDTH as f64 - extents.width) / 2.0,
        HEIGHT as f64 + extents.height + extents.y_bearing,
    );
    ctx.show_text(&time);
}

fn draw_current_weather(ctx: &Context, current_metar: Metar<'_>) {
    // if let None = current_observation.weather {
    //     return;
    // }
    // let weather = current_observation.weather.unwrap();
    use uom::fmt::DisplayStyle;

    let mut concise_observation = String::new();
    if let Data::Known(temp) = &current_metar.temperature {
        use uom::si::f32::ThermodynamicTemperature;
        use uom::si::thermodynamic_temperature::degree_celsius;
        let temp_celsius = ThermodynamicTemperature::new::<degree_celsius>(*temp as f32);
        concise_observation += &format!(
            "{:.1}",
            temp_celsius.into_format_args(TEMPERATURE_UNITS, DisplayStyle::Abbreviation)
        );
    }
    if let Data::Known(wind_speed) = &current_metar.wind.speed {
        if let Data::Known(_) = &current_metar.temperature {
            concise_observation += " ";
        }
        use uom::si::f32::Velocity;
        use uom::si::velocity::{knot, meter_per_second};
        let velocity = match wind_speed.unit {
            SpeedUnit::Knot => Velocity::new::<knot>(wind_speed.speed as f32),
            SpeedUnit::MetresPerSecond => {
                Velocity::new::<meter_per_second>(wind_speed.speed as f32)
            }
        };
        concise_observation += &format!(
            "{:.1}",
            velocity.into_format_args(WIND_SPEED_UNITS, DisplayStyle::Abbreviation)
        );
    }

    set_font(ctx, FONT);
    ctx.set_font_size(DPI * 0.75);
    let extents = ctx.text_extents(&concise_observation);
    ctx.move_to((WIDTH as f64 - extents.width) / 2., HEIGHT as f64 / 4f64);
    ctx.show_text(&concise_observation);
    debug!("{:?}", current_metar);

    let mut weather_emojis = match current_metar.clouds {
        Data::Known(Clouds::SkyClear)
        | Data::Known(Clouds::NoCloudDetected)
        | Data::Known(Clouds::NoSignificantCloud) => {
            "\u{263c}".to_owned() // sunny
        }
        Data::Known(Clouds::CloudLayers) => "".to_owned(),
        _ => "\u{003F}".to_owned(),
    };
    for weather in &current_metar.weather {
        weather_emojis += match weather.intensity {
            WeatherIntensity::Heavy => "\u{2795}",
            WeatherIntensity::Light => "\u{2796}",
            WeatherIntensity::Moderate => "~",
            WeatherIntensity::InVicinity => "\u{1F5FA}",
        };
        for condition in &weather.conditions {
            use WeatherCondition::*;
            weather_emojis += match condition {
                Rain | Drizzle | Showers => "\u{1F327}",
                Snow | SnowGrains => "\u{1F328}",
                Blowing | Squall => "\u{1F32C}",
                Smoke => "\u{1F32C}\u{1F6AC}",
                Thunderstorm => "\u{26C8}",
                Freezing => "\u{1F976}",
                IceCrystals | IcePellets | Hail | SnowPelletsOrSmallHail => "\u{1F9CA}",
                Fog | Haze => "\u{1F32B}",
                Mist | Spray => "\u{1F32B}\u{1F4A7}",
                VolcanicAsh => "\u{E08D}",
                Sand | Dust | WidespreadDust => "\u{1F3DC}",
                Duststorm | Sandstorm | FunnelCloud => "\u{1F32A}",
                Partial | Shallow => "¼",
                LowDrifting => "\u{2601}",
                UnknownPrecipitation | Patches => "\u{003F}",
            }
        }
    }
    set_font(ctx, EMOJI_FONT);
    ctx.set_font_size(DPI * 1.5);
    let extents = ctx.text_extents(&weather_emojis);
    ctx.move_to((WIDTH as f64 - extents.width) / 2., HEIGHT as f64 * 0.6);
    ctx.show_text(&weather_emojis);
}

fn draw_art(ctx: &Context, date_time: &DateTime<Local>) {
    let (_, hour12) = date_time.hour12();
    let surface = crate::art::get_surface_for_hour12(hour12);
    ctx.set_source_surface(&surface, WIDTH as f64 - surface.get_width() as f64, HEIGHT as f64 / 2.0 - surface.get_height() as f64 / 2.0);
    ctx.paint();
}
