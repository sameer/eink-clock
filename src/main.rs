#[macro_use]
extern crate clap;

#[macro_use]
extern crate log;

use chrono::prelude::*;

mod audio;
mod clock;
mod network;
mod render;
mod ssh;

use audio::*;
use clock::*;
use render::*;
use ssh::*;

use std::env;
use std::io::Write;
use std::net::Ipv4Addr;

/// E Ink Pearl 1200x824 150 DPI 4-bit 16-level grayscale
const WIDTH: usize = 1200;
const HEIGHT: usize = 824;
const DPI: f64 = 150.0;

const FONT: &str = "DejaVu Sans";
const WEATHER_STATION: &str = "KTPA";

const KINDLE_IP_ADDRESS: Ipv4Addr = Ipv4Addr::new(192, 168, 2, 2);
const KINDLE_SSH_PORT: u16 = 22;
const KINDLE_USERNAME: &str = "root";
const KINDLE_PASSWORD: &str = "root";
const KINDLE_CONNECT_TIMEOUT: u64 = 1000;

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "eink-clock=info")
    }
    env_logger::init();
    let matches = clap_app!(svg2gcode =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg debug: --debug "To debug locally, eink-clock will simply output the SVG for the current time")
    )
    .get_matches();

    let now = Local::now();

    let surf = create_surface().expect("failed to create cairo surface");
    let ctx = create_context(&surf);
    draw_clock(&ctx, &now);
    let png = write_surface_to_png(&surf);

    if matches.is_present("debug") {
        info!("In debug mode, printing svg to stdout");
        std::io::stdout().write_all(&png).unwrap();
        return;
    }

    let ssh_tcp_stream = open_tcp_connection().expect("failed to connect to Kindle");
    let mut ssh_session =
        open_ssh_session(ssh_tcp_stream).expect("ssh authorized failed, is the password correct?");

    eips_show_image(&mut ssh_session, &png).expect("failed to send image to Kindle");

    if now.minute() == 0 && !night_time(&now) {
        let (_, hour12) = now.hour12();
        play_audio_for_hour(&mut ssh_session, now.hour(), hour12)
            .expect("failed to play hourly tune");
    }
    ssh_session
        .disconnect(None, "done sending commands", None)
        .unwrap();
}

fn night_time(now: &DateTime<Local>) -> bool {
    now.hour() < 7 || now.hour() > 23
}
