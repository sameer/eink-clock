#[macro_use]
extern crate clap;

#[macro_use]
extern crate log;

use chrono::prelude::*;

mod clock;
mod network;
mod ssh;
mod svg;
mod audio;

use audio::play_audio_for_hour;
use clock::get_svg_text;
use svg::{render, image_into_png};
use ssh::{eips_show_image, open_tcp_connection, open_ssh_session};

use std::net::Ipv4Addr;

/// E Ink Pearl 1200x824 150 DPI 4-bit 16-level grayscale
const WIDTH: u32 = 1200;
const HEIGHT: u32 = 824;
const DPI: f64 = 150.0;

const LANG: &str = "en-US";
const FONT: &str = "Inter";
const WEATHER_STATION: &str = "KTPA";

const KINDLE_IP_ADDRESS: Ipv4Addr = Ipv4Addr::new(192, 168, 2, 2);
const KINDLE_SSH_PORT: u16 = 22;
const KINDLE_USERNAME: &str = "root";
const KINDLE_PASSWORD: &str = "root";
const KINDLE_CONNECT_TIMEOUT: u64 = 1000;

fn main() {
    let matches = clap_app!(svg2gcode =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg debug: --debug "To debug locally, eink-clock will simply output the SVG for the current time")
    )
    .get_matches();

    let now = Local::now();
    let svg_text = get_svg_text(&now);

    if matches.is_present("debug") {
        info!("In debug mode, printing svg to stdout");
        println!("{}", svg_text);
        return;
    }

    let image = render(&svg_text).expect("failed to render SVG");
    let png = image_into_png(image).expect("failed to convert image to png");

    let ssh_tcp_stream = open_tcp_connection().expect("failed to connect to Kindle");
    let mut ssh_session = open_ssh_session(ssh_tcp_stream).expect("ssh authorized failed, is the password correct?");

    eips_show_image(&mut ssh_session, &png).expect("failed to send image to Kindle");

    if now.minute() == 0 && !night_time(&now) {
        let (_, hour12) = now.hour12();
        play_audio_for_hour(&mut ssh_session, now.hour(), hour12).expect("failed to play hourly tune");
    }
    ssh_session.disconnect(None, "done sending commands", None).unwrap();
}

fn night_time(now: &DateTime<Local>) -> bool {
    now.hour() < 7 || now.hour() > 23
}
