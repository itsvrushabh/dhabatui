import Foundation
import Combine

public enum ConnectionStatus {
    case connected
    case connecting
    case disconnected

    public var title: String {
        switch self {
        case .connected: return "🟢 Connected"
        case .connecting: return "🟡 Connecting"
        case .disconnected: return "🔴 Offline"
        }
    }
}

public class DhabaNetworkService: ObservableObject {
    @Published public var data: DashboardData?
    @Published public var status: ConnectionStatus = .connecting
    @Published public var counterIp: String = "192.168.1.10" {
        didSet {
            UserDefaults.standard.set(counterIp, forKey: "counter_ip")
        }
    }
    @Published public var counterPort: String = "8080" {
        didSet {
            UserDefaults.standard.set(counterPort, forKey: "counter_port")
        }
    }

    private var timer: AnyCancellable?
    private var lastCachedData: DashboardData?

    public init() {
        if let savedIp = UserDefaults.standard.string(forKey: "counter_ip"), !savedIp.isEmpty {
            self.counterIp = savedIp
        }
        if let savedPort = UserDefaults.standard.string(forKey: "counter_port"), !savedPort.isEmpty {
            self.counterPort = savedPort
        }
        startPolling()
    }

    public var baseUrlString: String {
        let cleanIp = counterIp.trimmingCharacters(in: .whitespacesAndNewlines)
            .replacingOccurrences(of: "http://", with: "")
            .replacingOccurrences(of: "https://", with: "")
            .trimmingCharacters(in: CharacterSet(charactersIn: "/"))
        let port = Int(counterPort.trimmingCharacters(in: .whitespacesAndNewlines)) ?? 8080
        return "http://\(cleanIp):\(port)"
    }

    public func startPolling() {
        fetchStatus()
        timer?.cancel()
        timer = Timer.publish(every: 2.5, on: .main, in: .common)
            .autoconnect()
            .sink { [weak self] _ in
                self?.fetchStatus()
            }
    }

    public func fetchStatus() {
        guard let url = URL(string: "\(baseUrlString)/api/status") else {
            self.status = .disconnected
            return
        }

        var request = URLRequest(url: url)
        request.timeoutInterval = 3.0
        request.cachePolicy = .reloadIgnoringLocalAndRemoteCacheData

        URLSession.shared.dataTask(with: request) { [weak self] data, response, error in
            DispatchQueue.main.async {
                guard let self = self else { return }

                if let data = data,
                   let decoded = try? JSONDecoder().decode(DashboardData.self, from: data) {
                    self.data = decoded
                    self.lastCachedData = decoded
                    self.status = .connected
                } else {
                    // Retain offline cache
                    if self.data == nil {
                        self.data = self.lastCachedData
                    }
                    self.status = .disconnected
                }
            }
        }.resume()
    }
}
