use crossterm::event::KeyCode::Down;
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

    fn draw(&mut self, frame: &mut Frame) {
        let layout = Layout::default().direction(Direction::Vertical).constraints([Constraint::Percentage(100)]).margin(1).split(frame.area());

        let title_menu = Line::from(" Menu ".bold());
        let title_main = Line::from(" JSON reader app ".bold());

        let controls_menu = Line::from(" < Press Up/Down arrows to move > ");
        let controls_main = Line::from(" < Press Q to leave > ").centered();

        let block_menu = Block::bordered()
            .title(title_menu.centered())
            .title_bottom(controls_menu)
            .border_set(border::THICK).border_style(Style::default().fg(Color::Green));

        let block_main = Block::bordered()
            .title(title_main.centered())    
            .title_bottom(controls_main)
            .border_set(border::THICK).border_style(Style::default().fg(Color::Green));

        let [menu_area, main_area] = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Length(40), Constraint::Percentage(100)]).areas(layout[0]);

        let menu = List::new(self.menu.iter().map(|item| { ListItem::from(Line::from(item.name.as_str()).centered()) })).highlight_style(Style::default().fg(Color::LightGreen)).block(block_menu);

        frame.render_stateful_widget(menu, menu_area, &mut self.menu_index);
    
        let main = Paragraph::new("Text2").block(block_main);

        frame.render_widget(main, main_area);

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
            KeyCode::Esc => self.exit = true,
            KeyCode::Up => {
                self.menu_index.select_previous();
            },
            KeyCode::Down => {
                self.menu_index.select_next();
            }

            _ => {}
        }
    }

}


fn main() -> std::io::Result<()> {
    // let path = Path::new("src/some.json");
    // let file = File::open(path)?;
    // let file_buff = BufReader::new(file);
    // let json_data: serde_json::Value = serde_json::from_reader(file_buff)?;

    let mut app = App {exit: false, menu: vec![MenuItem{name: String::from("Create")}, MenuItem{name: String::from("Update")} ], ..Default::default()};
    app.menu_index.select(Some(0));
    ratatui::run(|terminal| app.run(terminal))
}