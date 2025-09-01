use crate::{
    interfaces::{Application, CompassTable, GeocodingService, Location, QiblaCalculator, QiblaDirection},
    geocoding::parse_coordinates,
};
use anyhow::Result;
use async_trait::async_trait;

pub struct MeccaApp<G, Q>
where
    G: GeocodingService,
    Q: QiblaCalculator,
{
    geocoding_service: G,
    qibla_calculator: Q,
}

impl<G, Q> MeccaApp<G, Q>
where
    G: GeocodingService,
    Q: QiblaCalculator,
{
    pub fn new(geocoding_service: G, qibla_calculator: Q) -> Self {
        Self {
            geocoding_service,
            qibla_calculator,
        }
    }

    pub async fn get_location(&self, input: &str) -> Result<Location> {
        if let Ok(location) = parse_coordinates(input) {
            Ok(location)
        } else {
            self.geocoding_service.geocode(input).await
        }
    }

    pub fn get_compass_table(&self, location: &Location) -> CompassTable {
        self.qibla_calculator.calculate_compass_table(location)
    }

    pub fn get_qibla(&self, location: &Location) -> QiblaDirection {
        self.qibla_calculator.calculate_qibla(location)
    }
}

#[async_trait]
impl<G, Q> Application for MeccaApp<G, Q>
where
    G: GeocodingService + Send + Sync,
    Q: QiblaCalculator + Send + Sync,
{
    async fn run(&self, input: &str) -> Result<QiblaDirection> {
        let location = self.get_location(input).await?;
        Ok(self.qibla_calculator.calculate_qibla(&location))
    }
}