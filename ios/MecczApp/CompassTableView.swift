import SwiftUI

struct CompassTableView: View {
    let latitude: Double
    let longitude: Double
    
    @State private var compassTable: MecczBridge.CompassTable?
    @State private var isLoading = true
    @State private var errorMessage = ""
    @Environment(\.dismiss) private var dismiss
    
    var body: some View {
        NavigationView {
            Group {
                if isLoading {
                    ProgressView("Calculating compass table...")
                        .frame(maxWidth: .infinity, maxHeight: .infinity)
                } else if !errorMessage.isEmpty {
                    VStack {
                        Image(systemName: "exclamationmark.triangle")
                            .font(.largeTitle)
                            .foregroundColor(.orange)
                        Text("Error")
                            .font(.headline)
                        Text(errorMessage)
                            .multilineTextAlignment(.center)
                            .foregroundColor(.secondary)
                    }
                    .padding()
                } else if let table = compassTable {
                    CompassTableContent(table: table)
                }
            }
            .navigationTitle("Compass Table")
            .navigationBarTitleDisplayMode(.inline)
            .navigationBarItems(trailing: Button("Done") { dismiss() })
        }
        .onAppear {
            loadCompassTable()
        }
    }
    
    private func loadCompassTable() {
        DispatchQueue.global(qos: .userInitiated).async {
            let result = MecczBridge.calculateCompassTable(latitude: latitude, longitude: longitude)
            
            DispatchQueue.main.async {
                isLoading = false
                if result.success {
                    compassTable = result
                } else {
                    errorMessage = result.error ?? "Failed to calculate compass table"
                }
            }
        }
    }
}

struct CompassTableContent: View {
    let table: MecczBridge.CompassTable
    
    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 20) {
                // Header info
                VStack(alignment: .leading, spacing: 8) {
                    Text("Location: \(String(format: "%.4f", table.latitude)), \(String(format: "%.4f", table.longitude))")
                        .font(.subheadline)
                        .foregroundColor(.secondary)
                    
                    Text("Qibla Direction: \(String(format: "%.1f", table.qiblaBearing))°")
                        .font(.subheadline)
                        .foregroundColor(.secondary)
                    
                    Text("Direct Distance: \(String(format: "%.0f", table.directDistance)) km")
                        .font(.subheadline)
                        .foregroundColor(.secondary)
                }
                .padding()
                .background(Color(.systemGray6))
                .cornerRadius(10)
                
                // Table explanation
                VStack(alignment: .leading, spacing: 5) {
                    Text("Distance to Mecca from Each Direction")
                        .font(.headline)
                    
                    Text("This table shows the distance to Mecca if you travel 1000km in each compass direction first.")
                        .font(.caption)
                        .foregroundColor(.secondary)
                    
                    HStack {
                        Circle()
                            .fill(Color.green)
                            .frame(width: 8, height: 8)
                        Text("Optimal direction (closest to actual Qibla)")
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                }
                
                // Compass entries
                LazyVStack(spacing: 1) {
                    // Header
                    HStack {
                        Text("Direction")
                            .font(.caption)
                            .fontWeight(.medium)
                            .frame(width: 60, alignment: .leading)
                        
                        Text("Bearing")
                            .font(.caption)
                            .fontWeight(.medium)
                            .frame(width: 60, alignment: .trailing)
                        
                        Text("Diff°")
                            .font(.caption)
                            .fontWeight(.medium)
                            .frame(width: 50, alignment: .trailing)
                        
                        Text("Short Path")
                            .font(.caption)
                            .fontWeight(.medium)
                            .frame(width: 70, alignment: .trailing)
                        
                        Text("Long Path")
                            .font(.caption)
                            .fontWeight(.medium)
                            .frame(width: 70, alignment: .trailing)
                    }
                    .padding(.horizontal)
                    .padding(.vertical, 8)
                    .background(Color(.systemGray5))
                    
                    ForEach(table.entries.sorted(by: { $0.shortPathDistance < $1.shortPathDistance }), id: \.bearing) { entry in
                        CompassEntryRow(entry: entry)
                    }
                }
                .background(Color(.systemBackground))
                .cornerRadius(10)
                .shadow(radius: 1)
                
                // Legend
                VStack(alignment: .leading, spacing: 5) {
                    Text("Legend:")
                        .font(.caption)
                        .fontWeight(.medium)
                    
                    Text("• Short Path: Distance via this direction")
                        .font(.caption)
                        .foregroundColor(.secondary)
                    
                    Text("• Long Path: Distance going opposite direction")
                        .font(.caption)
                        .foregroundColor(.secondary)
                    
                    Text("• Diff°: Angular difference from optimal Qibla")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
                .padding()
                .background(Color(.systemGray6))
                .cornerRadius(10)
            }
            .padding()
        }
    }
}

struct CompassEntryRow: View {
    let entry: MecczBridge.CompassEntry
    
    var body: some View {
        HStack {
            // Direction with optimal indicator
            HStack(spacing: 4) {
                if entry.isOptimal {
                    Circle()
                        .fill(Color.green)
                        .frame(width: 6, height: 6)
                }
                Text(entry.direction)
                    .font(.caption)
                    .fontWeight(entry.isOptimal ? .semibold : .regular)
            }
            .frame(width: 60, alignment: .leading)
            
            Text("\(String(format: "%.1f", entry.bearing))°")
                .font(.caption)
                .frame(width: 60, alignment: .trailing)
            
            Text("\(String(format: "%.1f", entry.angularDifference))°")
                .font(.caption)
                .frame(width: 50, alignment: .trailing)
            
            Text("\(formatDistance(entry.shortPathDistance))")
                .font(.caption)
                .frame(width: 70, alignment: .trailing)
            
            Text("\(formatDistance(entry.longPathDistance))")
                .font(.caption)
                .frame(width: 70, alignment: .trailing)
        }
        .padding(.horizontal)
        .padding(.vertical, 4)
        .background(entry.isOptimal ? Color.green.opacity(0.1) : Color.clear)
    }
    
    private func formatDistance(_ distance: Double) -> String {
        if distance < 1000 {
            return "\(Int(distance))"
        } else {
            return "\(Int(distance / 1000))k"
        }
    }
}

#Preview {
    CompassTableView(latitude: 48.8566, longitude: 2.3522)
}