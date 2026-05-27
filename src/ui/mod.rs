use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Gauge, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{ActiveColumn, App};

// Palette Finance / Modern
const COLOR_BORDER: Color = Color::Rgb(60, 60, 60);
const COLOR_BORDER_ACTIVE: Color = Color::Rgb(100, 150, 255); // Soft Blue Focus
const COLOR_ACTIVE_ITEM_BG: Color = Color::Rgb(35, 50, 80);
const COLOR_ACTIVE_ITEM_FG: Color = Color::White;
const COLOR_TEXT: Color = Color::Rgb(200, 200, 200);

const COLOR_SAFE: Color = Color::Rgb(50, 200, 120); // Green
const COLOR_MID: Color = Color::Rgb(240, 180, 50); // Yellow/Orange
const COLOR_RISK: Color = Color::Rgb(220, 80, 80); // Red

pub fn render(app: &App, frame: &mut Frame) {
    let size = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Main
            Constraint::Length(3), // Footer
        ])
        .split(size);

    draw_header(app, frame, chunks[0]);
    draw_main_body(app, frame, chunks[1]);
    draw_footer(frame, chunks[2]);
}

fn draw_header(app: &App, frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(COLOR_BORDER));
    
    let mut breadcrumb = vec![
        Span::styled(" ODDATUI ", Style::default().fg(COLOR_ACTIVE_ITEM_FG).bg(COLOR_ACTIVE_ITEM_BG).add_modifier(Modifier::BOLD)),
        Span::raw("  "),
    ];

    if app.is_loading {
        breadcrumb.push(Span::styled(" Caricamento dati via API... ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));
    } else if let Some(err) = &app.error_msg {
        breadcrumb.push(Span::styled(format!(" ERRORE: {} ", err), Style::default().fg(Color::Red)));
    } else if let Some(sport) = app.sports.get(app.selected_sport) {
        breadcrumb.push(Span::styled(sport.title.clone(), Style::default().fg(COLOR_TEXT)));
        if let Some(events) = app.events.get(&sport.key) {
            if let Some(match_data) = events.get(app.selected_match) {
                breadcrumb.push(Span::styled(" > ", Style::default().fg(COLOR_BORDER)));
                breadcrumb.push(Span::styled(
                    match_data.sport_title.clone(), // or title if it was present
                    Style::default().fg(COLOR_ACTIVE_ITEM_FG).add_modifier(Modifier::BOLD)
                ));
            }
        }
    }

    let paragraph = Paragraph::new(Line::from(breadcrumb)).block(block);
    frame.render_widget(paragraph, area);
}

fn draw_main_body(app: &App, frame: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(30),
            Constraint::Percentage(50),
        ])
        .split(area);

    draw_sports_column(app, frame, chunks[0]);
    draw_matches_column(app, frame, chunks[1]);
    draw_odds_column(app, frame, chunks[2]);
}

fn draw_sports_column(app: &App, frame: &mut Frame, area: Rect) {
    let is_active = app.active_column == ActiveColumn::Sports;
    let border_color = if is_active { COLOR_BORDER_ACTIVE } else { COLOR_BORDER };

    let block = Block::default()
        .title(Span::styled(" Sport (Antepost) ", Style::default().fg(if is_active { COLOR_ACTIVE_ITEM_FG } else { COLOR_TEXT })))
        .borders(Borders::ALL)
        .border_type(if is_active { BorderType::Thick } else { BorderType::Plain })
        .border_style(Style::default().fg(border_color));

    let items: Vec<ListItem> = app.sports.iter().enumerate().map(|(i, sport)| {
        let style = if i == app.selected_sport {
            Style::default().bg(COLOR_ACTIVE_ITEM_BG).fg(COLOR_ACTIVE_ITEM_FG).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(COLOR_TEXT)
        };
        ListItem::new(Line::from(format!("  {}  ", sport.title))).style(style)
    }).collect();

    let list = List::new(items).block(block);
    frame.render_widget(list, area);
}

fn draw_matches_column(app: &App, frame: &mut Frame, area: Rect) {
    let is_active = app.active_column == ActiveColumn::Matches;
    let border_color = if is_active { COLOR_BORDER_ACTIVE } else { COLOR_BORDER };

    let block = Block::default()
        .title(Span::styled(" Eventi ", Style::default().fg(if is_active { COLOR_ACTIVE_ITEM_FG } else { COLOR_TEXT })))
        .borders(Borders::ALL)
        .border_type(if is_active { BorderType::Thick } else { BorderType::Plain })
        .border_style(Style::default().fg(border_color));

    let mut items = Vec::new();
    if let Some(sport) = app.sports.get(app.selected_sport) {
        if let Some(events) = app.events.get(&sport.key) {
            for (i, match_data) in events.iter().enumerate() {
                let style = if i == app.selected_match {
                    Style::default().bg(COLOR_ACTIVE_ITEM_BG).fg(COLOR_ACTIVE_ITEM_FG).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(COLOR_TEXT)
                };
                let display_title = match_data.home_team.clone().unwrap_or_else(|| match_data.id.clone());
                items.push(ListItem::new(Line::from(format!("  {}  ", display_title))).style(style));
            }
        }
    }

    let list = List::new(items).block(block);
    frame.render_widget(list, area);
}

