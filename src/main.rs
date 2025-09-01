use clap::Parser;
use meccz::{
    core::MeccaApp,
    geocoding::NominatimGeocoder,
    qibla::GreatCircleCalculator,
};
use serde_json;

#[derive(Parser)]
#[command(name = "meccz")]
#[command(about = "Calculate the direction to Mecca (Qibla) from any location")]
#[command(version = "1.0")]
struct Cli {
    #[arg(help = "Location as coordinates (lat,lon) or address to geocode")]
    location: String,
    
    #[arg(long, short, help = "Output result in JSON format")]
    json: bool,
    
    #[arg(long, short, help = "Display compass table showing distance to Mecca from each direction")]
    table: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let geocoder = NominatimGeocoder::new();
    let calculator = GreatCircleCalculator::new();
    let app = MeccaApp::new(geocoder, calculator);

    match app.get_location(&cli.location).await {
        Ok(location) => {
            if cli.table {
                let table = app.get_compass_table(&location);
                if cli.json {
                    let output = serde_json::to_string_pretty(&table)?;
                    println!("{}", output);
                } else {
                    display_table(&table);
                }
            } else {
                let qibla = app.get_qibla(&location);
                if cli.json {
                    let output = serde_json::to_string_pretty(&qibla)?;
                    println!("{}", output);
                } else {
                    println!("Direction to Mecca:");
                    println!("Bearing: {:.2}° from North", qibla.bearing);
                    println!("Direction: {}", qibla.direction);
                    println!("Distance: {:.0} km", qibla.distance_km);
                }
            }
        }
        Err(e) => {
            if cli.json {
                let error_output = serde_json::json!({"error": e.to_string()});
                println!("{}", serde_json::to_string_pretty(&error_output)?);
            } else {
                eprintln!("Error: {}", e);
            }
            std::process::exit(1);
        }
    }

    Ok(())
}

fn display_table(table: &meccz::CompassTable) {
    println!("Location: {:.4}, {:.4}", table.location.latitude, table.location.longitude);
    println!("Qibla Direction: {:.1}°", table.qibla_bearing);
    println!("Direct Distance to Mecca: {:.0} km", table.direct_distance_km);
    println!();
    println!("Compass Direction Table - Distances to Mecca via Each Direction");
    println!("================================================================");
    println!("{:<8} {:<8} {:<10} {:<12} {:<12} {:<8}", 
        "Direction", "Bearing", "Diff°", "Short Path", "Long Path", "Optimal");
    println!("{}", "-".repeat(70));
    
    // Sort entries by short path distance to show best routes first
    let mut sorted_entries = table.entries.clone();
    sorted_entries.sort_by(|a, b| a.short_path_distance_km.partial_cmp(&b.short_path_distance_km).unwrap());
    
    for entry in &sorted_entries {
        let optimal_marker = if entry.is_optimal_direction { "*" } else { "" };
        println!("{:<8} {:<8.1}° {:<10.1}° {:<12.0} {:<12.0} {:<8}", 
            entry.direction, 
            entry.bearing, 
            entry.angular_difference,
            entry.short_path_distance_km,
            entry.long_path_distance_km,
            optimal_marker
        );
    }
    
    println!();
    println!("* = Closest compass direction to actual Qibla bearing");
    println!("Short Path = Distance if traveling in this direction (shorter route)");
    println!("Long Path = Distance if traveling opposite direction (around the world)");
}
