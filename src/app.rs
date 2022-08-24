use crate::ui::ui;

use crossterm::event::{self, Event, KeyCode};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, io};
use tui::{backend::Backend, widgets::ListState, Terminal};

use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;

pub enum InputMode {
    Normal,
    Editing,
}

/// App holds the state of the application
pub struct App {
    /// Current value of the input box
    pub input: String,
    /// Current input mode
    pub input_mode: InputMode,
    /// Selected command from history
    pub selected_cmd: String,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            selected_cmd: String::new(),
        }
    }
}

// start run app
pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    let zsh_history_filename = env::var("HISTFILE").unwrap_or("/root/.zsh_history".to_string());

    let history_file = File::open(zsh_history_filename)?;
    let history: Vec<String> = BufReader::new(
        DecodeReaderBytesBuilder::new()
            .encoding(Some(WINDOWS_1252))
            .build(history_file),
    )
    .lines()
    .map(|h| h.unwrap())
    .collect();

    let mut history_state = ListState::default();
    history_state.select(Some(0));

    let mut input_str = String::new();

    loop {
        let hisroty_commands: Vec<&str> = history
            .iter()
            .rev()
            .filter(|line_content| input_str.is_empty() || line_content.contains(&input_str))
            .map(|line_content| {
                let line_command = line_content.split(";").last().unwrap();
                line_command
            })
            .collect();

        terminal.draw(|f| ui(f, app, &hisroty_commands, &mut history_state))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Down => {
                        if let Some(current) = history_state.selected() {
                            let history_list_count = hisroty_commands.len();
                            if current >= history_list_count - 1 {
                                history_state.select(Some(0));
                            } else {
                                history_state.select(Some(current + 1));
                            }
                        }
                    }
                    KeyCode::Up => {
                        if let Some(current) = history_state.selected() {
                            let history_list_count = hisroty_commands.len();
                            if current > 0 {
                                history_state.select(Some(current - 1));
                            } else {
                                history_state.select(Some(history_list_count - 1));
                            }
                        }
                    }
                    KeyCode::Enter => {
                        let selected_command = hisroty_commands
                            .get(history_state.selected().unwrap())
                            .unwrap_or(&"");

                        app.selected_cmd.push_str(selected_command);

                        return Ok(());
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Char(c) => {
                        app.input.push(c);
                        input_str = app.input.to_owned();
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                        input_str = app.input.to_owned();
                    }
                    KeyCode::Enter => {
                        app.input_mode = InputMode::Normal;
                        input_str = app.input.to_owned();
                        app.input.clear();
                    }
                    _ => {}
                },
            }
        }
    }
}
