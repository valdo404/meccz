import SwiftUI
import CoreLocation

struct ContentView: View {
    @StateObject private var locationManager = LocationManager()
    @State private var qiblaResult: QiblaCalculation?
    @State private var inputLatitude = ""
    @State private var inputLongitude = ""
    @State private var errorMessage = ""
    @State private var showingCompassTable = false
    
    var body: some View {
        NavigationView {
            ScrollView {
                VStack(spacing: 20) {
                    // Header
                    VStack {
                        Image(systemName: "location.north.circle.fill")
                            .font(.system(size: 60))
                            .foregroundColor(.blue)
                        
                        Text("Meccz")
                            .font(.largeTitle)
                            .fontWeight(.bold)
                        
                        Text("Find the direction to Mecca")
                            .font(.subheadline)
                            .foregroundColor(.secondary)
                    }
                    .padding(.top)
                    
                    // Location Input Section
                    VStack(alignment: .leading, spacing: 15) {
                        Text("Location")
                            .font(.headline)
                        
                        Button(action: {
                            locationManager.requestLocation()
                        }) {
                            HStack {
                                Image(systemName: "location.fill")
                                Text("Use Current Location")
                            }
                            .frame(maxWidth: .infinity)
                            .padding()
                            .background(Color.blue)
                            .foregroundColor(.white)
                            .cornerRadius(10)
                        }
                        .disabled(locationManager.authorizationStatus != .authorizedWhenInUse && 
                                 locationManager.authorizationStatus != .authorizedAlways)
                        
                        Text("Or enter coordinates manually:")
                            .font(.subheadline)
                            .foregroundColor(.secondary)
                        
                        HStack {
                            VStack(alignment: .leading) {
                                Text("Latitude")
                                    .font(.caption)
                                    .foregroundColor(.secondary)
                                TextField("48.8566", text: $inputLatitude)
                                    .textFieldStyle(RoundedBorderTextFieldStyle())
                                    .keyboardType(.decimalPad)
                            }
                            
                            VStack(alignment: .leading) {
                                Text("Longitude")
                                    .font(.caption)
                                    .foregroundColor(.secondary)
                                TextField("2.3522", text: $inputLongitude)
                                    .textFieldStyle(RoundedBorderTextFieldStyle())
                                    .keyboardType(.decimalPad)
                            }
                        }
                        
                        Button("Calculate Qibla Direction") {
                            calculateQibla()
                        }
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(canCalculate ? Color.green : Color.gray)
                        .foregroundColor(.white)
                        .cornerRadius(10)
                        .disabled(!canCalculate)
                    }
                    .padding()
                    .background(Color(.systemGray6))
                    .cornerRadius(15)
                    
                    // Error Message
                    if !errorMessage.isEmpty {
                        Text(errorMessage)
                            .foregroundColor(.red)
                            .padding()
                            .background(Color.red.opacity(0.1))
                            .cornerRadius(10)
                    }
                    
                    // Results Section
                    if let result = qiblaResult {
                        QiblaResultView(result: result, showCompassTable: $showingCompassTable)
                    }
                }
                .padding()
            }
            .navigationTitle("Qibla Direction")
            .navigationBarTitleDisplayMode(.inline)
            .sheet(isPresented: $showingCompassTable) {
                if let result = qiblaResult {
                    CompassTableView(latitude: result.latitude, longitude: result.longitude)
                }
            }
        }
        .onAppear {
            locationManager.requestPermission()
        }
        .onChange(of: locationManager.location) { location in
            if let location = location {
                inputLatitude = String(format: "%.6f", location.coordinate.latitude)
                inputLongitude = String(format: "%.6f", location.coordinate.longitude)
                calculateQibla()
            }
        }
        .onChange(of: locationManager.error) { error in
            if let error = error {
                errorMessage = "Location error: \(error.localizedDescription)"
            }
        }
    }
    
    private var canCalculate: Bool {
        !inputLatitude.isEmpty && !inputLongitude.isEmpty &&
        Double(inputLatitude) != nil && Double(inputLongitude) != nil
    }
    
    private func calculateQibla() {
        guard let lat = Double(inputLatitude),
              let lon = Double(inputLongitude) else {
            errorMessage = "Please enter valid coordinates"
            return
        }
        
        errorMessage = ""
        
        let result = MecczBridge.calculateQibla(latitude: lat, longitude: lon)
        
        if result.success {
            qiblaResult = QiblaCalculation(
                latitude: lat,
                longitude: lon,
                bearing: result.bearing,
                direction: result.direction,
                distance: result.distance
            )
        } else {
            errorMessage = result.error ?? "Failed to calculate Qibla direction"
        }
    }
}

struct QiblaCalculation {
    let latitude: Double
    let longitude: Double
    let bearing: Double
    let direction: String
    let distance: Double
}

struct QiblaResultView: View {
    let result: QiblaCalculation
    @Binding var showCompassTable: Bool
    
    var body: some View {
        VStack(spacing: 15) {
            Text("Qibla Direction")
                .font(.headline)
            
            // Compass-style direction display
            ZStack {
                Circle()
                    .stroke(Color.gray.opacity(0.3), lineWidth: 2)
                    .frame(width: 200, height: 200)
                
                // North indicator
                Text("N")
                    .font(.caption)
                    .fontWeight(.bold)
                    .offset(y: -90)
                
                // Direction arrow
                Image(systemName: "location.north.fill")
                    .font(.title)
                    .foregroundColor(.red)
                    .rotationEffect(.degrees(result.bearing))
                
                // Bearing text in center
                VStack {
                    Text("\(String(format: "%.1f", result.bearing))°")
                        .font(.title2)
                        .fontWeight(.bold)
                    Text(result.direction)
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
            }
            
            VStack(spacing: 8) {
                HStack {
                    Text("Bearing:")
                        .fontWeight(.medium)
                    Spacer()
                    Text("\(String(format: "%.2f", result.bearing))° from North")
                }
                
                HStack {
                    Text("Direction:")
                        .fontWeight(.medium)
                    Spacer()
                    Text(result.direction)
                }
                
                HStack {
                    Text("Distance:")
                        .fontWeight(.medium)
                    Spacer()
                    Text("\(String(format: "%.0f", result.distance)) km")
                }
                
                HStack {
                    Text("Location:")
                        .fontWeight(.medium)
                    Spacer()
                    Text("\(String(format: "%.4f", result.latitude)), \(String(format: "%.4f", result.longitude))")
                }
            }
            .padding()
            .background(Color(.systemGray6))
            .cornerRadius(10)
            
            Button("View Compass Table") {
                showCompassTable = true
            }
            .padding(.horizontal)
            .padding(.vertical, 8)
            .background(Color.blue)
            .foregroundColor(.white)
            .cornerRadius(8)
        }
        .padding()
        .background(Color(.systemBackground))
        .cornerRadius(15)
        .shadow(radius: 2)
    }
}

#Preview {
    ContentView()
}