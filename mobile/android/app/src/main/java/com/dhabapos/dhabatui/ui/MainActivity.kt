package com.dhabapos.dhabatui.ui

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.grid.GridCells
import androidx.compose.foundation.lazy.grid.LazyVerticalGrid
import androidx.compose.foundation.lazy.grid.items
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Refresh
import androidx.compose.material.icons.filled.Settings
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.dhabapos.dhabatui.model.DashboardData
import com.dhabapos.dhabatui.model.DashboardTable
import com.dhabapos.dhabatui.network.ConnectionStatus
import com.dhabapos.dhabatui.network.DhabaApiClient
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch

class MainActivity : ComponentActivity() {
    private val apiClient = DhabaApiClient()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            DhabaTuiTheme {
                DhabaLiveDashboardScreen(apiClient)
            }
        }
    }
}

@Composable
fun DhabaTuiTheme(content: @Composable () -> Unit) {
    MaterialTheme(
        colorScheme = darkColorScheme(
            background = Color(0xFF0B0F19),
            surface = Color(0xFF161E2E),
            primary = Color(0xFFF59E0B),
            secondary = Color(0xFF06B6D4)
        ),
        content = content
    )
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun DhabaLiveDashboardScreen(apiClient: DhabaApiClient) {
    var dashboardData by remember { mutableStateOf<DashboardData?>(null) }
    var connectionStatus by remember { mutableStateOf(ConnectionStatus.CONNECTING) }
    var showSettingsDialog by remember { mutableStateOf(false) }
    var counterIp by remember { mutableStateOf("192.168.1.10") }
    var counterPort by remember { mutableStateOf("8080") }

    val coroutineScope = rememberCoroutineScope()

    // Real-time polling loop (every 2.5 seconds)
    LaunchedEffect(Unit) {
        while (true) {
            val result = apiClient.fetchStatus()
            if (result.isSuccess) {
                dashboardData = result.getOrNull()
                connectionStatus = ConnectionStatus.CONNECTED
            } else {
                dashboardData = apiClient.getLastCachedData()
                connectionStatus = ConnectionStatus.DISCONNECTED
            }
            delay(2500)
        }
    }

    Scaffold(
        topBar = {
            TopAppBar(
                title = {
                    Row(verticalAlignment = Alignment.CenterVertically) {
                        Text(
                            text = "🍲 DHABA TUI LIVE",
                            fontWeight = FontWeight.Bold,
                            color = Color(0xFFF59E0B),
                            fontSize = 18.sp
                        )
                        Spacer(modifier = Modifier.width(12.dp))
                        ConnectionStatusBadge(connectionStatus)
                    }
                },
                actions = {
                    IconButton(onClick = {
                        coroutineScope.launch {
                            connectionStatus = ConnectionStatus.CONNECTING
                            val res = apiClient.fetchStatus()
                            if (res.isSuccess) {
                                dashboardData = res.getOrNull()
                                connectionStatus = ConnectionStatus.CONNECTED
                            } else {
                                connectionStatus = ConnectionStatus.DISCONNECTED
                            }
                        }
                    }) {
                        Icon(Icons.Default.Refresh, contentDescription = "Refresh", tint = Color.White)
                    }
                    IconButton(onClick = { showSettingsDialog = true }) {
                        Icon(Icons.Default.Settings, contentDescription = "Settings", tint = Color.White)
                    }
                },
                colors = TopAppBarDefaults.topAppBarColors(
                    containerColor = Color(0xFF0B0F19)
                )
            )
        },
        containerColor = Color(0xFF0B0F19)
    ) { paddingValues ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(paddingValues)
                .padding(horizontal = 12.dp)
        ) {
            // KPI Summary Row
            SummaryKpiRow(dashboardData)

            Spacer(modifier = Modifier.height(12.dp))

            // Tables Grid (Read-Only)
            val tables = dashboardData?.tables ?: emptyList()
            if (tables.isEmpty() && connectionStatus == ConnectionStatus.CONNECTING) {
                Box(modifier = Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
                    CircularProgressIndicator(color = Color(0xFFF59E0B))
                }
            } else if (tables.isEmpty()) {
                Box(modifier = Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
                    Text(
                        text = "Connecting to counter at ${apiClient.getBaseUrl()}...\nTap Settings (⚙) to configure counter IP",
                        color = Color(0xFF94A3B8),
                        fontSize = 14.sp
                    )
                }
            } else {
                LazyVerticalGrid(
                    columns = GridCells.Adaptive(minSize = 280.dp),
                    horizontalArrangement = Arrangement.spacedBy(12.dp),
                    verticalArrangement = Arrangement.spacedBy(12.dp),
                    modifier = Modifier.fillMaxSize()
                ) {
                    items(tables) { table ->
                        TableOrderCard(table)
                    }
                }
            }
        }
    }

    if (showSettingsDialog) {
        AlertDialog(
            onDismissRequest = { showSettingsDialog = false },
            title = { Text("Configure Counter IP", color = Color.White) },
            text = {
                Column {
                    Text("Enter local IP address of cashier terminal:", color = Color(0xFF94A3B8), fontSize = 13.sp)
                    Spacer(modifier = Modifier.height(8.dp))
                    OutlinedTextField(
                        value = counterIp,
                        onValueChange = { counterIp = it },
                        label = { Text("IP Address (e.g. 192.168.1.50)") },
                        singleLine = true
                    )
                    Spacer(modifier = Modifier.height(8.dp))
                    OutlinedTextField(
                        value = counterPort,
                        onValueChange = { counterPort = it },
                        label = { Text("Port (default: 8080)") },
                        singleLine = true
                    )
                }
            },
            confirmButton = {
                TextButton(onClick = {
                    apiClient.updateBaseUrl(counterIp, counterPort.toIntOrNull() ?: 8080)
                    showSettingsDialog = false
                    connectionStatus = ConnectionStatus.CONNECTING
                }) {
                    Text("Connect", color = Color(0xFF10B981), fontWeight = FontWeight.Bold)
                }
            },
            dismissButton = {
                TextButton(onClick = { showSettingsDialog = false }) {
                    Text("Cancel", color = Color(0xFF94A3B8))
                }
            },
            containerColor = Color(0xFF161E2E)
        )
    }
}

