import SwiftUI

@main
struct MecczApp: App {
    var body: some Scene {
        WindowGroup {
            ContentView()
        }
    }
}

// Info.plist additions needed:
/*
<key>NSLocationWhenInUseUsageDescription</key>
<string>This app uses your location to calculate the direction to Mecca (Qibla). Your location data is processed locally and never shared.</string>
<key>NSLocationAlwaysAndWhenInUseUsageDescription</key>
<string>This app uses your location to calculate the direction to Mecca (Qibla). Your location data is processed locally and never shared.</string>
*/