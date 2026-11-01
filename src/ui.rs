use crate::app::{ActivePane, App, InputMode, ToastType};
use crate::models::{format_paise, TableStatus};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, BorderType, Borders, Clear, HighlightSpacing, List, ListItem, Paragraph, Row, Table,
    },
    Frame,
};

pub fn render(f: &mut Frame, app: &mut App) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header banner
            Constraint::Length(3), // Interactive Tables Bar (T1..T8, Parcel)
            Constraint::Min(10),   // Content (Menu + Current Table Order)
            Constraint::Length(1), // Toast / Status line
            Constraint::Length(1), // Hotkey guide footer
        ])
        .split(f.area());

    render_header(f, app, main_chunks[0]);
    render_table_bar(f, app, main_chunks[1]);
    render_body(f, app, main_chunks[2]);
    render_status_bar(f, app, main_chunks[3]);
    render_footer(f, app, main_chunks[4]);

    // Render interactive modal if in input mode
    match app.input_mode {
        InputMode::SelectingPaymentMethod => render_payment_method_modal(f, app),
        InputMode::EditingPrice => render_edit_price_modal(f, app),
        InputMode::AddingCustomName => render_custom_name_modal(f, app),
        InputMode::AddingCustomPrice => render_custom_price_modal(f, app),
        InputMode::Normal | InputMode::QuickCode => {}
    }
}

fn render_header(f: &mut Frame, app: &App, area: Rect) {
    let mut spans = vec![
        Span::styled(
            " 🍲 DHABA TUI ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "  Smart POS & Billing System",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
    ];

    if !app.quick_code_buffer.is_empty() {
        spans.push(Span::styled(
            format!("   [Code: {}_]  ", app.quick_code_buffer),
            Style::default()
                .fg(Color::White)
                .bg(Color::Red)
                .add_modifier(Modifier::BOLD),
        ));
    }

    let title_line = Line::from(spans);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Yellow));

    let paragraph = Paragraph::new(title_line)
        .block(block)
        .alignment(Alignment::Left);
    f.render_widget(paragraph, area);
}

fn render_table_bar(f: &mut Frame, app: &App, area: Rect) {
    let mut spans = Vec::new();
    spans.push(Span::styled(
        " Tables: ",
        Style::default().fg(Color::DarkGray),
    ));

    for (idx, table) in app.tables.iter().enumerate() {
        let is_active = idx == app.active_table_idx;
        let summary = table.summary();

        let status_color = match table.status {
            TableStatus::Vacant => Color::DarkGray,
            TableStatus::Ordering => Color::Yellow,
            TableStatus::Served => Color::Cyan,
            TableStatus::Billed => Color::Green,
        };

        let badge = if table.status == TableStatus::Vacant {
            format!(" {} ", table.id)
        } else {
            format!(
                " {}: {} ({}) ",
                table.id,
                format_paise(summary.total_paise),
                table.status.as_str()
            )
        };

        if is_active {
            spans.push(Span::styled(
                format!("▶{badge}◀"),
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ));
        } else {
            spans.push(Span::styled(
                badge,
                Style::default()
                    .fg(status_color)
                    .add_modifier(Modifier::BOLD),
            ));
        }
        spans.push(Span::raw(" "));
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray))
        .title(Span::styled(
            " Dining Tables & Takeaway (Press 't' to cycle, 's' to toggle status) ",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ));

    let paragraph = Paragraph::new(Line::from(spans)).block(block);
    f.render_widget(paragraph, area);
}

fn render_body(f: &mut Frame, app: &mut App, area: Rect) {
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Left: Menu
            Constraint::Percentage(50), // Right: Active Table Order & Summary
        ])
        .split(area);

    render_menu(f, app, body_chunks[0]);
    render_cart_and_summary(f, app, body_chunks[1]);
}

fn render_menu(f: &mut Frame, app: &mut App, area: Rect) {
    let is_focused = app.active_pane == ActivePane::Menu;
    let border_color = if is_focused {
        Color::Cyan
    } else {
        Color::DarkGray
    };

    let items: Vec<ListItem> = app
        .menu
        .iter()
        .map(|item| {
            let cat_color = match item.category.as_str() {
                "Mains" => Color::LightRed,
                "Breads" => Color::LightYellow,
                "Rice" => Color::LightGreen,
                "Making Chg" => Color::Magenta,
                _ => Color::LightBlue,
            };

            let line = Line::from(vec![
                Span::styled(
                    format!("[{:>3}] ", item.code),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("{:<26}", item.name),
                    Style::default().fg(Color::White),
                ),
                Span::styled(
                    format!("{:<12}", item.category),
                    Style::default().fg(cat_color),
                ),
                Span::styled(
                    format!("{:>9}", format_paise(item.price_paise)),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
            ]);
            ListItem::new(line)
        })
        .collect();

    let title = if is_focused {
        " [1] Menu Catalog ▶ (Enter: Add to Table | Digits: Quick Code) "
    } else {
        " [1] Menu Catalog (Press Tab or 'h' to Focus) "
    };

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(border_color))
                .title(Span::styled(
                    title,
                    Style::default()
                        .fg(if is_focused {
                            Color::Cyan
                        } else {
                            Color::White
                        })
                        .add_modifier(Modifier::BOLD),
                )),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Rgb(30, 60, 90))
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ")
        .highlight_spacing(HighlightSpacing::Always);

    f.render_stateful_widget(list, area, &mut app.menu_state);
}