@Composable
fun ConnectionStatusBadge(status: ConnectionStatus) {
    val (dotColor, text) = when (status) {
        ConnectionStatus.CONNECTED -> Color(0xFF10B981) to "🟢 Connected"
        ConnectionStatus.CONNECTING -> Color(0xFFF59E0B) to "🟡 Connecting"
        ConnectionStatus.DISCONNECTED -> Color(0xFFEF4444) to "🔴 Offline"
    }

    Row(
        modifier = Modifier
            .background(Color(0xFF161E2E), RoundedCornerShape(12.dp))
            .padding(horizontal = 8.dp, vertical = 4.dp),
        verticalAlignment = Alignment.CenterVertically
    ) {
        Box(
            modifier = Modifier
                .size(8.dp)
                .background(dotColor, CircleShape)
        )
        Spacer(modifier = Modifier.width(6.dp))
        Text(text = text, color = Color.White, fontSize = 11.sp, fontWeight = FontWeight.SemiBold)
    }
}

@Composable
fun SummaryKpiRow(data: DashboardData?) {
    Row(
        modifier = Modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.spacedBy(8.dp)
    ) {
        KpiCard(
            title = "Active Tables",
            value = "${data?.totalActiveTables ?: 0}",
            modifier = Modifier.weight(1f),
            color = Color(0xFFF59E0B)
        )
        KpiCard(
            title = "Live Revenue",
            value = data?.totalLiveRevenueFormatted ?: "₹0.00",
            modifier = Modifier.weight(1f),
            color = Color(0xFF10B981)
        )
        KpiCard(
            title = "Updated",
            value = data?.updatedAt ?: "--:--",
            modifier = Modifier.weight(1f),
            color = Color(0xFF06B6D4)
        )
    }
}

@Composable
fun KpiCard(title: String, value: String, modifier: Modifier, color: Color) {
    Card(
        modifier = modifier,
        colors = CardDefaults.cardColors(containerColor = Color(0xFF161E2E)),
        shape = RoundedCornerShape(8.dp)
    ) {
        Column(modifier = Modifier.padding(10.dp)) {
            Text(text = title, fontSize = 11.sp, color = Color(0xFF94A3B8))
            Text(text = value, fontSize = 16.sp, fontWeight = FontWeight.Bold, color = color)
        }
    }
}

@Composable
fun TableOrderCard(table: DashboardTable) {
    val statusColor = try {
        Color(android.graphics.Color.parseColor(table.statusColor))
    } catch (_: Exception) {
        Color(0xFF718096)
    }

    Card(
        colors = CardDefaults.cardColors(containerColor = Color(0xFF161E2E)),
        shape = RoundedCornerShape(10.dp),
        modifier = Modifier
            .fillMaxWidth()
            .border(1.dp, Color(0xFF2A374D), RoundedCornerShape(10.dp))
    ) {
        Column(modifier = Modifier.padding(12.dp)) {
            // Header
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically
            ) {
                Text(
                    text = table.label,
                    fontWeight = FontWeight.Bold,
                    fontSize = 16.sp,
                    color = Color.White
                )
                Text(
                    text = table.status.uppercase(),
                    color = Color.Black,
                    fontWeight = FontWeight.Bold,
                    fontSize = 11.sp,
                    modifier = Modifier
                        .background(statusColor, RoundedCornerShape(4.dp))
                        .padding(horizontal = 6.dp, vertical = 2.dp)
                )
            }

            Spacer(modifier = Modifier.height(8.dp))

            // Dish items
            if (table.items.isEmpty()) {
                Text(
                    text = "No active food items",
                    fontSize = 13.sp,
                    color = Color(0xFF94A3B8),
                    modifier = Modifier.padding(vertical = 12.dp)
                )
            } else {
                table.items.forEach { item ->
                    Row(
                        modifier = Modifier
                            .fillMaxWidth()
                            .padding(vertical = 3.dp),
                        horizontalArrangement = Arrangement.SpaceBetween
                    ) {
                        Text(
                            text = if (item.isCustom) "* ${item.name}" else item.name,
                            color = if (item.isCustom) Color(0xFFF472B6) else Color(0xFFE2E8F0),
                            fontSize = 13.sp,
                            modifier = Modifier.weight(1f)
                        )
                        Text(
                            text = "x${item.quantity}",
                            color = Color(0xFFF59E0B),
                            fontSize = 13.sp,
                            fontWeight = FontWeight.Bold,
                            modifier = Modifier.padding(horizontal = 8.dp)
                        )
                        Text(
                            text = item.totalFormatted,
                            color = Color(0xFF10B981),
                            fontFamily = FontFamily.Monospace,
                            fontSize = 13.sp
                        )
                    }
                }
            }

            Divider(color = Color(0xFF2A374D), thickness = 1.dp, modifier = Modifier.padding(vertical = 8.dp))

            // Footer
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically
            ) {
                Text(
                    text = table.openedAt?.let { "Opened $it" } ?: "Ready",
                    fontSize = 11.sp,
                    color = Color(0xFF94A3B8)
                )
                Text(
                    text = table.totalFormatted,
                    fontWeight = FontWeight.Bold,
                    fontSize = 16.sp,
                    color = Color(0xFF10B981)
                )
            }
        }
    }
}
