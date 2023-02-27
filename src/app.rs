use crate::ui::ui;

use crossterm::event::{self, Event, KeyCode};
use itertools::Itertools;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::{env, io};
use tui::{backend::Backend, widgets::ListState, Terminal};

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

// Zsh uses a meta char (0x83) to signify that the previous character should be ^ 32.
fn read_and_unmetafy(path: &Path) -> String {
    let mut f = File::open(path).unwrap_or_else(|_| panic!("{:?} file not found", &path));
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)
        .unwrap_or_else(|_| panic!("unable to read from {:?}", &path));
    for index in (0..buffer.len()).rev() {
        if buffer[index] == 0x83 {
            buffer.remove(index);
            buffer[index] ^= 32;
        }
    }
    String::from_utf8_lossy(&buffer).to_string()
}

// start run app
pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    let zsh_history_filename = env::var("HISTFILE").unwrap_or("/root/.zsh_history".to_string());

    let history_content = read_and_unmetafy(Path::new(zsh_history_filename.as_str()));
    let history: Vec<&str> = history_content.lines().collect();

    let mut history_state = ListState::default();
    history_state.select(Some(0));

    let mut input_str = String::new();

    loop {
        let hisroty_commands: Vec<&str> = history
            .iter()
            .rev()
            .filter(|line_content| {
                !line_content.is_empty() && {
                    let mut is_match = true;
                    for (_, item) in input_str.split("&").enumerate() {
                        if !line_content.contains(item) {
                            is_match = false;
                            break;
                        }
                    }
                    is_match
                }
            })
            .map(|line_content| {
                let line_command = line_content.split(";").last().unwrap();
                line_command
            })
            .unique()
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
