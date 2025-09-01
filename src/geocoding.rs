use crate::interfaces::{GeocodingService, Location};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Deserialize)]
struct NominatimResponse {
    lat: String,
    lon: String,
}

pub struct NominatimGeocoder {
    client: reqwest::Client,
}

impl NominatimGeocoder {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl GeocodingService for NominatimGeocoder {
    async fn geocode(&self, address: &str) -> Result<Location> {
        let url = format!(
            "https://nominatim.openstreetmap.org/search?format=json&q={}&limit=1",
            urlencoding::encode(address)
        );

        let response = self.client
            .get(&url)
            .header("User-Agent", "meccz/1.0")
            .send()
            .await?;

        let results: Vec<NominatimResponse> = response.json().await?;
        
        if results.is_empty() {
            return Err(anyhow!("Location not found: {}", address));
        }

        let result = &results[0];
        
        Ok(Location {
            latitude: result.lat.parse()?,
            longitude: result.lon.parse()?,
        })
    }
}

pub fn parse_coordinates(input: &str) -> Result<Location> {
    let parts: Vec<&str> = input.split(',').collect();
    if parts.len() != 2 {
        return Err(anyhow!("Expected format: latitude,longitude"));
    }

    let latitude = parts[0].trim().parse::<f64>()?;
    let longitude = parts[1].trim().parse::<f64>()?;

    if !(-90.0..=90.0).contains(&latitude) {
        return Err(anyhow!("Latitude must be between -90 and 90 degrees"));
    }

    if !(-180.0..=180.0).contains(&longitude) {
        return Err(anyhow!("Longitude must be between -180 and 180 degrees"));
    }

    Ok(Location { latitude, longitude })
}