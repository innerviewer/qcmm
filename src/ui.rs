use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    style::{Color, Style},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Widget, Wrap},
};

pub fn render_main_screen(area: Rect, buf: &mut Buffer) {
    let title = Line::from(" QCM Manager ".bold());
    let instructions = Line::from(vec![
        " Create Test ".into(),
        "<C>".blue().bold(),
        " Quit ".into(),
        "<Q> ".blue().bold(),
    ]);
    let block = Block::bordered()
        .title_bottom(instructions.centered())
        .title_top(title)
        .border_set(border::ROUNDED);

    let counter_text = Text::from(vec![Line::from(vec!["Main Menu".into()])]);

    Paragraph::new(counter_text)
        .centered()
        .block(block)
        .render(area, buf);
}

pub fn render_choosing_screen(area: Rect, buf: &mut Buffer) {}
pub fn render_creating_screen(area: Rect, buf: &mut Buffer) {}
pub fn render_answering_screen(area: Rect, buf: &mut Buffer) {}
pub fn render_exiting_screen(area: Rect, buf: &mut Buffer) {}
