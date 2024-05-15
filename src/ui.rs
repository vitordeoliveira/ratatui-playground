use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Stylize,
    text::Line,
    widgets::{block::Title, Block, Paragraph},
    Frame,
};

use crate::App;

pub fn ui(f: &mut Frame, state: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(state.left_size), Constraint::Min(1)])
        .split(f.size());

    let instructions = Title::from(Line::from(vec![
        " Decrement Menu Size".into(),
        "<Left>".blue().bold(),
        " Increment Menu Size".into(),
        "<Right>".blue().bold(),
        " Quit ".into(),
        "<Q> ".blue().bold(),
    ]));
    let block = Block::default().title(instructions);
    f.render_widget(block, chunks[0]);
    let text = format!("size: {}", state.left_size);
    f.render_widget(Paragraph::new(text).white().on_blue(), chunks[1]);
}
