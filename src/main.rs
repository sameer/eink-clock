#[macro_use]
extern crate clap;

#[macro_use]
extern crate log;

use chrono::prelude::*;

mod art;
mod audio;
mod clock;
mod network;
mod render;
mod ssh;
mod weather;

use audio::*;
use clock::*;
use render::*;
use ssh::*;
use weather::*;

use std::env;
use std::io::Write;
use std::net::{IpAddr, Ipv4Addr};

/// E Ink Pearl 1200x824 150 DPI 4-bit 16-level grayscale
const WIDTH: usize = 1200;
const HEIGHT: usize = 824;
const DPI: f64 = 150.0;
const FONT: &str = "Inter";
const EMOJI_FONT: &str = "OpenMoji";

const WEATHER_STATION: &str = "KTPA";
const TEMPERATURE_UNITS: uom::si::thermodynamic_temperature::degree_fahrenheit =
    uom::si::thermodynamic_temperature::degree_fahrenheit;
const WIND_SPEED_UNITS: uom::si::velocity::mile_per_hour = uom::si::velocity::mile_per_hour;

const PI_IP_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::new(192, 168, 2, 1));
const KINDLE_IP_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::new(192, 168, 2, 2));
const KINDLE_SSH_PORT: u16 = 22;
const KINDLE_USERNAME: &str = "root";
const KINDLE_PASSWORD: &str = "root";
const KINDLE_CONNECT_TIMEOUT: u64 = 1000;
const KINDLE_INTERFACE: &str = "usb0";

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "eink_clock=info")
    }
    env_logger::init();
    let matches = clap_app!(svg2gcode =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg debug: --debug "To debug locally, eink-clock will simply output the PNG for the current time")
    )
    .get_matches();

    let now = Local::now();

    let current_metar_bytes =
        get_current_metar_data().expect("failed to get metar from weather.gov");
    let current_metar_data = String::from_utf8(current_metar_bytes).expect("metar was not UTF-8");
    let current_metar = parse_metar_data(&current_metar_data).expect("failed to parse metar data");

    let surf = create_surface().expect("failed to create cairo surface");
    let ctx = create_context(&surf);
    draw_clock(&ctx, &now, current_metar);
    let png = write_surface_to_png(&surf);

    if matches.is_present("debug") {
        info!("In debug mode, printing png to stdout");
        std::io::stdout().write_all(&png).unwrap();
        return;
    }

    network::setup_if_down().expect("failed to set up network via rtnetlink");

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
