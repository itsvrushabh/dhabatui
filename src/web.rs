use crate::models::{format_paise, BillRecord, TableOrder, TableStatus};
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::sync::{Arc, RwLock};
use std::thread;
use tiny_http::{Header, Response, Server};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardItem {
    pub name: String,
    pub quantity: u32,
    pub price_formatted: String,
    pub total_formatted: String,
    pub is_custom: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardTable {
    pub id: String,
    pub label: String,
    pub status: String,
    pub status_color: String,
    pub items: Vec<DashboardItem>,
    pub opened_at: Option<String>,
    pub total_paise: u32,
    pub total_formatted: String,
    pub total_items: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardRecentBill {
    pub id: u64,
    pub table_id: String,
    pub timestamp: String,
    pub total_formatted: String,
    pub items_count: u32,
    pub payment_method: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardData {
    pub tables: Vec<DashboardTable>,
    pub recent_bills: Vec<DashboardRecentBill>,
    pub total_active_tables: usize,
    pub total_live_revenue_formatted: String,
    pub lan_url: String,
    pub updated_at: String,
}

impl DashboardData {
    pub fn build(tables: &[TableOrder], recent_bills: &[BillRecord], lan_url: &str) -> Self {
        let mut active_tables_count = 0;
        let mut total_live_revenue_paise: u64 = 0;

        let dashboard_tables: Vec<DashboardTable> = tables
            .iter()
            .map(|t| {
                let summary = t.summary();
                if t.status != TableStatus::Vacant {
                    active_tables_count += 1;
                    total_live_revenue_paise += summary.total_paise as u64;
                }

                let items: Vec<DashboardItem> = t
                    .items
                    .iter()
                    .map(|it| DashboardItem {
                        name: it.name.clone(),
                        quantity: it.quantity,
                        price_formatted: format_paise(it.price_paise),
                        total_formatted: format_paise(it.line_total_paise()),
                        is_custom: it.is_custom,
                    })
                    .collect();

                DashboardTable {
                    id: t.id.clone(),
                    label: t.label.clone(),
                    status: t.status.as_str().to_string(),
                    status_color: t.status.color_code().to_string(),
                    items,
                    opened_at: t.opened_at.clone(),
                    total_paise: summary.total_paise,
                    total_formatted: format_paise(summary.total_paise),
                    total_items: summary.total_items,
                }
            })
            .collect();

        let dashboard_bills: Vec<DashboardRecentBill> = recent_bills
            .iter()
            .take(10)
            .map(|b| DashboardRecentBill {
                id: b.id,
                table_id: b.table_id.clone(),
                timestamp: b.timestamp.clone(),
                total_formatted: format_paise(b.summary.total_paise),
                items_count: b.summary.total_items,
                payment_method: b.payment_method.as_str().to_string(),
            })
            .collect();

        Self {
            tables: dashboard_tables,
            recent_bills: dashboard_bills,
            total_active_tables: active_tables_count,
            total_live_revenue_formatted: format_paise(total_live_revenue_paise as u32),
            lan_url: lan_url.to_string(),
            updated_at: Local::now().format("%H:%M:%S").to_string(),
        }
    }
}

pub struct WebServerHandle {
    pub lan_url: String,
    pub shared_data: Arc<RwLock<DashboardData>>,
}

/// Detects local network IPv4 address using standard UDP socket probe.
pub fn detect_lan_ip() -> String {
    if let Ok(socket) = UdpSocket::bind("0.0.0.0:0") {
        if socket.connect("8.8.8.8:80").is_ok() {
            if let Ok(addr) = socket.local_addr() {
                let ip = addr.ip();
                if !ip.is_loopback() {
                    return ip.to_string();
                }
            }
        }
    }
    "127.0.0.1".to_string()
}

/// Validates whether an incoming IP address belongs to a private local network.
pub fn is_lan_ip(addr: &IpAddr) -> bool {
    match addr {
        IpAddr::V4(v4) => {
            v4.is_loopback()
                || v4.is_private()
                || v4.is_link_local()
                || *v4 == Ipv4Addr::new(172, 17, 0, 1) // Docker/Container local
        }
        IpAddr::V6(v6) => v6.is_loopback() || v6.is_unicast_link_local(),
    }
}

/// Starts the embedded LAN read-only dashboard server in a background thread.
pub fn start_lan_server(
    initial_data: DashboardData,
) -> Result<WebServerHandle, Box<dyn std::error::Error>> {
    let lan_ip = detect_lan_ip();
    let mut selected_port = 8080;
    let mut server = None;

    // Try ports 8080 to 8089 to handle cases where 8080 is in use
    for port in 8080..8090 {
        match Server::http(SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port)) {
            Ok(s) => {
                selected_port = port;
                server = Some(s);
                break;
            }
            Err(e) => {
                tracing::warn!(port, %e, "Port occupied, trying next port");
            }
        }
    }

    let server = server.ok_or("Failed to bind web server to any local port between 8080-8089")?;
    let lan_url = format!("http://{lan_ip}:{selected_port}");
    tracing::info!(lan_url = %lan_url, "Embedded read-only LAN web dashboard active");

    let mut data = initial_data;
    data.lan_url = lan_url.clone();
    let shared_data = Arc::new(RwLock::new(data));
    let shared_clone = Arc::clone(&shared_data);

    thread::spawn(move || {
        let html_content = include_str!("dashboard.html");

        for request in server.incoming_requests() {
            let remote_addr = request.remote_addr().map(|s| s.ip());
            let is_allowed = remote_addr.as_ref().map(is_lan_ip).unwrap_or(false);

            if !is_allowed {
                tracing::warn!(?remote_addr, "Blocked non-LAN request to dashboard");
                let resp = Response::from_string(
                    "403 Forbidden: Access restricted to local network only.",
                )
                .with_status_code(403);
                let _ = request.respond(resp);
                continue;
            }

            let url = request.url().to_string();
            if url == "/" || url == "/index.html" {
                let mut resp = Response::from_string(html_content);
                if let Ok(header) =
                    Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..])
                {
                    resp.add_header(header);
                }
                let _ = request.respond(resp);
            } else if url == "/api/status" {
                let json = {
                    let guard = shared_clone.read().unwrap();
                    serde_json::to_string(&*guard).unwrap_or_else(|_| "{}".to_string())
                };

                let mut resp = Response::from_string(json);
                if let Ok(header) =
                    Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..])
                {
                    resp.add_header(header);
                }
                if let Ok(cors) = Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..])
                {
                    resp.add_header(cors);
                }
                let _ = request.respond(resp);
            } else {
                let resp = Response::from_string("404 Not Found").with_status_code(404);
                let _ = request.respond(resp);
            }
        }
    });

    Ok(WebServerHandle {
        lan_url,
        shared_data,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lan_ip_detection() {
        let ip = detect_lan_ip();
        assert!(!ip.is_empty());
    }

    #[test]
    fn test_is_lan_ip() {
        assert!(is_lan_ip(&"127.0.0.1".parse().unwrap()));
        assert!(is_lan_ip(&"192.168.1.50".parse().unwrap()));
        assert!(is_lan_ip(&"10.0.0.1".parse().unwrap()));
        assert!(is_lan_ip(&"172.16.0.5".parse().unwrap()));
        assert!(is_lan_ip(&"::1".parse().unwrap()));
        // Public IPs must be rejected
        assert!(!is_lan_ip(&"8.8.8.8".parse().unwrap()));
        assert!(!is_lan_ip(&"1.1.1.1".parse().unwrap()));
        assert!(!is_lan_ip(&"142.250.190.46".parse().unwrap()));
    }

    #[test]
    fn test_dashboard_data_build() {
        let mut t1 = TableOrder::new("T1", "Table 1");
        t1.status = TableStatus::Ordering;
        t1.items.push(crate::models::CartItem {
            code: "101".to_string(),
            name: "Dal Tadka".to_string(),
            price_paise: 13000,
            quantity: 2,
            is_custom: false,
        });

        let tables = vec![t1, TableOrder::new("T2", "Table 2")];
        let recent = vec![BillRecord {
            id: 101,
            table_id: "T1".to_string(),
            timestamp: "2026-11-01 12:00:00".to_string(),
            items: vec![],
            summary: crate::models::BillSummary::default(),
            payment_method: crate::models::PaymentMethod::Upi,
        }];
        let data = DashboardData::build(&tables, &recent, "http://192.168.1.10:8080");

        assert_eq!(data.tables.len(), 2);
        assert_eq!(data.total_active_tables, 1);
        assert_eq!(data.tables[0].total_formatted, "₹273.00");
        assert_eq!(data.lan_url, "http://192.168.1.10:8080");
        assert_eq!(data.recent_bills[0].payment_method, "UPI / Online");
    }
}
