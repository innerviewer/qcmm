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
use crate::app::CurrentScreen;
use crate::renderable::*;

pub fn construct_base(app: &App) -> Block<'_> {
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

    Block::default()
        .borders(Borders::ALL)
        .title_bottom(instructions.centered())
        .title_top(title)
        .title(Line::from(screen_title).centered().yellow())
        .border_style(Style::default().fg(Color::White))
        .border_type(ratatui::widgets::BorderType::Rounded)
}

pub fn render_main_screen(area: Rect, buf: &mut Buffer, app: &App) {
    construct_base(app).render(area, buf);
}

pub fn render_choosing_screen(area: Rect, buf: &mut Buffer, app: &App) {}
pub fn render_creating_screen(area: Rect, buf: &mut Buffer, app: &App) {}
pub fn render_answering_screen(area: Rect, buf: &mut Buffer, app: &App) {
    let base = construct_base(app);
    let inner = base.inner(area);

    base.render(area, buf);

    Paragraph::new("Type your QCM questions...")
        .alignment(Alignment::Center)
        .render(inner, buf);

    let mut y = inner.y;
    if let CurrentScreen::Answering(answering_state) = &app.current_screen {
        for question in &answering_state.qcm.questions {
            let question_area = Rect {
                x: inner.x,
                y,
                width: inner.width,
                height: 6, // adjust per question type
            };
            question.render(question_area, buf);
            y += 7; // spacing
        }
    }
}
pub fn render_exiting_screen(area: Rect, buf: &mut Buffer, app: &App) {}