fn render_cart_and_summary(f: &mut Frame, app: &mut App, area: Rect) {
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(6),    // Cart items table
            Constraint::Length(8), // Financials summary
        ])
        .split(area);

    render_cart(f, app, right_chunks[0]);
    render_summary(f, app, right_chunks[1]);
}

fn render_cart(f: &mut Frame, app: &mut App, area: Rect) {
    let is_focused = app.active_pane == ActivePane::Cart;
    let border_color = if is_focused {
        Color::Cyan
    } else {
        Color::DarkGray
    };

    let header_row = Row::new(vec![
        cell_styled("Code", Color::Yellow),
        cell_styled("Item Name", Color::Yellow),
        cell_styled("Qty", Color::Yellow),
        cell_styled("Unit Price", Color::Yellow),
        cell_styled("Total", Color::Yellow),
    ])
    .style(
        Style::default()
            .bg(Color::Rgb(20, 25, 35))
            .add_modifier(Modifier::BOLD),
    )
    .bottom_margin(1);

    let active_table = app.active_table();
    let rows: Vec<Row> = active_table
        .items
        .iter()
        .map(|item| {
            let name_span = if item.is_custom {
                Span::styled(
                    format!("* {}", item.name),
                    Style::default().fg(Color::Magenta),
                )
            } else {
                Span::styled(item.name.clone(), Style::default().fg(Color::White))
            };

            Row::new(vec![
                Span::styled(
                    format!("[{}]", item.code),
                    Style::default().fg(Color::DarkGray),
                ),
                name_span,
                Span::styled(
                    format!("{:>3}", item.quantity),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("{:>10}", format_paise(item.price_paise)),
                    Style::default().fg(Color::White),
                ),
                Span::styled(
                    format!("{:>10}", format_paise(item.line_total_paise())),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
            ])
        })
        .collect();

    let title = format!(
        " [2] Order: {} [{}] ▶ (+/- Qty | e: Edit Price | d: Del) ",
        active_table.label,
        active_table.status.as_str()
    );

    let widths = [
        Constraint::Length(8),
        Constraint::Min(18),
        Constraint::Length(5),
        Constraint::Length(12),
        Constraint::Length(12),
    ];

    let table = Table::new(rows, widths)
        .header(header_row)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(border_color))
                .title(Span::styled(
                    title,
                    Style::default()
                        .fg(if is_focused {
                            Color::Cyan
                        } else {
                            Color::White
                        })
                        .add_modifier(Modifier::BOLD),
                )),
        )
        .row_highlight_style(
            Style::default()
                .bg(Color::Rgb(30, 60, 90))
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    f.render_stateful_widget(table, area, &mut app.cart_state);
}

fn cell_styled<'a>(text: &'a str, color: Color) -> Span<'a> {
    Span::styled(
        text,
        Style::default().fg(color).add_modifier(Modifier::BOLD),
    )
}

fn render_summary(f: &mut Frame, app: &App, area: Rect) {
    let summary = app.active_table().summary();

    let summary_lines = vec![
        Line::from(vec![
            Span::styled(" Total Items: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{} ({} lines)", summary.total_items, summary.total_lines),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("   |   "),
            Span::styled("Subtotal: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{:>12}", format_paise(summary.subtotal_paise)),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(" CGST (2.5%):  ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{:>10}", format_paise(summary.cgst_paise)),
                Style::default().fg(Color::LightBlue),
            ),
            Span::raw("   |   "),
            Span::styled("SGST (2.5%): ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{:>10}", format_paise(summary.sgst_paise)),
                Style::default().fg(Color::LightBlue),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                " GRAND TOTAL:  ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("{:>14}", format_paise(summary.total_paise)),
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray))
        .title(Span::styled(
            format!(" Bill & Tax (5% GST) - {} ", app.active_table().label),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ));

    let paragraph = Paragraph::new(summary_lines)
        .block(block)
        .alignment(Alignment::Right);
    f.render_widget(paragraph, area);
}

