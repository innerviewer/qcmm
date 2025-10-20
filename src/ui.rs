use crate::app::{App, CurrentScreen, CurrentlyAnswering};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
};

pub fn ui(frame: &mut Frame, app: &App) {}
