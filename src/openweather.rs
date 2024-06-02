use crate::models::*;
use std::fmt::Debug;
use reqwest::{Client, Result};
use serde::de::DeserializeOwned;

pub struct OpenWeather {
    api_key: String,
    client: Client
}
impl OpenWeather {
    pub fn new (api_key: String) -> OpenWeather { // Creates new instance of OpenWeather client
        OpenWeather {
            api_key,
            client: Client::new()
        }
    }
    pub async fn get_by_city(
        &self,
        city: &str
    ) -> Result<Weather> {
        self.get(&format!("weather?q={}", &city)).await
    }
    async fn get<T: DeserializeOwned>(&self, query: &str) -> Result<T> {
        let base_http = "https://api.openweathermap.org/data/2.5/";
        let addr = format!("{}{}&APPID=03111683adc5c97b7355ccbe671beb35", base_http, query ); // an api call
        let res = self.client
            .get(addr)
            .send()
            .await?;
        let data = res.json().await?; // Deserializing response from json format
        Ok(data)
    }
}