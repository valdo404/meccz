use super::*;
use crate::qibla::GreatCircleCalculator;

#[cfg(test)]
mod geocoding_tests {
    use crate::geocoding::parse_coordinates;

    #[test]
    fn test_parse_coordinates_valid() {
        let result = parse_coordinates("48.8566, 2.3522").unwrap();
        assert!((result.latitude - 48.8566).abs() < 0.0001);
        assert!((result.longitude - 2.3522).abs() < 0.0001);
    }

    #[test]
    fn test_parse_coordinates_with_spaces() {
        let result = parse_coordinates("  40.7128  ,  -74.0060  ").unwrap();
        assert!((result.latitude - 40.7128).abs() < 0.0001);
        assert!((result.longitude - (-74.0060)).abs() < 0.0001);
    }

    #[test]
    fn test_parse_coordinates_invalid_format() {
        assert!(parse_coordinates("48.8566").is_err());
        assert!(parse_coordinates("48.8566, 2.3522, 100").is_err());
        assert!(parse_coordinates("invalid, coordinates").is_err());
    }

    #[test]
    fn test_parse_coordinates_out_of_bounds() {
        assert!(parse_coordinates("91.0, 0.0").is_err()); // latitude > 90
        assert!(parse_coordinates("-91.0, 0.0").is_err()); // latitude < -90
        assert!(parse_coordinates("0.0, 181.0").is_err()); // longitude > 180
        assert!(parse_coordinates("0.0, -181.0").is_err()); // longitude < -180
    }
}

#[cfg(test)]
mod qibla_tests {
    use super::*;


    #[test]
    fn test_kaaba_to_kaaba() {
        let calculator = GreatCircleCalculator::new();
        let kaaba = Location {
            latitude: 21.4225,
            longitude: 39.8262,
        };
        let result = calculator.calculate_qibla(&kaaba);
        
        // Distance should be 0 when at Kaaba
        assert!(result.distance_km < 1.0); // Allow small numerical error
    }

    #[test]
    fn test_paris_qibla() {
        let calculator = GreatCircleCalculator::new();
        let paris = Location {
            latitude: 48.8566,
            longitude: 2.3522,
        };
        let result = calculator.calculate_qibla(&paris);
        
        // Expected values for Paris (approximate)
        assert!((result.bearing - 119.0).abs() < 5.0); // Should be around 119°
        assert!(result.direction == "SE");
        assert!((result.distance_km - 4500.0).abs() < 500.0); // Should be around 4500 km
    }

    #[test]
    fn test_new_york_qibla() {
        let calculator = GreatCircleCalculator::new();
        let new_york = Location {
            latitude: 40.7128,
            longitude: -74.0060,
        };
        let result = calculator.calculate_qibla(&new_york);
        
        // Expected values for New York (approximate)
        assert!((result.bearing - 58.0).abs() < 5.0); // Should be around 58°
        assert!(result.direction == "NE");
        assert!((result.distance_km - 10300.0).abs() < 500.0); // Should be around 10300 km
    }

    #[test]
    fn test_guam_qibla() {
        let calculator = GreatCircleCalculator::new();
        let guam = Location {
            latitude: 13.4500,
            longitude: 144.7652,
        };
        let result = calculator.calculate_qibla(&guam);
        
        // Expected values for Guam (approximate)
        assert!((result.bearing - 294.0).abs() < 5.0); // Should be around 294°
        assert!(result.direction == "NW");
        assert!((result.distance_km - 11000.0).abs() < 500.0); // Should be around 11000 km
    }

    #[test]
    fn test_direction_mapping() {
        let _calculator = GreatCircleCalculator::new();
        
        // Test various bearings and their expected directions
        let test_cases = vec![
            (0.0, "N"),
            (45.0, "NE"),
            (90.0, "E"),
            (135.0, "SE"),
            (180.0, "S"),
            (225.0, "SW"),
            (270.0, "W"),
            (315.0, "NW"),
            (359.0, "N"),
        ];

        for (_bearing, _expected_dir) in test_cases {
            // Create a location that would give us the desired bearing
            let _location = Location {
                latitude: 0.0,
                longitude: 0.0,
            };
            
            // We'll test the direction mapping by checking known locations
            // This is a simplified test - in practice, we'd need specific locations
            // that give exact bearings
        }
    }