fn draw_odds_column(app: &App, frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(Span::styled(" Analisi Quote ", Style::default().fg(COLOR_TEXT)))
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(COLOR_BORDER));

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    if app.is_loading { return; }

    if let Some(sport) = app.sports.get(app.selected_sport) {
        if let Some(events) = app.events.get(&sport.key) {
            if let Some(match_data) = events.get(app.selected_match) {
                
                // Estrai gli outcomes dal primo bookmaker
                let mut outcomes = Vec::new();
                let mut bookmaker_name = String::new();
                let mut update_time = String::new();

                if let Some(bookmaker) = match_data.bookmakers.first() {
                    bookmaker_name = bookmaker.title.clone();
                    update_time = bookmaker.last_update.clone();
                    if let Some(market) = bookmaker.markets.iter().find(|m| m.key == "outrights") {
                        outcomes = market.outcomes.clone();
                    }
                }

                // Ordina per probabilità (prezzo più basso = più probabile)
                outcomes.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap_or(std::cmp::Ordering::Equal));

                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Min(10), // Odds list
                        Constraint::Length(5), // Prediction / Info
                    ])
                    .margin(1)
                    .split(inner_area);

                // Draw Odds with Gauges
                let odd_rects = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(outcomes.iter().map(|_| Constraint::Length(2)).collect::<Vec<_>>())
                    .split(chunks[0]);

                for (i, odd) in outcomes.iter().enumerate() {
                    if i >= odd_rects.len() { break; }
                    
                    let odd_area = odd_rects[i];
                    let odd_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([
                            Constraint::Percentage(40), // Name
                            Constraint::Length(8),      // Price
                            Constraint::Percentage(60), // Gauge
                        ])
                        .split(odd_area);

                    frame.render_widget(
                        Paragraph::new(odd.name.clone()).style(Style::default().fg(COLOR_ACTIVE_ITEM_FG)),
                        odd_chunks[0],
                    );

                    let prob = (1.0 / odd.price) * 100.0;
                    let prob_ratio = prob / 100.0;
                    let color = if odd.price < 3.0 { COLOR_SAFE } else if odd.price < 10.0 { COLOR_MID } else { COLOR_RISK };

                    frame.render_widget(
                        Paragraph::new(format!("{:.2}", odd.price))
                            .style(Style::default().fg(color).add_modifier(Modifier::BOLD)),
                        odd_chunks[1],
                    );

                    let gauge = Gauge::default()
                        .gauge_style(Style::default().fg(color).bg(COLOR_BORDER))
                        .ratio(prob_ratio.clamp(0.0, 1.0))
                        .label(format!("{:.1}%", prob));
                    
                    let gauge_area = Rect { x: odd_chunks[2].x, y: odd_chunks[2].y, width: odd_chunks[2].width, height: 1 };
                    frame.render_widget(gauge, gauge_area);
                }

                // Draw Meta Info
                let meta_block = Block::default()
                    .title(" Metadati Bookmaker ")
                    .borders(Borders::TOP)
                    .border_style(Style::default().fg(COLOR_BORDER));

                let info_text = format!("Provider: {}\nUltimo aggiornamento: {}", bookmaker_name, update_time);
                let pred_text = Paragraph::new(info_text)
                    .block(meta_block)
                    .style(Style::default().fg(COLOR_TEXT))
                    .wrap(Wrap { trim: true });

                frame.render_widget(pred_text, chunks[1]);
            }
        }
    }
}

fn draw_footer(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .style(Style::default().fg(COLOR_BORDER));
    
    let paragraph = Paragraph::new(Line::from(vec![
        Span::styled(" [TAB/Freccie] ", Style::default().fg(COLOR_BORDER_ACTIVE)),
        Span::raw("Naviga  "),
        Span::styled(" [Q/ESC] ", Style::default().fg(COLOR_BORDER_ACTIVE)),
        Span::raw("Esci"),
    ]))
    .block(block);
    
    frame.render_widget(paragraph, area);
}
