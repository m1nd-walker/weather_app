mod models;
mod openweather;

use std::fmt::format;
use std::future::IntoFuture;
use std::thread;
use openweather::OpenWeather;

use eframe::{Frame, run_native};
use egui::Context;
use egui::Shape::Rect;
use crate::models::Weather;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    let weather = OpenWeather::new(
        String::from("d73f182a938020bf1085a250816ad9b1"));

    let data = weather.get_by_city("Moscow").await.unwrap();


    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 1000.0]),
        ..Default::default()
    };

    run_native("Weather", options, Box::new(|cc| {
        Box::new(MyApp::new(data))
    }))
}

struct MyApp {
    weather: Weather
}

impl MyApp {
    fn new(weather: Weather) -> Self {
        Self {
            weather
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            thread::spawn( async {
                update(&mut self).await;
            });
            ui.heading("Weather");
            ui.label(format!("Wind: {}", self.weather.wind.speed));
            ui.label(format!("Temperature: {}C", (self.weather.main.temp - 273.3).round()));

        });
    }

}

async fn update(app: &mut MyApp) {
    let weather = OpenWeather::new(
        String::from("d73f182a938020bf1085a250816ad9b1"));

    let data = weather.get_by_city("Moscow").await.unwrap();
    app.weather = data;
}