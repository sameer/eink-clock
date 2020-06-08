use chrono::prelude::*;
use resvg::prelude::*;

/// E Ink Pearl 1200x824 150 DPI 4-bit 16-level grayscale
const WIDTH: u32 = 1200;
const HEIGHT: u32 = 824;
const DPI: f64 = 150.0;

const LANG: &str = "en-US";
const FONT: &str = "Noto Sans";
const WEATHER_STATION: &str = "KTPA";

fn main() {
    let svg_text = get_svg_text();
    let image = render(&svg_text).expect("failed to render SVG");
    save(image).expect("failed to save image");
}

fn render(svg_text: &str) -> Result<Box<dyn OutputImage>, usvg::Error> {
    let mut options = resvg::Options::default();
    options.fit_to = FitTo::Width(WIDTH);
    options.usvg.dpi = DPI;
    options.usvg.font_family = FONT.to_owned();
    options.usvg.font_size = DPI;
    options.usvg.languages = vec![LANG.to_owned()];
    options.usvg.shape_rendering = usvg::ShapeRendering::CrispEdges;
    options.usvg.text_rendering = usvg::TextRendering::OptimizeLegibility;
    options.usvg.image_rendering = usvg::ImageRendering::OptimizeQuality;
    options.background = Some(usvg::Color::white());

    let tree = usvg::Tree::from_str(svg_text, &options.usvg)?;
    let backend = Box::new(resvg::backend_raqote::Backend);
    Ok(backend
        .render_to_image(&tree, &options)
        .expect("couldn't allocate image in raqote backend"))
}

fn save(mut image: Box<dyn OutputImage>) -> Result<(), std::io::Error> {
    let rgba_vec = image.make_rgba_vec();
    let mut rotated_rgba_vec = vec![0; rgba_vec.len() / 4];
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let first = x + y * WIDTH;
            let second = (WIDTH - 1 - x) * HEIGHT + y;
            let mut acc = 0u16;
            for z in 0..3 { // ignore alpha channel
                acc += rgba_vec[first as usize * 4 + z] as u16;
            }
            rotated_rgba_vec[second as usize] = (acc / 3) as u8;
        }
    }
    let mut stdout = std::io::stdout();
    let mut encoder = png::Encoder::new(&mut stdout, HEIGHT, WIDTH);
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(&rotated_rgba_vec)?;
    Ok(())
}

fn get_svg_text() -> String {
    format!(
        r#"
    <svg viewBox="0 0 {width} {height}" width="{width}" height="{height}" xmlns="http://www.w3.org/2000/svg">
        <text font-size="0.75in" x="50%" y="0.75in" text-anchor="middle">{date}</text>
        <text font-size="0.75in" x="50%" y="2in" text-anchor="middle">{weather}</text>
        <text font-size="1.5in" x="50%" y="95%" text-anchor="middle">{time}</text>
    </svg>
    "#,
        width = WIDTH,
        height = HEIGHT,
        date = get_date_as_string(),
        time = get_time_as_string(),
        weather = get_current_weather_as_string()
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

fn get_current_weather_as_string() -> String {
    let current_observation = weathergov::get_current_observation(WEATHER_STATION).unwrap();
    let weather = current_observation.weather.unwrap();
    let condition_emoji = match weather.as_str() {
        "Overcast" => "☁️",
        "A few clouds" => "🌤️",
        "Mostly Cloudy" => "🌥️",
        other => other
    };
    format!("{} {}°F 🌬️ {}MPH", condition_emoji, current_observation.temp_f.unwrap(), current_observation.wind_mph.unwrap())
}
