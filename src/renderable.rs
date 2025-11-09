use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use crate::qcm::*;

pub trait Renderable {
    fn render(&self, area: Rect, buf: &mut Buffer);
}

impl Renderable for ChoiceQuestion {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let mut lines = vec![Line::from(vec![
            Span::styled("Q: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(&self.prompt),
        ])];

        let is_correct = self.is_correct();
        // let question_color = if is_correct { Color::Green } else { Color::Red };
        let question_color = Color::White;

        for (i, option) in self.options.iter().enumerate() {
            let mut answer_color = Style::default();

            if self.is_correct_index(i) {
                answer_color = answer_color.bg(Color::Green);
            } else if self.incorrect_answers().contains(&i) {
                answer_color = answer_color.bg(Color::Red);
            }

            // if self.is_correct() {
            //     if let Answer::Single(correct) = &self.correct_answer {
            //         if *correct == i {
            //             style = style.fg(Color::Green);
            //         }
            //     } else if let Answer::Multiple(corrects) = &self.correct_answer
            //         && corrects.contains(&i)
            //     {
            //     }
            // }

            lines.push(Line::from(vec![
                Span::styled(
                    format!("  {}. ", i + 1),
                    Style::default(), // Style::default().fg(Color::DarkGray),
                ),
                Span::styled(option.clone(), answer_color),
            ]));
        }

        let paragraph = Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Choice Question")
                    .style(question_color),
            )
            .wrap(Wrap { trim: true });

        paragraph.render(area, buf);
    }
}

impl Renderable for FillInTheBlanksQuestion {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let mut text = String::new();

        for (i, seg) in self.segments.iter().enumerate() {
            match seg {
                Segment::Text(t) => text.push_str(t),
                Segment::Blank { user_answer, .. } => {
                    let answer_display = match user_answer {
                        a if !a.is_empty() => format!("[{}]", a),
                        _ => "[_____]".to_string(),
                    };
                    text.push_str(&answer_display);
                }
            }

            if i < self.segments.len() - 1 {
                text.push(' ');
            }
        }

        let paragraph = Paragraph::new(text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Fill in the blanks"),
            )
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        paragraph.render(area, buf);
    }
}

impl Renderable for Question {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        match self {
            Question::SingleChoice(q) | Question::MultipleChoice(q) => q.render(area, buf),
            Question::FillInTheBlanks(q) => q.render(area, buf),
        }
    }
}
