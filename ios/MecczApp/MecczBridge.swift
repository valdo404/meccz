import Foundation

// Rust FFI bridge for Swift
struct MecczBridge {
    
    // Result struct to match Rust
    struct QiblaResult {
        let bearing: Double
        let direction: String
        let distance: Double
        let success: Bool
        let error: String?
    }
    
    struct CompassEntry {
        let direction: String
        let bearing: Double
        let angularDifference: Double
        let shortPathDistance: Double
        let longPathDistance: Double
        let isOptimal: Bool
    }
    
    struct CompassTable {
        let latitude: Double
        let longitude: Double
        let qiblaBearing: Double
        let directDistance: Double
        let entries: [CompassEntry]
        let success: Bool
        let error: String?
    }
    
    // Calculate Qibla direction
    static func calculateQibla(latitude: Double, longitude: Double) -> QiblaResult {
        let result = calculate_qibla_ffi(latitude, longitude)
        defer { free_qibla_result(result) }
        
        guard let resultPtr = result else {
            return QiblaResult(
                bearing: 0,
                direction: "",
                distance: 0,
                success: false,
                error: "Failed to calculate Qibla"
            )
        }
        
        let rustResult = resultPtr.pointee
        
        if rustResult.success {
            let direction = rustResult.direction != nil ? 
                String(cString: rustResult.direction) : ""
            
            return QiblaResult(
                bearing: rustResult.bearing,
                direction: direction,
                distance: rustResult.distance_km,
                success: true,
                error: nil
            )
        } else {
            let errorMsg = rustResult.error_message != nil ?
                String(cString: rustResult.error_message) : "Unknown error"
            
            return QiblaResult(
                bearing: 0,
                direction: "",
                distance: 0,
                success: false,
                error: errorMsg
            )
        }
    }
    
    // Calculate compass table
    static func calculateCompassTable(latitude: Double, longitude: Double) -> CompassTable {
        let result = calculate_compass_table_ffi(latitude, longitude)
        defer { free_compass_table(result) }
        
        guard let resultPtr = result else {
            return CompassTable(
                latitude: 0,
                longitude: 0,
                qiblaBearing: 0,
                directDistance: 0,
                entries: [],
                success: false,
                error: "Failed to calculate compass table"
            )
        }
        
        let rustResult = resultPtr.pointee
        
        if rustResult.success {
            var entries: [CompassEntry] = []
            
            if rustResult.entries != nil && rustResult.entries_count > 0 {
                let entriesBuffer = UnsafeBufferPointer(
                    start: rustResult.entries,
                    count: rustResult.entries_count
                )
                
                for rustEntry in entriesBuffer {
                    let direction = rustEntry.direction != nil ?
                        String(cString: rustEntry.direction) : ""
                    
                    entries.append(CompassEntry(
                        direction: direction,
                        bearing: rustEntry.bearing,
                        angularDifference: rustEntry.angular_difference,
                        shortPathDistance: rustEntry.short_path_distance_km,
                        longPathDistance: rustEntry.long_path_distance_km,
                        isOptimal: rustEntry.is_optimal_direction
                    ))
                }
            }
            
            return CompassTable(
                latitude: rustResult.latitude,
                longitude: rustResult.longitude,
                qiblaBearing: rustResult.qibla_bearing,
                directDistance: rustResult.direct_distance_km,
                entries: entries,
                success: true,
                error: nil
            )
        } else {
            let errorMsg = rustResult.error_message != nil ?
                String(cString: rustResult.error_message) : "Unknown error"
            
            return CompassTable(
                latitude: 0,
                longitude: 0,
                qiblaBearing: 0,
                directDistance: 0,
                entries: [],
                success: false,
                error: errorMsg
            )
        }
    }
}

// Rust FFI function declarations
@_silgen_name("calculate_qibla_ffi")
func calculate_qibla_ffi(_ latitude: Double, _ longitude: Double) -> UnsafeMutablePointer<RustQiblaResult>?

@_silgen_name("free_qibla_result")  
func free_qibla_result(_ result: UnsafeMutablePointer<RustQiblaResult>?)

@_silgen_name("calculate_compass_table_ffi")
func calculate_compass_table_ffi(_ latitude: Double, _ longitude: Double) -> UnsafeMutablePointer<RustCompassTable>?

@_silgen_name("free_compass_table")
func free_compass_table(_ table: UnsafeMutablePointer<RustCompassTable>?)

// Rust structs (C-compatible)
struct RustQiblaResult {
    let bearing: Double
    let distance_km: Double
    let direction: UnsafeMutablePointer<CChar>?
    let success: Bool
    let error_message: UnsafeMutablePointer<CChar>?
}

struct RustCompassEntry {
    let direction: UnsafeMutablePointer<CChar>?
    let bearing: Double
    let angular_difference: Double
    let short_path_distance_km: Double
    let long_path_distance_km: Double
    let is_optimal_direction: Bool
}

struct RustCompassTable {
    let latitude: Double
    let longitude: Double
    let qibla_bearing: Double
    let direct_distance_km: Double
    let entries: UnsafeMutablePointer<RustCompassEntry>?
    let entries_count: Int
    let success: Bool
    let error_message: UnsafeMutablePointer<CChar>?
}