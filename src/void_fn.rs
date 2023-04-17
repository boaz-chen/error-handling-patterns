#![allow(dead_code)]

use crate::{get_temperture, get_weather_description};
use anyhow::Context;
use log::{error, info};

fn early_return() {
    let temperture = get_temperture();

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
    let temperture = get_temperture();

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
    let Ok(temperture) = get_temperture() else {
        error!("error getting temperture");
        return;
    };

    let Ok(weather_description) = get_weather_description(temperture) else {
        error!("error getting weather description");
        return;
    };

    info!("weather description: {}", weather_description);
}

fn and_then_high_level_error() {
    let weather_description = get_temperture().and_then(get_weather_description);

    if weather_description.is_err() {
        error!("error getting weather description"); // Won't differentiate between temperture and weather_description errors
        return;
    }

    let weather_description = weather_description.expect("value to exist");

    info!("weather description: {}", weather_description);
}

fn and_then_high_level_error_log_original_error() {
    // inspect_err is better for this but it's unstable
    if let Ok(weather_description) = get_temperture()
        .map_err(|error| {
            error!("error getting temperture: {}", error);
            error
        })
        .and_then(get_weather_description)
        .map_err(|error| {
            error!("error getting weather description: {}", error);
            error
        })
    {
        info!("weather description: {}", weather_description);
    }
}

fn or_else() {
    #[allow(clippy::bind_instead_of_map)]
    if let Ok(weather_description) = get_temperture()
        .or_else(|error| {
            error!("error getting temperture: {}", error);
            Err(error)
        })
        .and_then(get_weather_description)
        .or_else(|error| {
            error!("error getting weather description: {}", error);
            Err(error)
        })
    {
        info!("weather description: {}", weather_description)
    }
}

fn map_or_else() {
    if let Ok(weather_description) = get_temperture()
        .map_or_else(
            |error| {
                error!("error getting temperture: {}", error);
                Err(error)
            },
            get_weather_description,
        )
        .map_or_else(
            |error| {
                error!("error getting weather description: {}", error);
                Err(error)
            },
            Ok,
        )
    {
        info!("weather description: {}", weather_description)
    }
}

fn anyhow_context() {
    let weather_description = get_temperture()
        .context("error getting temperture")
        .and_then(get_weather_description)
        .context("error getting weather description");

    if let Err(e) = weather_description {
        error!("{e}");
        return;
    }

    info!(
        "weather description: {}",
        weather_description.expect("value to exist")
    )
}
