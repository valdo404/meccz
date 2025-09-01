use std::ffi::CString;
use std::os::raw::{c_char, c_double};

use crate::interfaces::{Location, QiblaCalculator};
use crate::qibla::GreatCircleCalculator;

/// Represents a Qibla result that can be passed to C/Swift
#[repr(C)]
pub struct QiblaResult {
    pub bearing: c_double,
    pub distance_km: c_double,
    pub direction: *mut c_char,
    pub success: bool,
    pub error_message: *mut c_char,
}

impl Drop for QiblaResult {
    fn drop(&mut self) {
        if !self.direction.is_null() {
            unsafe { let _ = CString::from_raw(self.direction); };
        }
        if !self.error_message.is_null() {
            unsafe { let _ = CString::from_raw(self.error_message); };
        }
    }
}

/// Calculates Qibla direction from latitude and longitude
/// 
/// # Safety
/// The returned QiblaResult must be freed using free_qibla_result
#[unsafe(no_mangle)]
pub extern "C" fn calculate_qibla_ffi(latitude: c_double, longitude: c_double) -> *mut QiblaResult {
    let location = Location {
        latitude,
        longitude,
    };

    // Validate coordinates
    if !(-90.0..=90.0).contains(&latitude) || !(-180.0..=180.0).contains(&longitude) {
        let error_msg = CString::new("Invalid coordinates: latitude must be between -90 and 90, longitude between -180 and 180")
            .unwrap_or_else(|_| CString::new("Invalid coordinates").unwrap());
        
        return Box::into_raw(Box::new(QiblaResult {
            bearing: 0.0,
            distance_km: 0.0,
            direction: std::ptr::null_mut(),
            success: false,
            error_message: error_msg.into_raw(),
        }));
    }

    let calculator = GreatCircleCalculator::new();
    let qibla = calculator.calculate_qibla(&location);

    let direction_cstring = match CString::new(qibla.direction) {
        Ok(s) => s,
        Err(_) => {
            let error_msg = CString::new("Failed to convert direction string").unwrap();
            return Box::into_raw(Box::new(QiblaResult {
                bearing: 0.0,
                distance_km: 0.0,
                direction: std::ptr::null_mut(),
                success: false,
                error_message: error_msg.into_raw(),
            }));
        }
    };

    Box::into_raw(Box::new(QiblaResult {
        bearing: qibla.bearing,
        distance_km: qibla.distance_km,
        direction: direction_cstring.into_raw(),
        success: true,
        error_message: std::ptr::null_mut(),
    }))
}

/// Frees a QiblaResult allocated by calculate_qibla_ffi
/// 
/// # Safety
/// The pointer must have been returned by calculate_qibla_ffi and not previously freed
#[unsafe(no_mangle)]
pub extern "C" fn free_qibla_result(result: *mut QiblaResult) {
    if !result.is_null() {
        unsafe {
            let _ = Box::from_raw(result);
        }
    }
}

/// Represents a compass table entry that can be passed to C/Swift
#[repr(C)]
pub struct CompassEntryC {
    pub direction: *mut c_char,
    pub bearing: c_double,
    pub angular_difference: c_double,
    pub short_path_distance_km: c_double,
    pub long_path_distance_km: c_double,
    pub is_optimal_direction: bool,
}

/// Represents a compass table that can be passed to C/Swift
#[repr(C)]
pub struct CompassTableC {
    pub latitude: c_double,
    pub longitude: c_double,
    pub qibla_bearing: c_double,
    pub direct_distance_km: c_double,
    pub entries: *mut CompassEntryC,
    pub entries_count: usize,
    pub success: bool,
    pub error_message: *mut c_char,
}

/// Calculates compass table from latitude and longitude
/// 
/// # Safety
/// The returned CompassTableC must be freed using free_compass_table
#[unsafe(no_mangle)]
pub extern "C" fn calculate_compass_table_ffi(latitude: c_double, longitude: c_double) -> *mut CompassTableC {
    let location = Location {
        latitude,
        longitude,
    };

    // Validate coordinates
    if !(-90.0..=90.0).contains(&latitude) || !(-180.0..=180.0).contains(&longitude) {
        let error_msg = CString::new("Invalid coordinates").unwrap();
        return Box::into_raw(Box::new(CompassTableC {
            latitude: 0.0,
            longitude: 0.0,
            qibla_bearing: 0.0,
            direct_distance_km: 0.0,
            entries: std::ptr::null_mut(),
            entries_count: 0,
            success: false,
            error_message: error_msg.into_raw(),
        }));
    }

    let calculator = GreatCircleCalculator::new();
    let table = calculator.calculate_compass_table(&location);

    // Convert entries to C representation
    let mut c_entries = Vec::with_capacity(table.entries.len());
    
    for entry in table.entries {
        let direction_cstring = match CString::new(entry.direction) {
            Ok(s) => s,
            Err(_) => continue, // Skip invalid entries
        };

        c_entries.push(CompassEntryC {
            direction: direction_cstring.into_raw(),
            bearing: entry.bearing,
            angular_difference: entry.angular_difference,
            short_path_distance_km: entry.short_path_distance_km,
            long_path_distance_km: entry.long_path_distance_km,
            is_optimal_direction: entry.is_optimal_direction,
        });
    }

    let entries_ptr = c_entries.as_mut_ptr();
    let entries_count = c_entries.len();
    std::mem::forget(c_entries); // Prevent Vec from deallocating

    Box::into_raw(Box::new(CompassTableC {
        latitude: table.location.latitude,
        longitude: table.location.longitude,
        qibla_bearing: table.qibla_bearing,
        direct_distance_km: table.direct_distance_km,
        entries: entries_ptr,
        entries_count,
        success: true,
        error_message: std::ptr::null_mut(),
    }))
}

/// Frees a CompassTableC allocated by calculate_compass_table_ffi
/// 
/// # Safety
/// The pointer must have been returned by calculate_compass_table_ffi and not previously freed
#[unsafe(no_mangle)]
pub extern "C" fn free_compass_table(table: *mut CompassTableC) {
    if table.is_null() {
        return;
    }

    unsafe {
        let table = Box::from_raw(table);
        
        if !table.error_message.is_null() {
            let _ = CString::from_raw(table.error_message);
        }
        
        if !table.entries.is_null() {
            let entries = Vec::from_raw_parts(
                table.entries, 
                table.entries_count, 
                table.entries_count
            );
            
            for entry in entries {
                if !entry.direction.is_null() {
                    let _ = CString::from_raw(entry.direction);
                }
            }
        }
    }
}