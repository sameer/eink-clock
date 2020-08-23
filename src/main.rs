#[macro_use]
extern crate clap;

#[macro_use]
extern crate log;

use chrono::prelude::*;
use chrono::{Duration, DurationRound};

mod art;
mod audio;
mod clock;
mod network;
mod render;
mod ssh;
mod usb;
mod weather;

use audio::*;
use clock::*;
use render::*;
use ssh::*;
use weather::*;

use std::env;
use std::io::Write;
use std::net::{IpAddr, Ipv4Addr};

use metar::Metar;
use tokio::time;

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
const KINDLE_VENDOR_ID: u16 = 0x0525;
const KINDLE_PRODUCT_ID: u16 = 0xa4a2;

#[tokio::main]
pub async fn main() {
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
    let debug = matches.is_present("debug");
    if debug {
        info!("In debug mode, printing pngs to stdout");
    }

    let one_minute = Duration::minutes(1);
    let now = Local::now();

    let mut interval = time::interval_at(
        time::Instant::now() + (start_of_next_minute(&now) - now).to_std().unwrap(),
        one_minute.to_std().unwrap(),
    );
    let mut metar_string = None;
    loop {
        if now.minute() % 5 == 0 || metar_string.is_none() {
            let new_metar_string = get_metar().await;
            if let Some(new_metar_string) = new_metar_string {
                let new_metar_is_ok = parse_metar_data(&new_metar_string)
                    .map_err(|err| error!("could not parse metar: {}", err))
                    .is_ok();
                if new_metar_is_ok {
                    metar_string = Some(new_metar_string);
                }
            }
        }
        let metar = metar_string
            .as_ref()
            .and_then(|metar_str| parse_metar_data(metar_str).ok());

        let next_minute = start_of_next_minute(&Local::now());
        let png = generate_image(metar.as_ref(), &next_minute).await;
        // ^ precompute
        interval.tick().await;
        debug!("timer went off");
        if debug {
            std::io::stdout().write_all(&png).unwrap();
            debug!("done writing image to stdout");
        } else {
            update_clock(&next_minute, &png).await;
            debug!("done updating clock")
        }
    }
}

fn start_of_next_minute(now: &DateTime<Local>) -> DateTime<Local> {
    let one_minute = Duration::minutes(1);
    now.duration_trunc(one_minute).unwrap() + one_minute
}

async fn update_clock(now: &DateTime<Local>, png: &Vec<u8>) {
    // Reduce update frequency at night time
    if now.minute() % 5 == 0 && night_time(now) {
        return;
    }

    network::setup_if_down()
        .await
        .expect("failed to set up network via rtnetlink");

    let ssh_tcp_stream = match open_tcp_connection() {
        Ok(ssh_tcp_stream) => ssh_tcp_stream,
        Err(err) => {
            warn!(
                "failed to open TCP connection to Kindle, attempting recovery: {}",
                err
            );
            if let Ok(kindle_opt) = usb::get_kindle() {
                if let Some(kindle) = kindle_opt {
                    if let Err(err) = usb::reset_kindle(&kindle) {
                        error!("Couldn't reset Kindle USB: {:?}", err);
                    }
                } else {
                    warn!("Kindle isn't connected!");
                    return;
                }
            } else {
                warn!("Error using libusb, proceeding blindly");
            }
            if network::try_recover().await.is_ok() {
                if let Ok(ssh_tcp_stream) = open_tcp_connection() {
                    ssh_tcp_stream
                } else {
                    return;
                }
            } else {
                // Kindle is disconnected or USB needs reset
                return;
            }
        }
    };
    let mut ssh_session =
        open_ssh_session(ssh_tcp_stream).expect("ssh handshake failed, is the password correct?");

    eips_show_image(&mut ssh_session, png, now.minute() == 0)
        .expect("failed to send image to Kindle");

    if now.minute() == 0 && !night_time(&now) {
        let (_, hour12) = now.hour12();
        play_audio_for_hour(&mut ssh_session, now.hour(), hour12)
            .expect("failed to play hourly tune");
    }
    ssh_session
        .disconnect(None, "done sending commands", None)
        .unwrap();
}

async fn get_metar() -> Option<String> {
    get_current_metar_data()
        .map_err(|err| error!("failed to get metar from aviationweather.gov: {}", err))
        .ok()
        .map(|metar_bytes| String::from_utf8_lossy(&metar_bytes).to_string())
}

async fn generate_image(current_metar: Option<&Metar<'_>>, now: &DateTime<Local>) -> Vec<u8> {
    debug!("Current metar parsed {:?}", current_metar);
    let surf = create_surface().expect("failed to create cairo surface");
    let ctx = create_context(&surf);
    draw_clock(&ctx, now, current_metar);
    let png = write_surface_to_png(&surf);
    png
}

fn night_time(now: &DateTime<Local>) -> bool {
    now.hour() < 7 || now.hour() > 23
}
