use std::fs::File;
use std::io::Write;

use cairo::{Context, FontFace, FontOptions, FontSlant, FontWeight, Format, ImageSurface};
use chrono::prelude::*;

const WIDTH: f64 = 1200.0;
const HEIGHT: f64 = 824.0;
const FONT: &str = "Inter";

fn main() {
    let surf = ImageSurface::create(Format::A8, HEIGHT as i32, WIDTH as i32)
        .expect("couldn't create Cairo surface");
    let ctx = Context::new(&surf);
    let font = FontFace::toy_create(FONT, FontSlant::Normal, FontWeight::Normal);
    let font_opts = FontOptions::default();
    ctx.set_font_face(&font);
    ctx.set_font_options(&font_opts);
    ctx.set_font_size(96.);
    ctx.set_source_rgb(1., 1., 1.);
    ctx.translate(HEIGHT / 2., WIDTH / 2.);
    ctx.rotate(90.0 * std::f64::consts::PI / 180.0);

    let date = get_date_as_string();
    let extent = ctx.text_extents(&date);
    println!("{:?}", extent);
    ctx.move_to(-WIDTH/2., -HEIGHT/2. + extent.height);
    ctx.show_text(&date);
    println!("{:?}", ctx.get_current_point());
    ctx.rel_move_to(-extent.x_advance, 0.0);
    println!("{:?}", ctx.get_current_point());

    let time = get_time_as_string();
    let extent = ctx.text_extents(&time);
    println!("{:?}", extent);
    ctx.rel_move_to(0.0, 96.0);
    println!("{:?}", ctx.get_current_point());
    ctx.show_text(&time);

    let mut out_png = File::create("out.png").expect("couldn't create output file");
    surf.write_to_png(&mut out_png).unwrap();
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
