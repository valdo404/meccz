use crate::interfaces::{CompassEntry, CompassTable, Location, QiblaCalculator, QiblaDirection};

const KAABA_LATITUDE: f64 = 21.4225;
const KAABA_LONGITUDE: f64 = 39.8262;
const EARTH_RADIUS_KM: f64 = 6371.0;

pub struct GreatCircleCalculator;

impl GreatCircleCalculator {
    pub fn new() -> Self {
        Self
    }

    fn to_radians(degrees: f64) -> f64 {
        degrees * std::f64::consts::PI / 180.0
    }

    fn to_degrees(radians: f64) -> f64 {
        radians * 180.0 / std::f64::consts::PI
    }

    fn normalize_bearing(bearing: f64) -> f64 {
        let mut normalized = bearing % 360.0;
        if normalized < 0.0 {
            normalized += 360.0;
        }
        normalized
    }

    fn bearing_to_direction(bearing: f64) -> String {
        match bearing {
            b if b >= 337.5 || b < 22.5 => "N".to_string(),
            b if b >= 22.5 && b < 67.5 => "NE".to_string(),
            b if b >= 67.5 && b < 112.5 => "E".to_string(),
            b if b >= 112.5 && b < 157.5 => "SE".to_string(),
            b if b >= 157.5 && b < 202.5 => "S".to_string(),
            b if b >= 202.5 && b < 247.5 => "SW".to_string(),
            b if b >= 247.5 && b < 292.5 => "W".to_string(),
            b if b >= 292.5 && b < 337.5 => "NW".to_string(),
            _ => "N".to_string(),
        }
    }

    fn calculate_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
        let lat1_rad = Self::to_radians(lat1);
        let lon1_rad = Self::to_radians(lon1);
        let lat2_rad = Self::to_radians(lat2);
        let lon2_rad = Self::to_radians(lon2);

        let delta_lat = lat2_rad - lat1_rad;
        let delta_lon = lon2_rad - lon1_rad;

        let a = (delta_lat / 2.0).sin().powi(2)
            + lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().asin();

        EARTH_RADIUS_KM * c
    }

}

impl QiblaCalculator for GreatCircleCalculator {
    fn calculate_qibla(&self, location: &Location) -> QiblaDirection {
        let lat1 = Self::to_radians(location.latitude);
        let lon1 = Self::to_radians(location.longitude);
        let lat2 = Self::to_radians(KAABA_LATITUDE);
        let lon2 = Self::to_radians(KAABA_LONGITUDE);

        let delta_lon = lon2 - lon1;

        let y = delta_lon.sin() * lat2.cos();
        let x = lat1.cos() * lat2.sin() - lat1.sin() * lat2.cos() * delta_lon.cos();

        let bearing = Self::to_degrees(y.atan2(x));
        let normalized_bearing = Self::normalize_bearing(bearing);

        let distance = Self::calculate_distance(
            location.latitude,
            location.longitude,
            KAABA_LATITUDE,
            KAABA_LONGITUDE,
        );

        QiblaDirection {
            bearing: normalized_bearing,
            direction: Self::bearing_to_direction(normalized_bearing),
            distance_km: distance,
        }
    }

    fn calculate_compass_table(&self, location: &Location) -> CompassTable {
        let mut entries = Vec::new();
        let compass_directions = [
            ("N", 0.0),
            ("NNE", 22.5),
            ("NE", 45.0),
            ("ENE", 67.5),
            ("E", 90.0),
            ("ESE", 112.5),
            ("SE", 135.0),
            ("SSE", 157.5),
            ("S", 180.0),
            ("SSW", 202.5),
            ("SW", 225.0),
            ("WSW", 247.5),
            ("W", 270.0),
            ("WNW", 292.5),
            ("NW", 315.0),
            ("NNW", 337.5),
        ];

        // Get the actual Qibla direction for this location
        let qibla = self.calculate_qibla(location);

        let mut min_angular_diff = f64::MAX;
        let mut optimal_direction_name = String::new();

        // First pass: find the optimal direction
        for (direction, bearing) in compass_directions.iter() {
            let mut angular_diff = (bearing - qibla.bearing).abs();
            if angular_diff > 180.0 {
                angular_diff = 360.0 - angular_diff;
            }
            if angular_diff < min_angular_diff {
                min_angular_diff = angular_diff;
                optimal_direction_name = direction.to_string();
            }
        }

        // Second pass: calculate all entries
        for (direction, bearing) in compass_directions.iter() {
            // Calculate the angular difference between this bearing and Qibla
            let mut angular_diff = (bearing - qibla.bearing).abs();
            if angular_diff > 180.0 {
                angular_diff = 360.0 - angular_diff;
            }

            // Calculate the actual distance if we travel in this direction
            
            // If we're going in exactly the right direction, distance = direct distance
            // If we're going perpendicular, distance = infinite
            // If we're going opposite, distance = circumference - direct distance
            
            let short_distance = if angular_diff < 90.0 {
                // Going roughly towards Mecca - calculate actual distance via this route
                qibla.distance_km / (angular_diff * std::f64::consts::PI / 180.0).cos().max(0.001)
            } else if angular_diff > 90.0 {
                // Going away from Mecca - would need to go the long way around
                EARTH_RADIUS_KM * 2.0 * std::f64::consts::PI - qibla.distance_km
            } else {
                // Perpendicular - theoretically infinite, but let's say it's the full circumference
                EARTH_RADIUS_KM * 2.0 * std::f64::consts::PI
            };
            
            let long_distance = EARTH_RADIUS_KM * 2.0 * std::f64::consts::PI - qibla.distance_km;

            let is_optimal = direction == &optimal_direction_name;

            entries.push(CompassEntry {
                direction: direction.to_string(),
                bearing: *bearing,
                angular_difference: angular_diff,
                short_path_distance_km: short_distance,
                long_path_distance_km: long_distance,
                is_optimal_direction: is_optimal,
            });
        }

        CompassTable {
            location: location.clone(),
            qibla_bearing: qibla.bearing,
            direct_distance_km: qibla.distance_km,
            entries,
        }
    }
}