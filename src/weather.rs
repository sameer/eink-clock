use curl::{
    easy::{Easy, List},
    Error,
};
use metar::{Metar, MetarError};

use std::io::Write;

use crate::WEATHER_STATION;

pub fn get_current_metar_data() -> Result<Vec<u8>, Error> {
    let url = format!(
        "https://www.aviationweather.gov/adds/dataserver_current/httpparam?dataSource=metars&requestType=retrieve&format=xml&hoursBeforeNow=3&mostRecent=true&stationString={}",
        WEATHER_STATION
    );
    let mut easy = Easy::new();
    easy.url(&url)?;
    let mut list = List::new();
    list.append(&format!(
        "User-Agent: curl/{}",
        curl::Version::get().version()
    ))?;
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
    const SKIP_START: &str = "<raw_text>";
    const SKIP_END: &str = "</raw_text>";
    let metar_start = data.find(SKIP_START);
    let metar_end = data.find(SKIP_END).unwrap_or(data.len());
    if let Some(index) = metar_start {
        Metar::parse(&data[index + SKIP_START.len()..metar_end])
    } else {
        Metar::parse(data)
    }
}

#[derive(Default)]
pub struct Remark {
    observation_type: Option<ObservationType>,
}

pub enum ObservationType {
    AutomatedWithoutPrecipitationDiscriminator,
    AutomatedWithPrecipitationDiscriminator,
}

pub struct DirectionalVector {
    distance: Option<Distance>,
    direction: Direction,
}

pub enum Distance {
    Distant,
}

pub enum Direction {
    N,
    W,
    S,
    E,
    NW,
    NE,
    SW,
    SE,
}

impl Remark {
    pub fn parse_from_str(raw: &str) -> Self {
        let mut remark = Remark::default();
        for term in raw.split_whitespace() {
            if term == "AO2" {
                remark.observation_type =
                    Some(ObservationType::AutomatedWithPrecipitationDiscriminator);
            } else if term == "AO1" {
                remark.observation_type =
                    Some(ObservationType::AutomatedWithoutPrecipitationDiscriminator);
            }
        }
        remark
    }
}
