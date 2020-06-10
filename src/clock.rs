use chrono::prelude::*;

use crate::{FONT, HEIGHT, WEATHER_STATION, WIDTH};

pub fn get_svg_text(date_time: &DateTime<Local>) -> String {
    let current_observation = weathergov::get_current_observation(WEATHER_STATION).unwrap();
    format!(
        r#"
    <svg viewBox="0 0 {width} {height}" width="{width}" height="{height}" xmlns="http://www.w3.org/2000/svg">
        <text font-family="{font}" font-size="0.5in" x="50%" y="0.75in" text-anchor="middle">{date}</text>
        <text font-family="{font}" font-size="0.75in" x="50%" y="2in" text-anchor="middle">{weather}</text>
        <text font-family="{font}" font-size="1.5in" x="50%" y="95%" text-anchor="middle">{time}</text>
        <g transform="translate({half_width} {half_height}) scale(0.25 0.25) translate(-{half_width} -{half_height})">{weather_svg}</g>
    </svg>
    "#,
        width = WIDTH,
        height = HEIGHT,
        half_width = WIDTH / 2,
        half_height = HEIGHT / 2,
        font = FONT,
        date = get_date_as_string(date_time),
        time = get_time_as_string(date_time),
        weather = get_current_weather_as_string(&current_observation),
        weather_svg = get_current_weather_svg(current_observation)
    )
}

fn get_date_as_string(date_time: &DateTime<Local>) -> String {
    format!("{}", date_time.format("%A %B %_d, %Y"))
}

fn get_time_as_string(date_time: &DateTime<Local>) -> String {
    let (_, hour) = date_time.hour12();
    format!("{}{}", hour, date_time.format(":%M %p"))
}

fn get_current_weather_as_string(
    current_observation: &weathergov::parse::CurrentObservation,
) -> String {
    format!(
        "{}°F {}MPH",
        current_observation.temp_f.unwrap(),
        current_observation.wind_mph.unwrap()
    )
}

fn get_current_weather_svg(current_observation: weathergov::parse::CurrentObservation) -> String {
    if let None = current_observation.weather {
        return String::new();
    }
    let weather = current_observation.weather.unwrap();
    match weather.as_str() {
        "Overcast" => include_str!("../emoji/2601.svg"),
        "A Few Clouds" => include_str!("../emoji/1F324.svg"),
        "Mostly Cloudy" => include_str!("../emoji/1F325.svg"),
        "Thunderstorm" => include_str!("../emoji/26C8.svg"),
        "Thunderstorm in Vicinity" => include_str!("../emoji/1F326.svg"),
        "Fair" => include_str!("../emoji/2600.svg"),
        other => other,
    }
    .to_owned()
}
