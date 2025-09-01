# Meccz iOS App

Native iOS app for calculating Qibla direction, built with SwiftUI frontend and Rust core.

## Architecture

- **Rust Core**: Calculation logic compiled as static library
- **Swift FFI Bridge**: Safe interface between Swift and Rust
- **SwiftUI Interface**: Modern iOS UI with location services

## Building

### Prerequisites

1. Xcode 15+ with iOS 17+ SDK
2. Rust with iOS targets:
   ```bash
   rustup target add aarch64-apple-ios
   rustup target add x86_64-apple-ios  
   rustup target add aarch64-apple-ios-sim
   ```
3. cbindgen for C header generation:
   ```bash
   cargo install cbindgen
   ```

### Build Process

1. **Build Rust libraries**:
   ```bash
   ./build_ios.sh
   ```

2. **Open in Xcode**:
   - Create new iOS project named "MecczApp"
   - Add Swift files from `ios/MecczApp/`
   - Link static libraries from `ios/libs/`
   - Add bridging header with `ios/include/meccz.h`

### Project Setup

1. **Library Linking**:
   - Add `libmeccz-ios.a` (device builds)
   - Add `libmeccz-simulator.a` (simulator builds)
   - Configure library search paths

2. **Permissions**:
   - Add location permissions to Info.plist:
   ```xml
   <key>NSLocationWhenInUseUsageDescription</key>
   <string>Calculate Qibla direction from your location</string>
   ```

3. **Build Settings**:
   - Set iOS deployment target to 17.0+
   - Enable bitcode: NO (required for Rust libraries)
   - Other Linker Flags: Add required system frameworks

## Features

### Current Implementation
- ✅ Location permission handling
- ✅ Manual coordinate input  
- ✅ Qibla calculation with compass display
- ✅ Distance and bearing information
- ✅ Compass direction table
- ✅ Rust FFI integration

### App Screens

1. **Main View**:
   - Location services integration
   - Manual coordinate input
   - Qibla calculation button
   - Results display with compass

2. **Compass Table View**:
   - 16-direction compass analysis
   - Distance comparisons
   - Optimal direction highlighting

## Technical Details

### Rust-Swift Bridge

The app uses FFI to call Rust functions from Swift:

```swift
// Swift side
let result = MecczBridge.calculateQibla(latitude: lat, longitude: lon)

// Calls Rust FFI
calculate_qibla_ffi(latitude, longitude)
```

### Memory Management

- Rust allocates result structs
- Swift consumes and immediately frees
- No memory leaks with proper cleanup

### Error Handling

- Rust returns success/error status
- Swift propagates errors to UI
- User-friendly error messages

## Security & Privacy

- Location processed locally only
- No network requests for calculation
- Optional OpenStreetMap geocoding (CLI only)
- Privacy-focused design

## Disclaimer

Educational project demonstrating Rust-Swift integration. For religious observance, consult appropriate religious authorities and use established traditional methods.

## Future Enhancements

- [ ] Compass needle animation
- [ ] Offline map integration  
- [ ] Prayer times calculation
- [ ] Apple Watch companion app
- [ ] Widget support