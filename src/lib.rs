#![allow(dead_code)]

use anyhow::{anyhow, Result};
use log::{error, info};

fn get_current_temperture() -> Result<i8> {
    Ok(50)
}

fn get_weather_description(temperture: i8) -> Result<String> {
    if temperture < -40 {
        Err(anyhow!("too cold"))
    } else if temperture < 0 {
        Ok("freezing".to_string())
    } else if temperture < 10 {
        Ok("cold".to_string())
    } else if temperture < 20 {
        Ok("cool".to_string())
    } else if temperture < 30 {
        Ok("warm".to_string())
    } else if temperture < 40 {
        Ok("hot".to_string())
    } else {
        Err(anyhow!("too hot"))
    }
}

fn early_return() {
    let temperture = get_current_temperture();

    if temperture.is_err() {
        return;
    }

    let temperture = temperture.expect("value to exist");
    let weather_description = get_weather_description(temperture);

    if weather_description.is_err() {
        return;
    }

    let weather_description = weather_description.expect("value to exist");

    info!("weather description: {}", weather_description);
}

fn early_return_if_let() {
    let temperture = get_current_temperture();

    if let Err(e) = temperture {
        error!("error getting temperture: {}", e);
        return;
    }

    let temperture = temperture.expect("value to exist");
    let weather_description = get_weather_description(temperture);

    if let Err(e) = weather_description {
        error!("error getting weather description: {}", e);
        return;
    }

    let weather_description = weather_description.expect("value to exist");

    info!("weather description: {}", weather_description);
}

fn early_return_let_else() {
    let Ok(temperture) = get_current_temperture() else {
        error!("error getting temperture");
        return;
    };

    let Ok(weather_description) = get_weather_description(temperture) else {
        error!("error getting weather description");
        return;
    };

    info!("weather description: {}", weather_description);
}
