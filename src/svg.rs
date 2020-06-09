use resvg::prelude::*;

use crate::{DPI, FONT, HEIGHT, LANG, WIDTH};

pub fn render(svg_text: &str) -> Result<Box<dyn OutputImage>, usvg::Error> {
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
    let backend = resvg::default_backend();
    Ok(backend
        .render_to_image(&tree, &options)
        .expect("couldn't allocate image in raqote backend"))
}

pub fn image_into_png(mut image: Box<dyn OutputImage>) -> Result<Vec<u8>, std::io::Error> {
    let rgba_vec = image.make_rgba_vec();
    let mut rotated_grayscale_vec = vec![0; rgba_vec.len() / 4];
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let first = x + y * WIDTH;
            let second = (WIDTH - 1 - x) * HEIGHT + y;
            let mut acc = 0u16;
            for z in 0..3 {
                // ignore alpha channel
                acc += rgba_vec[first as usize * 4 + z] as u16;
            }
            rotated_grayscale_vec[second as usize] = (acc / 3) as u8;
        }
    }
    let mut png = Vec::with_capacity(rotated_grayscale_vec.len());
    {
        let mut encoder = png::Encoder::new(&mut png, HEIGHT, WIDTH);
        encoder.set_color(png::ColorType::Grayscale);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(&rotated_grayscale_vec)?;
    }
    Ok(png)
}
