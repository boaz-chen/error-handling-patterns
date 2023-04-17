use anyhow::{anyhow, Result};

mod void_fn;

pub fn get_temperture() -> Result<i8> {
    Ok(50)
}

pub fn get_weather_description(temperture: i8) -> Result<String> {
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
