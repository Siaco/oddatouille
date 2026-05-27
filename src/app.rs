use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{backend::Backend, Terminal};
use std::time::Duration;

use crate::ui;

#[derive(PartialEq)]
pub enum ActiveColumn {
    Sports,
    Matches,
}

pub struct OddMock {
    pub name: String,
    pub price: f64,
}

pub struct MatchMock {
    pub title: String,
    pub odds: Vec<OddMock>,
    pub prediction: String,
}

pub struct SportMock {
    pub name: String,
    pub matches: Vec<MatchMock>,
}

/// Application state
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub active_column: ActiveColumn,
    
    // Mock Data
    pub sports: Vec<SportMock>,
    
    // Selection state
    pub selected_sport: usize,
    pub selected_match: usize,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        // Build mock data
        let sports = vec![
            SportMock {
                name: "Calcio".into(),
                matches: vec![
                    MatchMock {
                        title: "Vincente Serie A 24/25".into(),
                        odds: vec![
                            OddMock { name: "Inter".into(), price: 1.85 },
                            OddMock { name: "Juventus".into(), price: 3.50 },
                            OddMock { name: "Napoli".into(), price: 4.00 },
                        ],
                        prediction: "L'Inter ha mantenuto l'ossatura della squadra campione, rendendola la favorita statistica.".into(),
                    },
                    MatchMock {
                        title: "Vincente Champions League".into(),
                        odds: vec![
                            OddMock { name: "Real Madrid".into(), price: 3.00 },
                            OddMock { name: "Man City".into(), price: 3.25 },
                            OddMock { name: "Bayern".into(), price: 6.50 },
                        ],
                        prediction: "Con l'aggiunta di Mbappé, il Real Madrid domina i modelli predittivi.".into(),
                    }
                ]
            },
            SportMock {
                name: "Basket (NBA)".into(),
                matches: vec![
                    MatchMock {
                        title: "NBA Championship Winner".into(),
                        odds: vec![
                            OddMock { name: "Boston Celtics".into(), price: 4.00 },
                            OddMock { name: "Denver Nuggets".into(), price: 5.50 },
                            OddMock { name: "OKC Thunder".into(), price: 8.00 },
                        ],
                        prediction: "I Celtics vantano il miglior net rating proiettato per la stagione.".into(),
                    }
                ]
            },
            SportMock {
                name: "Tennis".into(),
                matches: vec![
                    MatchMock {
                        title: "Wimbledon 2025 Men".into(),
                        odds: vec![
                            OddMock { name: "C. Alcaraz".into(), price: 2.20 },
                            OddMock { name: "J. Sinner".into(), price: 2.75 },
                            OddMock { name: "N. Djokovic".into(), price: 4.50 },
                        ],
                        prediction: "L'algoritmo rileva un vantaggio marginale di Alcaraz sull'erba rispetto a Sinner.".into(),
                    }
                ]
            }
        ];

        Self { 
            running: true,
            active_column: ActiveColumn::Sports,
            sports,
            selected_sport: 0,
            selected_match: 0,
        }
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn next_sport(&mut self) {
        if self.selected_sport + 1 < self.sports.len() {
            self.selected_sport += 1;
            self.selected_match = 0; // Reset match selection when sport changes
        }
    }

    pub fn prev_sport(&mut self) {
        if self.selected_sport > 0 {
            self.selected_sport -= 1;
            self.selected_match = 0;
        }
    }

    pub fn next_match(&mut self) {
        if let Some(sport) = self.sports.get(self.selected_sport) {
            if self.selected_match + 1 < sport.matches.len() {
                self.selected_match += 1;
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

    /// Run the application loop
    pub async fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        while self.running {
            terminal.draw(|frame| ui::render(self, frame))?;

            if event::poll(Duration::from_millis(250))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => self.quit(),
                            KeyCode::Tab | KeyCode::Right | KeyCode::Left => self.toggle_column(),
                            KeyCode::Down | KeyCode::Char('j') => {
                                match self.active_column {
                                    ActiveColumn::Sports => self.next_sport(),
                                    ActiveColumn::Matches => self.next_match(),
                                }
                            }
                            KeyCode::Up | KeyCode::Char('k') => {
                                match self.active_column {
                                    ActiveColumn::Sports => self.prev_sport(),
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
