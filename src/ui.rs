use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Stylize,
    style::{Color, Style},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Widget, Wrap},
};

use crate::app::App;

pub fn render_base(area: Rect, buf: &mut Buffer, app: &App) {
    let title = Line::from(" QCM Manager ".bold());
    let instructions = Line::from(vec![
        " Create QCM ".into(),
        "<C>".blue().bold(),
        " Test QCM ".into(),
        "<T>".blue().bold(),
        " Quit ".into(),
        "<Q> ".blue().bold(),
    ]);

    let screen_title = format!(" {} Screen ", app.current_screen);

    let block = Block::default()
        .borders(Borders::ALL)
        .title_bottom(instructions.centered())
        .title_top(title)
        .title(Line::from(screen_title).centered().yellow())
        .border_style(Style::default().fg(Color::White))
        .border_type(ratatui::widgets::BorderType::Rounded);

    block.render(area, buf);
}

pub fn render_main_screen(area: Rect, buf: &mut Buffer, app: &App) {
    render_base(area, buf, app);
}

pub fn render_choosing_screen(area: Rect, buf: &mut Buffer, app: &App) {}
pub fn render_creating_screen(area: Rect, buf: &mut Buffer, app: &App) {}
pub fn render_answering_screen(area: Rect, buf: &mut Buffer, app: &App) {
    render_base(area, buf, app);
}
pub fn render_exiting_screen(area: Rect, buf: &mut Buffer, app: &App) {}
