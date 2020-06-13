use crate::{HEIGHT, WIDTH};

use cairo::{Context, FontFace, FontOptions, FontSlant, FontWeight, Format, ImageSurface, Status};

pub fn create_surface() -> Result<ImageSurface, Status> {
    ImageSurface::create(Format::Rgb24, HEIGHT as i32, WIDTH as i32)
}

pub fn create_context(surf: &ImageSurface) -> Context {
    let ctx = Context::new(surf);
    ctx.translate(HEIGHT as f64 / 2., WIDTH as f64 / 2.);
    ctx.rotate(-90.0 * std::f64::consts::PI / 180.0);
    ctx.translate(WIDTH as f64 / -2., HEIGHT as f64 / -2.);
    ctx
}

pub fn set_font(ctx: &Context, font_name: &str) {
    let font = FontFace::toy_create(font_name, FontSlant::Normal, FontWeight::Normal);
    let font_opts = FontOptions::default();
    ctx.set_font_face(&font);
    ctx.set_font_options(&font_opts);
}

pub fn write_surface_to_png(surf: &ImageSurface) -> Vec<u8> {
    let mut png = Vec::with_capacity(WIDTH * HEIGHT * 3);
    surf.write_to_png(&mut png).unwrap();
    png
}
