use crate::app::{App, InputMode};

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

// generate ui frame
pub fn ui<B: Backend>(
    f: &mut Frame<B>,
    app: &mut App,
    hisroty_commands: &Vec<&str>,
    history_state: &mut ListState,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::styled(
                    " q ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(": Exit "),
                Span::styled(
                    " e ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(": Input "),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::styled(
                    " Enter ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(": stop input "),
            ],
            Style::default(),
        ),
    };

    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(app.input.as_ref())
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .style(match app.input_mode {
                    InputMode::Normal => Style::default().fg(Color::White),
                    InputMode::Editing => Style::default().fg(Color::Magenta),
                })
                .title("Input"),
        );
    f.render_widget(input, chunks[1]);

    match app.input_mode {
        InputMode::Normal => {}

        InputMode::Editing => {
            f.set_cursor(chunks[1].x + app.input.len() as u16 + 1, chunks[1].y + 1)
        }
    }

    let history_list = render_history(app, hisroty_commands);

    f.render_stateful_widget(history_list, chunks[2], history_state);
}

// generate history list
fn render_history<'a>(app: &mut App, hisroty_commands: &Vec<&str>) -> List<'a> {
    let history_block = Block::default()
        .borders(Borders::all())
        .style(match app.input_mode {
            InputMode::Normal => Style::default().fg(Color::Magenta),
            InputMode::Editing => Style::default().fg(Color::White),
        })
        .title("History Command")
        .border_type(BorderType::Double);

    let cmd_list: Vec<_> = hisroty_commands
        .iter()
        .map(|line_content| {
            ListItem::new(Spans::from(vec![Span::styled(
                line_content.to_string(),
                Style::default(),
            )]))
        })
        .collect();

    let history_list = List::new(cmd_list)
        .block(history_block)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );

    history_list
}
