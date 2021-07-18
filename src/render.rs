use crate::{HEIGHT, WIDTH};

use cairo::{Context, Error, FontFace, FontOptions, FontSlant, FontWeight, Format, ImageSurface};

pub fn create_surface() -> Result<ImageSurface, Error> {
    ImageSurface::create(Format::Rgb24, HEIGHT as i32, WIDTH as i32)
}

pub fn create_context(surf: &ImageSurface) -> Context {
    let ctx = Context::new(surf).unwrap();
    ctx.translate(HEIGHT as f64 / 2., WIDTH as f64 / 2.);
    ctx.rotate(-90.0 * std::f64::consts::PI / 180.0);
    ctx.translate(WIDTH as f64 / -2., HEIGHT as f64 / -2.);
    ctx
}

pub fn set_font(ctx: &Context, font_name: &str) {
    let font = FontFace::toy_create(font_name, FontSlant::Normal, FontWeight::Normal).unwrap();
    let font_opts = FontOptions::new().unwrap();
    ctx.set_font_face(&font);
    ctx.set_font_options(&font_opts);
}

pub fn write_surface_to_png(surf: &ImageSurface) -> Vec<u8> {
    let mut png_data = Vec::with_capacity(WIDTH * HEIGHT * 3);
    surf.write_to_png(&mut png_data).unwrap();
    let mut png_data_slice = png_data.as_slice();
    let mut grayscale_buf = vec![0; WIDTH * HEIGHT];

    {
        let decoder = png::Decoder::new(&mut png_data_slice);
        let (info, mut reader) = decoder.read_info().unwrap();
        let mut buf = vec![0; info.buffer_size()];
        reader.next_frame(&mut buf).unwrap();
        for i in 0..(WIDTH * HEIGHT) {
            let rgb = &buf[3 * i..3 * i + 3];
            grayscale_buf[i] = ((rgb[0] as u16 + rgb[1] as u16 + rgb[2] as u16) / 3) as u8;
        }
    }

    let mut grayscale_png = Vec::with_capacity(WIDTH * HEIGHT);
    {
        let mut encoder = png::Encoder::new(&mut grayscale_png, HEIGHT as u32, WIDTH as u32);
        encoder.set_color(png::ColorType::Grayscale);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&grayscale_buf).unwrap();
    }
    grayscale_png
}
