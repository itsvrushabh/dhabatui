import Foundation

public struct DashboardData: Codable {
    public let tables: [DashboardTable]
    public let recentBills: [DashboardRecentBill]?
    public let totalActiveTables: Int
    public let totalLiveRevenueFormatted: String
    public let lanUrl: String
    public let updatedAt: String

    enum CodingKeys: String, CodingKey {
        case tables
        case recentBills = "recent_bills"
        case totalActiveTables = "total_active_tables"
        case totalLiveRevenueFormatted = "total_live_revenue_formatted"
        case lanUrl = "lan_url"
        case updatedAt = "updated_at"
    }
}

public struct DashboardTable: Codable, Identifiable {
    public let id: String
    public let label: String
    public let status: String
    public let statusColor: String
    public let items: [DashboardItem]
    public let openedAt: String?
    public let totalPaise: Int
    public let totalFormatted: String
    public let totalItems: Int

    enum CodingKeys: String, CodingKey {
        case id
        case label
        case status
        case statusColor = "status_color"
        case items
        case openedAt = "opened_at"
        case totalPaise = "total_paise"
        case totalFormatted = "total_formatted"
        case totalItems = "total_items"
    }
}

public struct DashboardItem: Codable, Identifiable {
    public var id: String { name }
    public let name: String
    public let quantity: Int
    public let priceFormatted: String
    public let totalFormatted: String
    public let isCustom: Bool

    enum CodingKeys: String, CodingKey {
        case name
        case quantity
        case priceFormatted = "price_formatted"
        case totalFormatted = "total_formatted"
        case isCustom = "is_custom"
    }
}

public struct DashboardRecentBill: Codable, Identifiable {
    public let id: Int
    public let tableId: String
    public let timestamp: String
    public let totalFormatted: String
    public let itemsCount: Int

    enum CodingKeys: String, CodingKey {
        case id
        case tableId = "table_id"
        case timestamp
        case totalFormatted = "total_formatted"
        case itemsCount = "items_count"
    }
}
