use cairo::Context;
use chrono::prelude::*;

use crate::render::set_font;
use crate::{DPI, HEIGHT, WEATHER_STATION, WIDTH, FONT};

pub fn draw_clock(ctx: &Context, date_time: &DateTime<Local>) {
    ctx.set_source_rgb(1.0, 1.0, 1.0);
    draw_date(ctx, date_time.date());
    draw_time(ctx, &date_time);
    let current_observation = weathergov::get_current_observation(WEATHER_STATION).unwrap();
    draw_current_weather(ctx, current_observation);
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

fn draw_current_weather(ctx: &Context, current_observation: weathergov::parse::CurrentObservation) {
    if let None = current_observation.weather {
        return;
    }
    let weather = current_observation.weather.unwrap();
    let weather_emoji = match weather.as_str() {
        "Thunderstorm" => "⛈️",
        "Thunderstorm in Vicinity" => "🌦️",
        "Overcast" => "☁️",
        "Mostly Cloudy" => "🌥️",
        "Partly Cloudy" => "⛅",
        "A Few Clouds" => "🌤️",
        "Fair" => "☀️",
        other => other,
    }
    .to_owned();

    let concise_observation = format!(
        "{}°F {}MPH",
        current_observation.temp_f.unwrap(),
        current_observation.wind_mph.unwrap(),
    );

    set_font(ctx, FONT);
    ctx.set_font_size(DPI * 0.75);
    let extents = ctx.text_extents(&concise_observation);
    ctx.move_to((WIDTH as f64 - extents.width) / 2., HEIGHT as f64 / 4.);
    ctx.show_text(&concise_observation);

    set_font(ctx, "Noto Color Emoji");
    ctx.set_font_size(DPI * 1.5);
    let extents = ctx.text_extents(&weather_emoji);
    ctx.move_to((WIDTH as f64 - extents.width) / 2., HEIGHT as f64 * 0.6);
    ctx.show_text(&weather_emoji);
}
