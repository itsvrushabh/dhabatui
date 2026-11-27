package com.dhabapos.dhabatui.model

import com.google.gson.Gson
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Test

class DhabaModelsTest {

    private val gson = Gson()

    @Test
    fun `dashboard data deserializes snake_case fields`() {
        val json = """
            {
              "tables": [
                {
                  "id": "T1",
                  "label": "Table 1",
                  "status": "Occupied",
                  "status_color": "#E53E3E",
                  "items": [
                    {
                      "name": "Dal Tadka",
                      "quantity": 2,
                      "price_formatted": "₹120.00",
                      "total_formatted": "₹240.00",
                      "is_custom": false
                    }
                  ],
                  "opened_at": "12:30 PM",
                  "total_paise": 24000,
                  "total_formatted": "₹240.00",
                  "total_items": 2
                }
              ],
              "recent_bills": [
                {
                  "id": 42,
                  "table_id": "T2",
                  "timestamp": "12:15 PM",
                  "total_formatted": "₹350.00",
                  "items_count": 3
                }
              ],
              "total_active_tables": 1,
              "total_live_revenue_formatted": "₹240.00",
              "lan_url": "http://192.168.1.10:8080",
              "updated_at": "12:35 PM"
            }
        """.trimIndent()

        val data = gson.fromJson(json, DashboardData::class.java)

        assertEquals(1, data.totalActiveTables)
        assertEquals("₹240.00", data.totalLiveRevenueFormatted)
        assertEquals("http://192.168.1.10:8080", data.lanUrl)
        assertEquals("12:35 PM", data.updatedAt)

        assertEquals(1, data.tables.size)
        val table = data.tables[0]
        assertEquals("T1", table.id)
        assertEquals("Occupied", table.status)
        assertEquals("#E53E3E", table.statusColor)
        assertEquals(24000L, table.totalPaise)
        assertEquals(2, table.totalItems)
        assertEquals("12:30 PM", table.openedAt)
        assertEquals("Dal Tadka", table.items[0].name)

        assertEquals(1, data.recentBills.size)
        assertEquals(42L, data.recentBills[0].id)
        assertEquals("T2", data.recentBills[0].tableId)
        assertEquals(3, data.recentBills[0].itemsCount)
    }

    @Test
    fun `dashboard models have sensible defaults`() {
        val data = DashboardData()
        assertTrue(data.tables.isEmpty())
        assertTrue(data.recentBills.isEmpty())
        assertEquals("₹0.00", data.totalLiveRevenueFormatted)

        val table = DashboardTable()
        assertEquals("Vacant", table.status)
        assertNull(table.openedAt)
        assertEquals(0L, table.totalPaise)
    }
}
