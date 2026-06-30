use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
// use crossterm::style::Stylize;
use ratatui::text::Line;
use ratatui::style::Stylize;
use ratatui::widgets::{Paragraph, Widget};
use ratatui::{DefaultTerminal, Frame, Terminal};
use serde_json::json;
use std::fs::File;
use std::io::{self, BufReader};
use std::error::Error;
use std::path::Path;

#[derive(Default)]
struct App {
    exit: bool
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;

        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_events(key_event);
            }

            _ => {}
        }
        Ok(())
    }

    fn handle_key_events(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,
            _ => {}
        }
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let title = Line::from(" JSON reader app ".bold());

        Paragraph::new(title).render(area, buf);
        
    }
    
}

fn main() -> std::io::Result<()> {
    // let path = Path::new("./some.json");
    // let file = File::open(path)?;
    // let file_buff = BufReader::new(file);
    ratatui::run(|terminal| App::default().run(terminal))
}