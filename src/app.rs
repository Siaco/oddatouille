use anyhow::Result;
use crossterm::event::{self, Event as CEvent, KeyCode, KeyEventKind};
use ratatui::{backend::Backend, Terminal};
use std::time::Duration;
use std::collections::HashMap;

use crate::api::OddsApi;
use crate::models::{Event, Sport};
use crate::ui;

#[derive(PartialEq)]
pub enum ActiveColumn {
    Sports,
    Matches,
}

pub struct App {
    pub running: bool,
    pub active_column: ActiveColumn,
    
    // Real Data
    pub api: OddsApi,
    pub sports: Vec<Sport>,
    pub events: HashMap<String, Vec<Event>>, // sport_key -> events
    
    // Selection state
    pub selected_sport: usize,
    pub selected_match: usize,
    pub is_loading: bool,
    pub error_msg: Option<String>,
}

impl App {
    pub fn new() -> Result<Self> {
        let api = OddsApi::new()?;
        Ok(Self { 
            running: true,
            active_column: ActiveColumn::Sports,
            api,
            sports: Vec::new(),
            events: HashMap::new(),
            selected_sport: 0,
            selected_match: 0,
            is_loading: false,
            error_msg: None,
        })
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub async fn load_current_sport_outrights(&mut self) {
        if let Some(sport) = self.sports.get(self.selected_sport) {
            let key = sport.key.clone();
            if !self.events.contains_key(&key) {
                self.is_loading = true;
                match self.api.fetch_outrights(&key).await {
                    Ok(events) => { 
                        // Only keep events that actually have bookmakers/odds
                        let valid_events: Vec<Event> = events.into_iter().filter(|e| !e.bookmakers.is_empty()).collect();
                        self.events.insert(key, valid_events); 
                    },
                    Err(e) => { self.error_msg = Some(format!("API Error: {}", e)); },
                }
                self.is_loading = false;
            }
        }
    }

    pub async fn next_sport(&mut self) {
        if self.selected_sport + 1 < self.sports.len() {
            self.selected_sport += 1;
            self.selected_match = 0;
            self.load_current_sport_outrights().await;
        }
    }

    pub async fn prev_sport(&mut self) {
        if self.selected_sport > 0 {
            self.selected_sport -= 1;
            self.selected_match = 0;
            self.load_current_sport_outrights().await;
        }
    }

    pub fn next_match(&mut self) {
        if let Some(sport) = self.sports.get(self.selected_sport) {
            if let Some(events) = self.events.get(&sport.key) {
                if self.selected_match + 1 < events.len() {
                    self.selected_match += 1;
                }
            }
        }
    }

    pub fn prev_match(&mut self) {
        if self.selected_match > 0 {
            self.selected_match -= 1;
        }
    }

    pub fn toggle_column(&mut self) {
        self.active_column = match self.active_column {
            ActiveColumn::Sports => ActiveColumn::Matches,
            ActiveColumn::Matches => ActiveColumn::Sports,
        };
    }

    pub async fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        
        // Initial Fetch
        self.is_loading = true;
        terminal.draw(|frame| ui::render(self, frame))?;
        
        match self.api.fetch_sports().await {
            Ok(s) => {
                self.sports = s.into_iter().filter(|s| s.has_outrights && s.active).collect();
                if !self.sports.is_empty() {
                    self.load_current_sport_outrights().await;
                }
            },
            Err(e) => self.error_msg = Some(format!("API Error: {}", e)),
        }
        self.is_loading = false;

        // Main Loop
        while self.running {
            terminal.draw(|frame| ui::render(self, frame))?;

            if event::poll(Duration::from_millis(250))? {
                if let CEvent::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => self.quit(),
                            KeyCode::Tab | KeyCode::Right | KeyCode::Left => self.toggle_column(),
                            KeyCode::Down | KeyCode::Char('j') => {
                                match self.active_column {
                                    ActiveColumn::Sports => self.next_sport().await,
                                    ActiveColumn::Matches => self.next_match(),
                                }
                            }
                            KeyCode::Up | KeyCode::Char('k') => {
                                match self.active_column {
                                    ActiveColumn::Sports => self.prev_sport().await,
                                    ActiveColumn::Matches => self.prev_match(),
                                }
                            }
                            _ => {}
                        }
                    }
                }
            } else {
                self.tick();
            }
        }
        Ok(())
    }
}
