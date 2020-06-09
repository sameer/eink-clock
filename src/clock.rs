use chrono::prelude::*;

use crate::{HEIGHT, WEATHER_STATION, WIDTH};

pub fn get_svg_text() -> String {
    let current_observation = weathergov::get_current_observation(WEATHER_STATION).unwrap();
    format!(
        r#"
    <svg viewBox="0 0 {width} {height}" width="{width}" height="{height}" xmlns="http://www.w3.org/2000/svg">
        <text font-size="0.75in" x="50%" y="0.75in" text-anchor="middle">{date}</text>
        <text font-size="0.75in" x="50%" y="2in" text-anchor="middle">{weather}</text>
        <text font-size="1.5in" x="50%" y="95%" text-anchor="middle">{time}</text>
        <g transform="translate(100 100)">{weather_svg}</g>
    </svg>
    "#,
        width = WIDTH,
        height = HEIGHT,
        date = get_date_as_string(),
        time = get_time_as_string(),
        weather = get_current_weather_as_string(&current_observation),
        weather_svg = get_current_weather_svg(&current_observation)
    )
}

fn get_date_as_string() -> String {
    let now: DateTime<Local> = Local::now();
    format!("{}", now.format("%A %B %_d, %Y"))
}

fn get_time_as_string() -> String {
    let now: DateTime<Local> = Local::now();
    let (_, hour) = now.hour12();
    format!("{}{}", hour, now.format(":%M %p"))
}

fn get_current_weather_as_string(
    current_observation: &weathergov::parse::CurrentObservation,
) -> String {
    let weather = current_observation.weather.clone().unwrap();

    format!(
        "{}°F {}MPH",
        current_observation.temp_f.unwrap(),
        current_observation.wind_mph.unwrap()
    )
    // r#"
    // <path fill="none" stroke="\#000000" stroke-linecap="round" stroke-linejoin="round" stroke-miterlimit="10" stroke-width="2" d="M15.9345,30.2552c-0.0372,0.4242-0.3794,0.7675-0.8028,0.8127C9.9704,31.6186,6,36.6998,6,42.8852 c0,6.553,4.5445,11.8652,10.1505,11.8652h38.6977C61.0072,54.7504,66,49.1365,66,42.2114c0-6.6379-4.5872-12.0711-10.3916-12.5103 c-0.4421-0.0335-0.8008-0.3444-0.8855-0.7796c-1.2964-6.6564-7.2763-11.6585-14.3462-11.6585 c-4.5964,0-8.6908,2.0817-11.3486,5.3929c-0.2582,0.3217-0.6903,0.4696-1.0705,0.3097c-1.0245-0.4306-2.1065-0.639-3.3175-0.639 C20.0727,22.3266,16.3237,25.8122,15.9345,30.2552z"/>
    // "#.to_owned()
}

