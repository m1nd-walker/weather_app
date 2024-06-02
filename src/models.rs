use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{self};
use std::fmt;


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Coord {
    #[serde(rename = "Lon")]
    pub lon: Option<f32>,
    #[serde(rename = "Lat")]
    pub lat: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename(deserialize = "Weather", serialize = "Weather"))]
pub struct WeatherData {
    pub id: u32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Main {
    pub temp: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub pressure: f32,
    pub humidity: f32,
    pub sea_level: Option<f32>,
    pub grnd_level: Option<f32>,
    pub temp_kf: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Wind {
    pub speed: f32,
    pub deg: f32,
    pub gust: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Rain {
    #[serde(rename = "1h")]
    pub _1h: Option<f32>,
    #[serde(rename = "3h")]
    pub _3h: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Snow {
    #[serde(rename = "1h")]
    pub _1h: Option<f32>,
    #[serde(rename = "3h")]
    pub _3h: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Clouds {
    pub all: Option<u32>,
    pub today: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Sys {
    #[serde(rename = "type")]
    pub message_type: Option<u32>,
    pub id: Option<u32>,
    pub country: String,
    pub sunrise: Option<u32>,
    pub sunset: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Weather {
    pub coord: Coord,
    pub weather: Option<Vec<WeatherData>>,
    pub base: Option<String>,
    pub main: Main,
    pub visibility: Option<u32>,
    pub wind: Wind,
    pub rain: Option<Rain>,
    pub snow: Option<Snow>,
    pub clouds: Clouds,
    pub dt: u32,
    pub sys: Option<Sys>,
    pub timezone:Option<i32>,
    pub id: i32,
    pub name: String,
    pub cod: Option<u32>,
}



