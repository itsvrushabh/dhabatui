package com.dhabapos.dhabatui.model

import com.google.gson.annotations.SerializedName

data class DashboardData(
    val tables: List<DashboardTable> = emptyList(),
    @SerializedName("recent_bills")
    val recentBills: List<DashboardRecentBill> = emptyList(),
    @SerializedName("total_active_tables")
    val totalActiveTables: Int = 0,
    @SerializedName("total_live_revenue_formatted")
    val totalLiveRevenueFormatted: String = "₹0.00",
    @SerializedName("lan_url")
    val lanUrl: String = "",
    @SerializedName("updated_at")
    val updatedAt: String = ""
)

data class DashboardTable(
    val id: String = "",
    val label: String = "",
    val status: String = "Vacant",
    @SerializedName("status_color")
    val statusColor: String = "#718096",
    val items: List<DashboardItem> = emptyList(),
    @SerializedName("opened_at")
    val openedAt: String? = null,
    @SerializedName("total_paise")
    val totalPaise: Long = 0,
    @SerializedName("total_formatted")
    val totalFormatted: String = "₹0.00",
    @SerializedName("total_items")
    val totalItems: Int = 0
)

data class DashboardItem(
    val name: String = "",
    val quantity: Int = 0,
    @SerializedName("price_formatted")
    val priceFormatted: String = "",
    @SerializedName("total_formatted")
    val totalFormatted: String = "",
    @SerializedName("is_custom")
    val isCustom: Boolean = false
)

data class DashboardRecentBill(
    val id: Long = 0,
    @SerializedName("table_id")
    val tableId: String = "",
    val timestamp: String = "",
    @SerializedName("total_formatted")
    val totalFormatted: String = "",
    @SerializedName("items_count")
    val itemsCount: Int = 0
)