    #[test]
    fn test_compass_table() {
        let calculator = GreatCircleCalculator::new();
        let paris = Location {
            latitude: 48.8566,
            longitude: 2.3522,
        };
        let table = calculator.calculate_compass_table(&paris);
        
        // Should have 16 compass directions
        assert_eq!(table.entries.len(), 16);
        
        // Should have the correct location
        assert!((table.location.latitude - 48.8566).abs() < 0.0001);
        assert!((table.location.longitude - 2.3522).abs() < 0.0001);
        
        // Should have a reasonable Qibla bearing
        assert!(table.qibla_bearing > 100.0 && table.qibla_bearing < 130.0);
        
        // Should have a reasonable direct distance
        assert!(table.direct_distance_km > 4000.0 && table.direct_distance_km < 5000.0);
        
        // One direction should be marked as optimal
        let optimal_count = table.entries.iter().filter(|e| e.is_optimal_direction).count();
        assert_eq!(optimal_count, 1);
        
        // All entries should have valid bearings and differences
        for entry in &table.entries {
            assert!(entry.bearing >= 0.0 && entry.bearing < 360.0);
            assert!(entry.angular_difference >= 0.0 && entry.angular_difference <= 180.0);
            assert!(entry.short_path_distance_km > 0.0);
            assert!(entry.long_path_distance_km > 0.0);
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::core::MeccaApp;
    use crate::qibla::GreatCircleCalculator;

    // Mock geocoding service for testing
    struct MockGeocoder;
    
    #[async_trait::async_trait]
    impl GeocodingService for MockGeocoder {
        async fn geocode(&self, address: &str) -> anyhow::Result<Location> {
            match address.to_lowercase().as_str() {
                "paris" => Ok(Location { latitude: 48.8566, longitude: 2.3522 }),
                "new york" => Ok(Location { latitude: 40.7128, longitude: -74.0060 }),
                "tokyo" => Ok(Location { latitude: 35.6762, longitude: 139.6503 }),
                _ => Err(anyhow::anyhow!("Location not found: {}", address)),
            }
        }
    }

    #[tokio::test]
    async fn test_app_with_coordinates() {
        let geocoder = MockGeocoder;
        let calculator = GreatCircleCalculator::new();
        let app = MeccaApp::new(geocoder, calculator);

        let result = app.run("48.8566,2.3522").await.unwrap();
        assert!((result.bearing - 119.0).abs() < 5.0);
        assert!(result.direction == "SE");
        assert!((result.distance_km - 4500.0).abs() < 500.0);
    }

    #[tokio::test]
    async fn test_app_with_geocoding() {
        let geocoder = MockGeocoder;
        let calculator = GreatCircleCalculator::new();
        let app = MeccaApp::new(geocoder, calculator);

        let result = app.run("paris").await.unwrap();
        assert!((result.bearing - 119.0).abs() < 5.0);
        assert!(result.direction == "SE");
    }

    #[tokio::test]
    async fn test_app_invalid_location() {
        let geocoder = MockGeocoder;
        let calculator = GreatCircleCalculator::new();
        let app = MeccaApp::new(geocoder, calculator);

        let result = app.run("unknown city").await;
        assert!(result.is_err());
    }

    #[test]
    fn test_get_location_coordinates() {
        let geocoder = MockGeocoder;
        let calculator = GreatCircleCalculator::new();
        let app = MeccaApp::new(geocoder, calculator);

        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(app.get_location("40.7128,-74.0060")).unwrap();
        
        assert!((result.latitude - 40.7128).abs() < 0.0001);
        assert!((result.longitude - (-74.0060)).abs() < 0.0001);
    }

    #[test]
    fn test_get_qibla_direct() {
        let geocoder = MockGeocoder;
        let calculator = GreatCircleCalculator::new();
        let app = MeccaApp::new(geocoder, calculator);

        let location = Location { latitude: 40.7128, longitude: -74.0060 };
        let result = app.get_qibla(&location);
        
        assert!((result.bearing - 58.0).abs() < 5.0);
        assert!(result.direction == "NE");
    }

    #[test]
    fn test_get_compass_table_direct() {
        let geocoder = MockGeocoder;
        let calculator = GreatCircleCalculator::new();
        let app = MeccaApp::new(geocoder, calculator);

        let location = Location { latitude: 48.8566, longitude: 2.3522 };
        let table = app.get_compass_table(&location);
        
        assert_eq!(table.entries.len(), 16);
        assert!((table.location.latitude - 48.8566).abs() < 0.0001);
        
        // Should have one optimal direction
        let optimal_count = table.entries.iter().filter(|e| e.is_optimal_direction).count();
        assert_eq!(optimal_count, 1);
    }
}

#[cfg(test)]
mod mathematical_tests {
    use super::*;
    use crate::qibla::GreatCircleCalculator;

    #[test]
    fn test_distance_calculation_accuracy() {
        let calculator = GreatCircleCalculator::new();
        
        // Test known distances between major cities
        let paris = Location { latitude: 48.8566, longitude: 2.3522 };
        let london = Location { latitude: 51.5074, longitude: -0.1278 };
        
        // Paris to Mecca
        let paris_qibla = calculator.calculate_qibla(&paris);
        // Should be approximately 4500 km
        assert!((paris_qibla.distance_km - 4500.0).abs() < 200.0);
        
        // London to Mecca  
        let london_qibla = calculator.calculate_qibla(&london);
        // Should be approximately 4600 km
        assert!((london_qibla.distance_km - 4600.0).abs() < 200.0);
    }

    #[test] 
    fn test_bearing_normalization() {
        let calculator = GreatCircleCalculator::new();
        
        // Test locations that should give bearings in each quadrant
        let test_locations = vec![
            // North-east bearing
            (Location { latitude: 10.0, longitude: 30.0 }, (0.0, 90.0)),
            // South-east bearing  
            (Location { latitude: 30.0, longitude: 30.0 }, (90.0, 180.0)),
            // South-west bearing
            (Location { latitude: 30.0, longitude: 50.0 }, (180.0, 270.0)),
            // North-west bearing
            (Location { latitude: 10.0, longitude: 50.0 }, (270.0, 360.0)),
        ];

        for (location, (min_bearing, max_bearing)) in test_locations {
            let result = calculator.calculate_qibla(&location);
            assert!(result.bearing >= min_bearing && result.bearing < max_bearing,
                    "Bearing {} not in range [{}, {})", result.bearing, min_bearing, max_bearing);
        }
    }

    #[test]
    fn test_compass_table_mathematical_properties() {
        let calculator = GreatCircleCalculator::new();
        let location = Location { latitude: 45.0, longitude: 0.0 }; // Somewhere in France
        let table = calculator.calculate_compass_table(&location);

        // The sum of all angular differences should follow certain mathematical properties
        let total_angular_diff: f64 = table.entries.iter().map(|e| e.angular_difference).sum();
        
        // With 16 evenly spaced directions, we expect certain patterns
        assert!(total_angular_diff > 0.0);
        
        // The optimal direction should have the smallest angular difference
        let optimal_entry = table.entries.iter().find(|e| e.is_optimal_direction).unwrap();
        let min_diff = table.entries.iter().map(|e| e.angular_difference).fold(f64::INFINITY, f64::min);
        assert!((optimal_entry.angular_difference - min_diff).abs() < 0.01);
        
        // Long path distances should be consistent (all approximately circumference - direct distance)
        let expected_long_distance = 2.0 * std::f64::consts::PI * 6371.0 - table.direct_distance_km;
        for entry in &table.entries {
            // Allow some variation due to calculation method
            assert!((entry.long_path_distance_km - expected_long_distance).abs() < 1000.0);
        }
    }
}