fn get_current_weather_svg(current_observation: &weathergov::parse::CurrentObservation) -> String {
    let weather = current_observation.weather.clone().unwrap();
    match weather.as_str() {
        "Overcast" => r##"
            <path fill="none" stroke="\#000000" stroke-linecap="round" stroke-linejoin="round" stroke-miterlimit="10" stroke-width="2" d="M15.9345,30.2552c-0.0372,0.4242-0.3794,0.7675-0.8028,0.8127C9.9704,31.6186,6,36.6998,6,42.8852 c0,6.553,4.5445,11.8652,10.1505,11.8652h38.6977C61.0072,54.7504,66,49.1365,66,42.2114c0-6.6379-4.5872-12.0711-10.3916-12.5103 c-0.4421-0.0335-0.8008-0.3444-0.8855-0.7796c-1.2964-6.6564-7.2763-11.6585-14.3462-11.6585 c-4.5964,0-8.6908,2.0817-11.3486,5.3929c-0.2582,0.3217-0.6903,0.4696-1.0705,0.3097c-1.0245-0.4306-2.1065-0.639-3.3175-0.639 C20.0727,22.3266,16.3237,25.8122,15.9345,30.2552z"/>
        "##,
        "A few clouds" => r##"
            <g id="line">
                <path fill="none" stroke="#000000" stroke-linecap="round" stroke-linejoin="round" stroke-miterlimit="10" stroke-width="2" d="M24.4645,53.5202c-6.1162-2.9451-10.3373-9.202-10.3373-16.4446c0-10.0744,8.1669-18.2414,18.2414-18.2414 c9.0895,0,16.6262,6.648,18.013,15.347"/>
                <polyline fill="none" stroke="#000000" stroke-linecap="round" stroke-linejoin="round" stroke-miterlimit="10" stroke-width="2" points="24.5472,56.9821 18.5628,61.0249 19.3576,50.132 8.4649,50.9267 14.5909,41.8884 4.7686,37.1215 14.591,32.3545 8.4653,23.3157 19.3582,24.1105 18.5635,13.2178 27.6018,19.3438 32.3686,9.5215 37.1357,19.3439 46.1745,13.2182 45.3797,24.111 56.2724,23.3164 50.1464,32.3547"/>
                <path fill="none" stroke="#000000" stroke-linecap="round" stroke-linejoin="round" stroke-miterlimit="10" stroke-width="2" d="M34.791,46.6066c-0.0241,0.2749-0.2468,0.4973-0.5226,0.5268c-3.3432,0.3581-5.9145,3.65-5.9145,7.6568 c0,4.2461,2.9447,7.6882,6.5771,7.6882h25.0744c3.9908,0,7.2259-3.6376,7.2259-8.1248c0-4.3002-2.9712-7.8202-6.7312-8.106 c-0.2877-0.0219-0.521-0.2232-0.5758-0.505c-0.84-4.3132-4.7148-7.5546-9.2958-7.5546c-2.9773,0-5.6294,1.3479-7.3517,3.4921 c-0.1683,0.2095-0.4507,0.3055-0.6992,0.2013c-0.6627-0.2779-1.3626-0.4124-2.1458-0.4124 C37.4724,41.4692,35.0432,43.7277,34.791,46.6066z"/>
            </g>
        "##,
        "Mostly Cloudy" => r##"
            <g id="line">
                <polyline fill="none" stroke="#000000" stroke-linecap="round" stroke-linejoin="round" stroke-miterlimit="10" stroke-width="1.9" points="11.2649,40.762 8.0651,42.558 9.731,34.5816 1.6667,33.4196 7.7025,27.9466 1.8613,22.2652 9.9649,21.3884 8.5716,13.3574 15.644,17.4142 19.239,10.0962 22.5746,17.5334 29.7833,13.7289 28.1174,21.7053 36.1818,22.8673 33.2577,25.2498"/>
                <path fill="none" stroke="#000000" stroke-linecap="round" stroke-linejoin="round" stroke-miterlimit="10" stroke-width="1.9" d="M14.2551,37.865c-3.6168-1.7402-6.1131-5.4394-6.1131-9.7216c0-5.9549,4.8274-10.7822,10.7822-10.7822 c5.59,0,10.1864,4.2539,10.7287,9.7012"/>
                <path fill="none" stroke="#000000" stroke-linecap="round" stroke-linejoin="round" stroke-miterlimit="10" stroke-width="2" d="M20.9631,37.7488c-0.0366,0.4183-0.3756,0.7568-0.7953,0.8018c-5.0879,0.545-9.0012,5.5548-9.0012,11.6528 c0,6.462,4.4814,11.7005,10.0096,11.7005l38.1602,0c6.0734,0,10.997-5.536,10.997-12.3649c0-6.5444-4.5218-11.9014-10.2441-12.3364 c-0.4378-0.0333-0.7928-0.3397-0.8763-0.7685c-1.2783-6.5642-7.1753-11.4971-14.1471-11.4971 c-4.531,0-8.5673,2.0513-11.1884,5.3146c-0.2561,0.3189-0.686,0.465-1.0641,0.3064c-1.0086-0.423-2.0738-0.6277-3.2657-0.6277 C25.0438,29.9303,21.3469,33.3675,20.9631,37.7488z"/>
            </g>
        "##,
        other => other,
    }.to_owned()
}
