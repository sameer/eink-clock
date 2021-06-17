use cairo::{Context, Gradient, LinearGradient, TextExtents};
use chrono::prelude::*;
use metar::{
    CloudLayer, CloudType, Clouds, Data, Metar, SpeedUnit, WeatherCondition, WeatherIntensity,
};

use crate::render::set_font;
use crate::{DPI, EMOJI_FONT, FONT, HEIGHT, TEMPERATURE_UNITS, WIDTH, WIND_SPEED_UNITS};

pub fn draw_clock(ctx: &Context, date_time: &DateTime<Local>, current_metar: Option<&Metar<'_>>) {
    ctx.set_source_rgb(1.0, 1.0, 1.0);
    ctx.rectangle(0.0, 0.0, WIDTH as f64, HEIGHT as f64);
    ctx.fill();
    ctx.set_source_rgb(0.0, 0.0, 0.0);

    // ctx.set_source_rgb(0.0, 0.0, 0.0);
    let date_extents = draw_date(ctx, date_time.date());
    draw_time(ctx, date_extents, &date_time);
    if let Some(current_metar) = current_metar {
        draw_current_weather(ctx, current_metar);
    }
    draw_art(ctx, &date_time);
}

fn draw_date(ctx: &Context, date: Date<Local>) -> TextExtents {
    let date = format!("{}", date.format("%A %B %_d, %Y"));

    set_font(ctx, FONT);
    ctx.set_font_size(DPI * 0.50);
    let extents = ctx.text_extents(&date);
    ctx.move_to((WIDTH as f64 - extents.width) / 2.0, extents.height);
    ctx.show_text(&date);
    ctx.stroke();
    extents
}

fn draw_time(ctx: &Context, date_extents: TextExtents, date_time: &DateTime<Local>) {
    let (_, hour12) = date_time.hour12();
    let time = format!("{}{}", hour12, date_time.format(":%M %p"));

    set_font(ctx, FONT);
    ctx.set_font_size(DPI * 1.5);
    let time_extents = ctx.text_extents(&time);
    ctx.move_to(
        (WIDTH as f64 - time_extents.width) / 2.0,
        date_extents.height * 1.5 + time_extents.height,
    );
    ctx.show_text(&time);
}

fn draw_current_weather(ctx: &Context, current_metar: &Metar<'_>) {
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
    ctx.set_font_size(DPI * 0.45);
    let extents = ctx.text_extents(&concise_observation);
    ctx.move_to(
        WIDTH as f64 * 0.25 - extents.width * 0.5,
        HEIGHT as f64 - (extents.height + extents.y_bearing) * 0.5,
    );
    ctx.show_text(&concise_observation);

    let mut weather_emojis: String = match &current_metar.clouds {
        Data::Known(Clouds::SkyClear)
        | Data::Known(Clouds::NoCloudDetected)
        | Data::Known(Clouds::NoSignificantCloud) => {
            "\u{263c}".to_owned() // sunny
        }
        Data::Known(Clouds::CloudLayers) => {
            let gradient_width = WIDTH as f64 / 16.;
            let gradient_height = HEIGHT as f64 / 2.;
            let gradient_x = WIDTH as f64 / 2.;
            let gradient_y = HEIGHT as f64 / 2.;
            ctx.set_source_rgb(0.0, 0.0, 0.0);
            ctx.rectangle(gradient_x, gradient_y, gradient_width, gradient_height);
            ctx.fill();

            let layers: Vec<(u8, u32)> = current_metar
                .cloud_layers
                .iter()
                .filter_map(|layer| {
                    use metar::CloudLayer::*;
                    match layer {
                        Few(_, Some(height)) => Some((1, *height)),
                        Scattered(_, Some(height)) => Some((3, *height)),
                        Broken(_, Some(height)) => Some((5, *height)),
                        Overcast(_, Some(height)) => Some((7, *height)),
                        _ => None,
                    }
                })
                // Normally cannot see clouds above 20,000 feet
                .filter(|layer| layer.1 < 200)
                .collect();
            let max_height =
                layers.iter().max_by_key(|l| l.1).map(|l| l.1).unwrap_or(0) as f64 * 1.05;
            ctx.set_source_rgb(1.0, 1.0, 1.0);
            set_font(ctx, FONT);
            ctx.set_font_size(DPI * 0.1);
            for layer in layers {
                let level = layer.0 as f64 / 8.;
                let text = format!("{} ft", layer.1 * 100);
                let extents = ctx.text_extents(&text);
                ctx.move_to(
                    gradient_x + gradient_width / 2. - extents.width / 2. - extents.x_bearing,
                    gradient_y + (1. - layer.1 as f64 / max_height) * gradient_height,
                );
                ctx.show_text(&text);
                ctx.set_dash(
                    &[
                        gradient_width * 0.25 * level,
                        gradient_width * 0.25 * (1. - level),
                    ],
                    0.,
                );
                ctx.set_line_width(5.);
                ctx.move_to(
                    gradient_x,
                    gradient_y + (1. - layer.1 as f64 / max_height) * gradient_height + 2.0,
                );
                ctx.line_to(
                    gradient_x + gradient_width,
                    gradient_y + (1. - layer.1 as f64 / max_height) * gradient_height + 5.,
                );
                ctx.stroke();
            }

            "".to_owned()
        }
        _ => "\u{2753}".to_owned(),
    };
    for weather in &current_metar.weather {
        weather_emojis += match weather.intensity {
            WeatherIntensity::Heavy => "\u{2795}",
            WeatherIntensity::Light => "\u{2796}",
            WeatherIntensity::Moderate => "\u{3030}",
            WeatherIntensity::InVicinity | WeatherIntensity::Recent => "\u{1F5FA}",
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
                UnknownPrecipitation | Patches => "\u{2753}",
            }
        }
    }
    set_font(ctx, EMOJI_FONT);
    ctx.set_source_rgb(0.0, 0.0, 0.0);
    let mut initial_scale = DPI * 1.5;

    let mut extents;
    while {
        // Silly shrink to fit, I'm not sure how to do it the right way
        ctx.set_font_size(initial_scale);
        extents = ctx.text_extents(&weather_emojis);
        extents.width > WIDTH as f64 / 2.0
    } {
        initial_scale *= 0.9;
    }
    ctx.move_to(
        WIDTH as f64 * 0.25 - (extents.x_bearing + extents.width) * 0.5,
        HEIGHT as f64 * 0.5 + extents.height,
    );
    ctx.show_text(&weather_emojis);
}

fn draw_art(ctx: &Context, date_time: &DateTime<Local>) {
    let (_, hour12) = date_time.hour12();
    let surface = crate::art::get_surface_for_hour12(hour12);
    ctx.set_source_surface(
        &surface,
        WIDTH as f64 * 0.75 - surface.get_width() as f64 * 0.5,
        HEIGHT as f64 * 0.75 - surface.get_height() as f64 * 0.5,
    );
    ctx.paint();
    ctx.set_source_rgb(0.0, 0.0, 0.0);
    set_font(ctx, FONT);
    ctx.set_font_size(DPI * 0.25);
    let art_name = crate::art::get_name_for_hour12(hour12);
    let extents = ctx.text_extents(&art_name);
    ctx.move_to(
        WIDTH as f64 * 0.75 - extents.width * 0.5,
        HEIGHT as f64 - (extents.height + extents.y_bearing),
    );
    ctx.show_text(&art_name);
}