fn render_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let (msg, style) = if let Some((toast_msg, _, toast_type)) = &app.toast {
        let st = match toast_type {
            ToastType::Success => Style::default()
                .fg(Color::Black)
                .bg(Color::Green)
                .add_modifier(Modifier::BOLD),
            ToastType::Error => Style::default()
                .fg(Color::White)
                .bg(Color::Red)
                .add_modifier(Modifier::BOLD),
            ToastType::Info => Style::default().fg(Color::Yellow),
        };
        (format!(" {toast_msg} "), st)
    } else {
        (
            " Ready. 't': Switch Table | 's': Toggle Status | 'p': Settle & Print.".to_string(),
            Style::default().fg(Color::DarkGray),
        )
    };

    let paragraph = Paragraph::new(Span::styled(msg, style));
    f.render_widget(paragraph, area);
}

fn render_footer(f: &mut Frame, _app: &App, area: Rect) {
    let keys = vec![
        Span::styled(
            "t",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(":Table | "),
        Span::styled(
            "s",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(":Status | "),
        Span::styled(
            "Tab",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(":Pane | "),
        Span::styled(
            "Enter",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(":Add | "),
        Span::styled(
            "+/-",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(":Qty | "),
        Span::styled(
            "e",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(":Edit Price | "),
        Span::styled(
            "m",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(":Making Chg | "),
        Span::styled(
            "p",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(":Settle | "),
        Span::styled(
            "q",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(":Quit"),
    ];

    let paragraph = Paragraph::new(Line::from(keys)).alignment(Alignment::Center);
    f.render_widget(paragraph, area);
}

// --- Modals ---

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn render_edit_price_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(50, 25, f.area());
    f.render_widget(Clear, area);

    let item_name = app
        .cart_state
        .selected()
        .and_then(|i| app.active_table().items.get(i))
        .map(|it| it.name.as_str())
        .unwrap_or("Selected Item");

    let text = vec![
        Line::from(vec![
            Span::styled("Item: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                item_name,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("New Unit Price (₹): ", Style::default().fg(Color::Yellow)),
            Span::styled(
                format!("{}_", app.input_buffer),
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "[Enter]",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Commit   "),
            Span::styled(
                "[Esc]",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Cancel"),
        ]),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(Color::Cyan))
        .title(Span::styled(
            " Alter Unit Price on the Go ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ));

    let paragraph = Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Center);
    f.render_widget(paragraph, area);
}

fn render_custom_name_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 28, f.area());
    f.render_widget(Clear, area);

    let text = vec![
        Line::from(Span::styled(
            "Customer-Supplied Meat / Special Cooking (Step 1 of 2)",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("Dish / Service Name: ", Style::default().fg(Color::Yellow)),
            Span::styled(
                format!("{}_", app.input_buffer),
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(Span::styled(
            "(e.g. '1/2kg Mutton Masala (Making)' or '1kg Fish Fry')",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "[Enter]",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Next (Set Price)   "),
            Span::styled(
                "[Esc]",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Cancel"),
        ]),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(Color::Magenta))
        .title(Span::styled(
            " Add Custom / Making Charge ",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ));

    let paragraph = Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Center);
    f.render_widget(paragraph, area);
}

fn render_custom_price_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(50, 25, f.area());
    f.render_widget(Clear, area);

    let text = vec![
        Line::from(vec![
            Span::styled("Item: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                &app.pending_custom_name,
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Cooking / Making Charge (₹): ",
                Style::default().fg(Color::Yellow),
            ),
            Span::styled(
                format!("{}_", app.input_buffer),
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "[Enter]",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Add to Order   "),
            Span::styled(
                "[Esc]",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Cancel"),
        ]),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(Color::Magenta))
        .title(Span::styled(
            " Set Making Price (Step 2 of 2) ",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ));

    let paragraph = Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Center);
    f.render_widget(paragraph, area);
}

fn render_payment_method_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(55, 35, f.area());
    f.render_widget(Clear, area);

    let table = app.active_table();
    let summary = table.summary();

    let text = vec![
        Line::from(vec![
            Span::styled("Settle ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                &table.label,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" — Total Payable: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format_paise(summary.total_paise),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                " [1] ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " 💵 Cash Payment (Default / Enter) ",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                " [2] ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " 📱 UPI / Online (GPay, PhonePe, Paytm QR) ",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                " [3] ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " 📒 Khaata / Credit (Regular Customer) ",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Press [1], [2], [3] or Enter for Cash | [Esc] Cancel",
            Style::default().fg(Color::DarkGray),
        )]),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(Color::Green))
        .title(Span::styled(
            " Select Payment Method ",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ));

    let paragraph = Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Center);
    f.render_widget(paragraph, area);
}
