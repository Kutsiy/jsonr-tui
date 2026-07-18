use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{self, Alignment, Constraint, Direction, Flex, Layout};
use ratatui::symbols::border;
// use crossterm::style::Stylize;
use ratatui::text::{Line, Span, Text, ToText};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, List, ListItem, ListState, Paragraph, Widget};
use ratatui::{DefaultTerminal, Frame, Terminal};
use serde_json::json;
use std::fs::File;
use std::io::{self, BufReader};
use std::error::Error;
use std::path::Path;

struct MenuItem {
    name: String
}

#[derive(Default)]
struct App {
    exit: bool,
    json_text: serde_json::Value,
    menu: Vec<MenuItem>,
    menu_index: ListState
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
        let layout = Layout::default().direction(Direction::Vertical).constraints([Constraint::Percentage(100)]).margin(1).split(frame.area());

        frame.render_widget(self, layout[0]);
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
            KeyCode::Up => {
             
            },

            _ => {}
        }
    }

}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let title_menu = Line::from(" Menu ".bold());
        let title_main = Line::from(" JSON reader app ".bold());

        let controls_menu = Line::from(" < Press Up/Down arrows to move > ");
        let controls_main = Line::from(" < Press Q to leave > ").centered();

        let block_menu = Block::bordered()
            .title(title_menu.centered())
            .title_bottom(controls_menu)
            .border_set(border::THICK).border_style(Style::default().fg(Color::Blue));

        let block_main = Block::bordered()
            .title(title_main.centered())    
            .title_bottom(controls_main)
            .border_set(border::THICK).border_style(Style::default().fg(Color::Blue));

        let layout = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Length(40), Constraint::Percentage(100)]).split(area);

        let menu = List::new(self.menu.iter().map(|item| { ListItem::from(Line::from(item.name.as_str()).centered()) })).block(block_menu);

        menu.render(layout[0], buf);
    
        Paragraph::new("Text2").block(block_main).render(layout[1], buf);
    }
    
}

fn main() -> std::io::Result<()> {
    // let path = Path::new("src/some.json");
    // let file = File::open(path)?;
    // let file_buff = BufReader::new(file);
    // let json_data: serde_json::Value = serde_json::from_reader(file_buff)?;

    let mut app = App {exit: false, menu: vec![MenuItem{name: String::from("Create")}, MenuItem{name: String::from("Update")} ], ..Default::default()};

    ratatui::run(|terminal| app.run(terminal))
}