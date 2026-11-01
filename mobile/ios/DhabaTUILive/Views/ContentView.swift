import SwiftUI

struct ContentView: View {
    @StateObject private var networkService = DhabaNetworkService()
    @State private var showSettings = false

    var body: some View {
        NavigationView {
            ZStack {
                Color(red: 11/255, green: 15/255, blue: 25/255)
                    .ignoresSafeArea()

                VStack(spacing: 12) {
                    // KPI Summary Banner
                    KpiSummaryView(data: networkService.data)

                    // Tables Grid (Read-Only)
                    if let tables = networkService.data?.tables, !tables.isEmpty {
                        ScrollView {
                            LazyVGrid(columns: [GridItem(.adaptive(minimum: 280), spacing: 12)], spacing: 12) {
                                ForEach(tables) { table in
                                    TableCardView(table: table)
                                }
                            }
                            .padding(.horizontal, 12)
                            .padding(.bottom, 16)
                        }
                    } else if networkService.status == .connecting {
                        Spacer()
                        ProgressView("Connecting to counter...")
                            .tint(.orange)
                            .foregroundColor(.gray)
                        Spacer()
                    } else {
                        Spacer()
                        VStack(spacing: 8) {
                            Image(systemName: "wifi.slash")
                                .font(.system(size: 40))
                                .foregroundColor(.red)
                            Text("Disconnected from Counter")
                                .font(.headline)
                                .foregroundColor(.white)
                            Text("Connecting to \(networkService.baseUrlString)\nTap ⚙ to change counter IP")
                                .font(.subheadline)
                                .multilineTextAlignment(.center)
                                .foregroundColor(.gray)
                        }
                        Spacer()
                    }
                }
            }
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .principal) {
                    HStack(spacing: 8) {
                        Text("🍲 DHABA TUI LIVE")
                            .font(.system(size: 16, weight: .bold))
                            .foregroundColor(.orange)
                        Text(networkService.status.title)
                            .font(.system(size: 11, weight: .semibold))
                            .padding(.horizontal, 8)
                            .padding(.vertical, 3)
                            .background(Color(white: 0.15))
                            .cornerRadius(8)
                            .foregroundColor(.white)
                    }
                }
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button(action: { showSettings = true }) {
                        Image(systemName: "gearshape.fill")
                            .foregroundColor(.white)
                    }
                }
            }
            .sheet(isPresented: $showSettings) {
                SettingsView(networkService: networkService, isPresented: $showSettings)
            }
        }
        .navigationViewStyle(.stack)
    }
}

struct KpiSummaryView: View {
    let data: DashboardData?

    var body: some View {
        HStack(spacing: 8) {
            KpiBox(title: "Active Tables", value: "\(data?.totalActiveTables ?? 0)", color: .orange)
            KpiBox(title: "Live Revenue", value: data?.totalLiveRevenueFormatted ?? "₹0.00", color: .green)
            KpiBox(title: "Last Sync", value: data?.updatedAt ?? "--:--", color: .cyan)
        }
        .padding(.horizontal, 12)
    }
}

struct KpiBox: View {
    let title: String
    let value: String
    let color: Color

    var body: some View {
        VStack(alignment: .leading, spacing: 4) {
            Text(title)
                .font(.system(size: 11))
                .foregroundColor(.gray)
            Text(value)
                .font(.system(size: 15, weight: .bold))
                .foregroundColor(color)
        }
        .frame(maxWidth: .infinity, alignment: .leading)
        .padding(10)
        .background(Color(red: 22/255, green: 30/255, blue: 46/255))
        .cornerRadius(8)
    }
}

struct TableCardView: View {
    let table: DashboardTable

    var statusColor: Color {
        switch table.status.lowercased() {
        case "vacant": return .gray
        case "ordering": return .orange
        case "served": return .cyan
        case "billed": return .green
        default: return .gray
        }
    }

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            // Header
            HStack {
                Text(table.label)
                    .font(.system(size: 16, weight: .bold))
                    .foregroundColor(.white)
                Spacer()
                Text(table.status.uppercased())
                    .font(.system(size: 10, weight: .bold))
                    .padding(.horizontal, 6)
                    .padding(.vertical, 2)
                    .background(statusColor)
                    .foregroundColor(.black)
                    .cornerRadius(4)
            }

            Divider().background(Color(white: 0.2))

            // Items list
            if table.items.isEmpty {
                Text("No active food items")
                    .font(.system(size: 13))
                    .foregroundColor(.gray)
                    .padding(.vertical, 8)
            } else {
                VStack(spacing: 4) {
                    ForEach(table.items) { item in
                        HStack {
                            Text(item.isCustom ? "* \(item.name)" : item.name)
                                .font(.system(size: 13))
                                .foregroundColor(item.isCustom ? .pink : .white)
                                .lineLimit(1)
                            Spacer()
                            Text("x\(item.quantity)")
                                .font(.system(size: 13, weight: .bold))
                                .foregroundColor(.orange)
                            Text(item.totalFormatted)
                                .font(.system(size: 13, design: .monospaced))
                                .foregroundColor(.green)
                        }
                    }
                }
            }

            Divider().background(Color(white: 0.2))

            // Footer
            HStack {
                Text(table.openedAt.map { "Opened \($0)" } ?? "Ready")
                    .font(.system(size: 11))
                    .foregroundColor(.gray)
                Spacer()
                Text(table.totalFormatted)
                    .font(.system(size: 16, weight: .bold))
                    .foregroundColor(.green)
            }
        }
        .padding(12)
        .background(Color(red: 22/255, green: 30/255, blue: 46/255))
        .cornerRadius(10)
        .overlay(
            RoundedRectangle(cornerRadius: 10)
                .stroke(Color(red: 42/255, green: 55/255, blue: 77/255), lineWidth: 1)
        )
    }
}

struct SettingsView: View {
    @ObservedObject var networkService: DhabaNetworkService
    @Binding var isPresented: Bool
    @State private var ipText: String = ""
    @State private var portText: String = ""

    var body: some View {
        NavigationView {
            Form {
                Section(header: Text("Cashier Counter Network Setup")) {
                    TextField("IP Address (e.g. 192.168.1.50)", text: $ipText)
                        .autocapitalization(.none)
                        .disableAutocorrection(true)
                    TextField("Port (default 8080)", text: $portText)
                        .keyboardType(.numberPad)
                }

                Section(footer: Text("Connect your iPhone/iPad to the same Wi-Fi network as the DhabaTUI terminal.")) {
                    Button("Save & Connect") {
                        networkService.counterIp = ipText
                        networkService.counterPort = portText
                        networkService.fetchStatus()
                        isPresented = false
                    }
                    .foregroundColor(.orange)
                }
            }
            .navigationTitle("Counter Setup")
            .navigationBarItems(trailing: Button("Cancel") { isPresented = false })
            .onAppear {
                ipText = networkService.counterIp
                portText = networkService.counterPort
            }
        }
    }
}
