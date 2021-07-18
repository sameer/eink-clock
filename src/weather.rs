use metar::{Metar, MetarError};

use crate::WEATHER_STATION;

pub async fn get_current_metar_data() -> reqwest::Result<String> {
    let url = format!(
        "https://www.aviationweather.gov/adds/dataserver_current/httpparam?datasource=metars&requesttype=retrieve&format=xml&hoursBeforeNow=1.25&mostRecentForEachStation=constraint&stationString={}",
        WEATHER_STATION
    );

    reqwest::get(url).await?.text().await
}

pub fn parse_metar_data(data: &str) -> Result<Metar<'_>, MetarError<'_>> {
    const SKIP_START: &str = "<raw_text>";
    const SKIP_END: &str = "</raw_text>";
    let metar_start = data.find(SKIP_START);
    let metar_end = data.find(SKIP_END).unwrap_or_else(|| data.len());
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
