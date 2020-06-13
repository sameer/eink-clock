use cairo::ImageSurface;

macro_rules! hours {
    ($hour12: ident, $($hour: expr,)*) => {
        match $hour12 {
            $($hour => ImageSurface::create_from_png(&mut include_bytes!(concat!("../art/", stringify!($hour), ".png")).to_vec().as_slice()).unwrap(),)*
            _ => ImageSurface::create_from_png(&mut include_bytes!("../art/1x1.png").to_vec().as_slice()).unwrap()
        }
    };
}

pub fn get_surface_for_hour12(hour12: u32) -> ImageSurface {
    hours!(hour12, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,)
}
