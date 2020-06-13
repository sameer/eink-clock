use curl::{easy::{Easy, List}, Error};
use metar::{Metar, MetarError};

use std::io::Write;

use crate::WEATHER_STATION;

pub fn get_current_metar_data() -> Result<Vec<u8>, Error> {
    let url = format!("https://w1.weather.gov/data/METAR/{}.1.txt", WEATHER_STATION);
    eprintln!("{}", url);
    let mut easy = Easy::new();
    easy.url(&url)?;
    let mut list = List::new();
    list.append(&format!("User-Agent: curl/{}", curl::Version::get().version()))?;
    easy.http_headers(list)?;
    let mut dst = vec![];
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            dst.extend_from_slice(data);
            Ok(data.len())
        })?;
        transfer.perform()?;
    }
    Ok(dst)
}

pub fn parse_metar_data<'a>(data: &'a str) -> Result<Metar<'a>, MetarError<'a>> {
    const SKIP_START: &str = "METAR ";
    let metar_start = data.find(SKIP_START);
    if let Some(index) = metar_start {
        Metar::parse(&data[index + SKIP_START.len()..])
    } else {
        Metar::parse(data)
    }
}