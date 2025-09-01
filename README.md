# Meccz - Qibla Direction Calculator

A Rust command-line tool to calculate the direction to Mecca (Qibla) from any location on Earth.

**Note**: This is a toy project for learning purposes. The author is not Muslim and this tool was created purely as an educational exercise in geographic calculations and Rust programming.

## Features

- Calculate Qibla direction from coordinates (latitude, longitude) or geocodable addresses
- Support for both human-readable and JSON output formats
- Compass direction table showing distances to Mecca via each bearing
- Comprehensive test coverage (19 unit tests)
- Clean, modular architecture with abstracted interfaces

## Installation

```bash
# Clone the repository
git clone https://github.com/valdo404/meccz.git
cd meccz

# Build the project
cargo build --release

# The binary will be available at target/release/meccz
```

## Usage

### Basic Usage

```bash
# Using coordinates
meccz "48.8566,2.3522"

# Using a geocodable address
meccz "Paris, France"
```

Output:
```
Direction to Mecca:
Bearing: 119.16° from North
Direction: SE
Distance: 4496 km
```

### JSON Output

```bash
meccz --json "New York"
```

Output:
```json
{
  "bearing": 58.48169371717677,
  "direction": "NE",
  "distance_km": 10306.311660231411
}
```

### Compass Direction Table

```bash
meccz --table "Paris"
```

Output:
```
Location: 48.8535, 2.3484
Qibla Direction: 119.2°
Direct Distance to Mecca: 4496 km

Compass Direction Table - Distances to Mecca via Each Direction
================================================================
Direction Bearing  Diff°      Short Path   Long Path    Optimal 
----------------------------------------------------------------------
ESE      112.5   ° 6.7       ° 4621         29534        *       
SE       135.0   ° 15.8      ° 4746         29534                
E        90.0    ° 29.2      ° 5289         29534                
...

* = Closest compass direction to actual Qibla bearing
Short Path = Distance if traveling in this direction (shorter route)  
Long Path = Distance if traveling opposite direction (around the world)
```

## Command Line Options

- `--json, -j`: Output results in JSON format
- `--table, -t`: Display compass direction table
- `--help`: Show help information

## Architecture

The project uses a clean, modular architecture:

- **Interfaces** (`src/interfaces.rs`): Abstract traits for geocoding and calculations
- **Geocoding** (`src/geocoding.rs`): Location parsing and OpenStreetMap integration
- **Qibla Calculator** (`src/qibla.rs`): Great circle calculations for bearing and distance
- **Core Application** (`src/core.rs`): Main application logic
- **CLI** (`src/main.rs`): Command-line interface

## Testing

Run the comprehensive test suite:

```bash
cargo test
```

The project includes 19 unit tests covering:
- Coordinate parsing and validation
- Qibla calculations for various locations
- Compass table generation
- Integration testing with mock geocoding
- Mathematical accuracy verification

## Technical Details

### Calculations

The tool uses spherical trigonometry for accurate calculations:
- **Great Circle Distance**: Haversine formula
- **Bearing Calculation**: Forward azimuth using atan2
- **Kaaba Coordinates**: 21.4225°N, 39.8262°E

### Geocoding

- Uses OpenStreetMap's Nominatim API for address geocoding
- Includes proper User-Agent header and rate limiting respect
- Fallback to coordinate parsing if geocoding fails

## Disclaimer

This tool is provided for educational and reference purposes only. For religious observance, please consult with appropriate religious authorities and use established, traditional methods for determining Qibla direction.

## License

This project is open source. Feel free to use, modify, and distribute as needed.

## Contributing

This is a learning project, but contributions are welcome! Please feel free to:
- Report issues
- Suggest improvements
- Submit pull requests
- Use this code as a reference for your own projects