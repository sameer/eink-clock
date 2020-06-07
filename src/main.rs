use std::path::PathBuf;

use resvg::prelude::*;
use chrono::prelude::*;

/// E Ink Pearl 1200x824 150 DPI 4-bit 16-level grayscale
const WIDTH: u32 = 1200;
const HEIGHT: u32 = 824;
const DPI: f64 = 150.0;

const LANG: &str = "en-US";
const FONT: &str = "Inter";

fn main() {
    let svg_text = get_svg_text();
    let mut image = render(&svg_text).expect("failed to render SVG");
    let path = PathBuf::from("out.png");
    image.save_png(&path);
}

fn render(svg_text: &str)  -> Result<Box<dyn OutputImage>, usvg::Error> {
    let mut options = resvg::Options::default();
    options.fit_to = FitTo::Width(WIDTH);
    options.usvg.dpi = DPI;
    options.usvg.font_family = FONT.to_owned();
    options.usvg.font_size = DPI;
    options.usvg.languages = vec![LANG.to_owned()];
    options.usvg.shape_rendering = usvg::ShapeRendering::CrispEdges;
    options.usvg.text_rendering = usvg::TextRendering::OptimizeLegibility;
    options.usvg.image_rendering = usvg::ImageRendering::OptimizeQuality;

    let tree = usvg::Tree::from_str(svg_text, &options.usvg)?;
    let backend = Box::new(resvg::backend_raqote::Backend);
    Ok(backend.render_to_image(&tree, &options).expect("couldn't allocate image in raqote backend"))
}

fn get_svg_text() -> String {
    format!(r#"
    <svg viewBox="0 0 {width} {height}" width="{width}" height="{height}" xmlns="http://www.w3.org/2000/svg">
        <text font-size="0.75in" x="50%" y="0.75in" text-anchor="middle">{date}</text>
        <text font-size="1.5in" x="50%" y="95%" text-anchor="middle">{time}</text>
    </svg>
    "#, width=WIDTH, height=HEIGHT, date=get_date_as_string(), time=get_time_as_string())
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
