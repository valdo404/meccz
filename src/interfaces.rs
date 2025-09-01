use async_trait::async_trait;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QiblaDirection {
    pub bearing: f64, // degrees from North (0-360)
    pub direction: String, // Cardinal direction (N, NE, E, SE, S, SW, W, NW)
    pub distance_km: f64, // distance to Mecca in kilometers
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompassEntry {
    pub direction: String,
    pub bearing: f64,
    pub angular_difference: f64, // degrees difference from Qibla direction
    pub short_path_distance_km: f64, // distance via shorter great circle
    pub long_path_distance_km: f64, // distance via longer great circle
    pub is_optimal_direction: bool, // true if this is the closest to Qibla direction
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompassTable {
    pub location: Location,
    pub qibla_bearing: f64,
    pub direct_distance_km: f64,
    pub entries: Vec<CompassEntry>,
}

#[async_trait]
pub trait GeocodingService {
    async fn geocode(&self, address: &str) -> Result<Location>;
}

pub trait QiblaCalculator {
    fn calculate_qibla(&self, location: &Location) -> QiblaDirection;
    fn calculate_compass_table(&self, location: &Location) -> CompassTable;
}

#[async_trait]
pub trait Application {
    async fn run(&self, input: &str) -> Result<QiblaDirection>;
}