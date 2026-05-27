use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Gauge, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{ActiveColumn, App};

// Palette Finance / Modern
const COLOR_BG: Color = Color::Reset; // Keep terminal background
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

    // Costruisci il Breadcrumb dinamico
    let mut breadcrumb = vec![
        Span::styled(
            " ODDATUI ",
            Style::default()
                .fg(COLOR_ACTIVE_ITEM_FG)
                .bg(COLOR_ACTIVE_ITEM_BG)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
    ];

    if let Some(sport) = app.sports.get(app.selected_sport) {
        breadcrumb.push(Span::styled(
            sport.name.clone(),
            Style::default().fg(COLOR_TEXT),
        ));
        if let Some(match_data) = sport.matches.get(app.selected_match) {
            breadcrumb.push(Span::styled(" > ", Style::default().fg(COLOR_BORDER)));
            breadcrumb.push(Span::styled(
                match_data.title.clone(),
                Style::default()
                    .fg(COLOR_ACTIVE_ITEM_FG)
                    .add_modifier(Modifier::BOLD),
            ));
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
    let border_color = if is_active {
        COLOR_BORDER_ACTIVE
    } else {
        COLOR_BORDER
    };

    let block = Block::default()
        .title(Span::styled(
            " Sport ",
            Style::default().fg(if is_active {
                COLOR_ACTIVE_ITEM_FG
            } else {
                COLOR_TEXT
            }),
        ))
        .borders(Borders::ALL)
        .border_type(if is_active {
            BorderType::Thick
        } else {
            BorderType::Plain
        })
        .border_style(Style::default().fg(border_color));

    let items: Vec<ListItem> = app
        .sports
        .iter()
        .enumerate()
        .map(|(i, sport)| {
            let style = if i == app.selected_sport {
                Style::default()
                    .bg(COLOR_ACTIVE_ITEM_BG)
                    .fg(COLOR_ACTIVE_ITEM_FG)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(COLOR_TEXT)
            };
            ListItem::new(Line::from(format!("  {}  ", sport.name))).style(style)
        })
        .collect();

    let list = List::new(items).block(block);
    frame.render_widget(list, area);
}

fn draw_matches_column(app: &App, frame: &mut Frame, area: Rect) {
    let is_active = app.active_column == ActiveColumn::Matches;
    let border_color = if is_active {
        COLOR_BORDER_ACTIVE
    } else {
        COLOR_BORDER
    };

    let block = Block::default()
        .title(Span::styled(
            " Eventi ",
            Style::default().fg(if is_active {
                COLOR_ACTIVE_ITEM_FG
            } else {
                COLOR_TEXT
            }),
        ))
        .borders(Borders::ALL)
        .border_type(if is_active {
            BorderType::Thick
        } else {
            BorderType::Plain
        })
        .border_style(Style::default().fg(border_color));

    let mut items = Vec::new();
    if let Some(sport) = app.sports.get(app.selected_sport) {
        for (i, match_data) in sport.matches.iter().enumerate() {
            let style = if i == app.selected_match {
                Style::default()
                    .bg(COLOR_ACTIVE_ITEM_BG)
                    .fg(COLOR_ACTIVE_ITEM_FG)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(COLOR_TEXT)
            };
            items.push(ListItem::new(Line::from(format!("  {}  ", match_data.title))).style(style));
        }
    }

    let list = List::new(items).block(block);
    frame.render_widget(list, area);
}

fn draw_odds_column(app: &App, frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(Span::styled(
            " Quote & Analisi ",
            Style::default().fg(COLOR_TEXT),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(COLOR_BORDER));

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    if let Some(sport) = app.sports.get(app.selected_sport) {
        if let Some(match_data) = sport.matches.get(app.selected_match) {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(10), Constraint::Length(6)])
                .margin(1) // Padding within the block
                .split(inner_area);

            // Draw Odds with Gauges
            let odd_rects = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    match_data
                        .odds
                        .iter()
                        .map(|_| Constraint::Length(2))
                        .collect::<Vec<_>>(),
                )
                .split(chunks[0]);

            for (i, odd) in match_data.odds.iter().enumerate() {
                if i >= odd_rects.len() {
                    break;
                }

                let odd_area = odd_rects[i];
                let odd_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(40), // Name
                        Constraint::Length(8),      // Price
                        Constraint::Percentage(60), // Gauge
                    ])
                    .split(odd_area);

                // Name
                frame.render_widget(
                    Paragraph::new(odd.name.clone())
                        .style(Style::default().fg(COLOR_ACTIVE_ITEM_FG)),
                    odd_chunks[0],
                );

                // Price & Color logic
                let prob = (1.0 / odd.price) * 100.0;
                let prob_ratio = prob / 100.0;
                let color = if odd.price < 2.0 {
                    COLOR_SAFE
                } else if odd.price < 4.0 {
                    COLOR_MID
                } else {
                    COLOR_RISK
                };

                frame.render_widget(
                    Paragraph::new(format!("{:.2}", odd.price))
                        .style(Style::default().fg(color).add_modifier(Modifier::BOLD)),
                    odd_chunks[1],
                );

                // Gauge
                let gauge = Gauge::default()
                    .gauge_style(Style::default().fg(color).bg(COLOR_BORDER))
                    .ratio(prob_ratio.clamp(0.0, 1.0))
                    .label(format!("{:.1}%", prob));

                let gauge_area = Rect {
                    x: odd_chunks[2].x,
                    y: odd_chunks[2].y,
                    width: odd_chunks[2].width,
                    height: 1, // Single line gauge
                };
                frame.render_widget(gauge, gauge_area);
            }

            // Draw Prediction
            let pred_block = Block::default()
                .title(" Insight Predittivo ")
                .borders(Borders::TOP)
                .border_style(Style::default().fg(COLOR_BORDER));

            let pred_text = Paragraph::new(match_data.prediction.clone())
                .block(pred_block)
                .style(Style::default().fg(COLOR_TEXT))
                .wrap(Wrap { trim: true });

            frame.render_widget(pred_text, chunks[1]);
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
