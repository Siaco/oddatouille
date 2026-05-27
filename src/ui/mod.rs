use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Cell, List, ListItem, Paragraph, Row, Table, Wrap},
    Frame,
};

use crate::app::{ActiveColumn, App};

/// User interface module
pub fn render(app: &App, frame: &mut Frame) {
    let size = frame.area();

    // Split vertically into: Header (3), Main Body (Min 0), Footer (3)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(size);

    draw_header(frame, chunks[0]);
    draw_main_body(app, frame, chunks[1]);
    draw_footer(frame, chunks[2]);
}

fn draw_header(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Cyan));
    
    let paragraph = Paragraph::new(Line::from(vec![
        Span::styled(" oddatui 🦀 📊 ", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw("  |  Stato: "),
        Span::styled("Offline (Mock Mode)", Style::default().fg(Color::Yellow)),
    ]))
    .block(block);
    
    frame.render_widget(paragraph, area);
}

fn draw_main_body(app: &App, frame: &mut Frame, area: Rect) {
    // 3 Columns: 20% (Sport), 30% (Matches), 50% (Odds/Predictions)
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
    let border_color = if is_active { Color::Cyan } else { Color::DarkGray };

    let block = Block::default()
        .title(" Sport ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(border_color));

    let items: Vec<ListItem> = app.sports.iter().enumerate().map(|(i, sport)| {
        let style = if i == app.selected_sport {
            Style::default().bg(Color::DarkGray).fg(Color::White).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        };
        ListItem::new(Line::from(format!(" {}", sport.name))).style(style)
    }).collect();

    let list = List::new(items).block(block);
    frame.render_widget(list, area);
}

fn draw_matches_column(app: &App, frame: &mut Frame, area: Rect) {
    let is_active = app.active_column == ActiveColumn::Matches;
    let border_color = if is_active { Color::Magenta } else { Color::DarkGray };

    let block = Block::default()
        .title(" Eventi Antepost ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(border_color));

    let mut items = Vec::new();
    if let Some(sport) = app.sports.get(app.selected_sport) {
        for (i, match_data) in sport.matches.iter().enumerate() {
            let style = if i == app.selected_match {
                Style::default().bg(Color::DarkGray).fg(Color::White).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };
            items.push(ListItem::new(Line::from(format!(" {}", match_data.title))).style(style));
        }
    }

    let list = List::new(items).block(block);
    frame.render_widget(list, area);
}

fn draw_odds_column(app: &App, frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(" Analisi & Quote ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray));

    // Split this column vertically: Top for odds table, Bottom for prediction
    let inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),
            Constraint::Length(6), // Prediction block size
        ])
        .margin(1) // Padding inside the block
        .split(area);

    frame.render_widget(block, area);

    if let Some(sport) = app.sports.get(app.selected_sport) {
        if let Some(match_data) = sport.matches.get(app.selected_match) {
            // 1. Draw Table
            let header_cells = ["Scelta", "Quota"]
                .iter()
                .map(|h| Cell::from(*h).style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)));
            let header = Row::new(header_cells).style(Style::default().bg(Color::DarkGray)).height(1).bottom_margin(1);
            
            let rows = match_data.odds.iter().map(|odd| {
                let color = if odd.price < 2.0 { Color::Green } else if odd.price < 4.0 { Color::Yellow } else { Color::Red };
                Row::new(vec![
                    Cell::from(odd.name.clone()),
                    Cell::from(format!("{:.2}", odd.price)).style(Style::default().fg(color)),
                ])
            });

            let table = Table::new(rows, [Constraint::Percentage(70), Constraint::Percentage(30)])
                .header(header)
                .column_spacing(1);
            
            frame.render_widget(table, inner_chunks[0]);

            // 2. Draw Prediction Block
            let pred_block = Block::default()
                .title(" 🤖 Insight Predittivo ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Yellow));

            let pred_text = Paragraph::new(match_data.prediction.clone())
                .block(pred_block)
                .wrap(Wrap { trim: true });

            frame.render_widget(pred_text, inner_chunks[1]);
        }
    }
}

fn draw_footer(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::DarkGray));
    
    let paragraph = Paragraph::new(Line::from(vec![
        Span::raw(" Comandi: "),
        Span::styled("[↑/↓]", Style::default().fg(Color::Cyan)),
        Span::raw(" Naviga Liste  |  "),
        Span::styled("[Tab, ←/→]", Style::default().fg(Color::Cyan)),
        Span::raw(" Cambia Colonna  |  "),
        Span::styled("[q]", Style::default().fg(Color::Cyan)),
        Span::raw(" Esci"),
    ]))
    .block(block);
    
    frame.render_widget(paragraph, area);
}
