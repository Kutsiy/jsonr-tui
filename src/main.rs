use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::symbols::border;
// use crossterm::style::Stylize;
use ratatui::text::{Line, Text, ToText};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Paragraph, Widget};
use ratatui::{DefaultTerminal, Frame, Terminal};
use serde_json::json;
use std::fs::File;
use std::io::{self, BufReader};
use std::error::Error;
use std::path::Path;

#[derive(Default)]
struct App {
    exit: bool,
    json_text: serde_json::Value
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

    fn render_json(&self) -> Block<'static> {
        Block::bordered().title(Line::from(self.json_text.to_string())).border_set(border::EMPTY)
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
        let bottom_title = Line::from(" Press 'Q' to exit ".bold());
        let inner_block = self.render_json();
        let outer_block = Block::bordered()
            .title(title.centered())
            .title_bottom(bottom_title.centered())
            .border_set(border::THICK);

        let outer_area = outer_block.inner(area);

        outer_block.render(area, buf);

        Paragraph::new("Title").block(inner_block).render(outer_area, buf);

    }
    
}

fn main() -> std::io::Result<()> {
    let path = Path::new("src/some.json");
    let file = File::open(path)?;
    let file_buff = BufReader::new(file);
    let json_data: serde_json::Value = serde_json::from_reader(file_buff)?;

    let mut app = App {exit: false, json_text: json_data};

    ratatui::run(|terminal| app.run(terminal))
}