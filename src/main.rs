use crossterm::event::{self, Event, KeyCode};
use ratatui::text::Text;
use ratatui::{DefaultTerminal, Frame};

struct Combatant {
    name: String,
    initiative: i32,
}

enum Mode {
    Tracker,
    AddingCombatant,
}

struct App {
    combatants: Vec<Combatant>,
    current_turn: usize,
    mode: Mode,
    input_buffer: String,
}

impl App {
    fn new() -> Self {
        let mut combatants = vec![
            Combatant {
                name: String::from("Frodo"),
                initiative: 8,
            },
            Combatant {
                name: String::from("Sam"),
                initiative: 16,
            },
            Combatant {
                name: String::from("Gollum"),
                initiative: 11,
            },
        ];

        combatants.sort_by(|a, b| b.initiative.cmp(&a.initiative));

        App {
            combatants: combatants,
            current_turn: 0,
            mode: Mode::Tracker,
            input_buffer: String::new(),
        }
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut app = App::new();
    loop {
        terminal.draw(|frame| {
            // capture app by reference
            let app = &app;
            render(app, frame);
        })?;
        match (&app.mode, event::read()?) {
            (Mode::Tracker, Event::Key(key_event)) => match key_event.code {
                KeyCode::Down => {
                    app.current_turn = (app.current_turn + 1) % app.combatants.len();
                }
                KeyCode::Up => {
                    app.current_turn =
                        (app.current_turn + app.combatants.len() - 1) % app.combatants.len();
                }
                KeyCode::Char('a') => {
                    app.mode = Mode::AddingCombatant;
                    app.input_buffer = String::new();
                }
                KeyCode::Char('q') => break Ok(()),
                _ => {}
            },
            (Mode::AddingCombatant, Event::Key(key_event)) => match key_event.code {
                KeyCode::Enter => {
                    app.mode = Mode::Tracker;
                    if let Some((name, initiative)) = app.input_buffer.split_once(',') {
                        let initiative = initiative.trim().parse::<i32>().unwrap_or(0);
                        app.combatants.push(Combatant {
                            name: name.trim().to_string(),
                            initiative,
                        });
                        app.combatants
                            .sort_by(|a, b| b.initiative.cmp(&a.initiative));
                    }
                    app.input_buffer = String::new();
                }
                KeyCode::Esc => {
                    app.mode = Mode::Tracker;
                    app.input_buffer = String::new();
                }
                KeyCode::Backspace => {
                    app.input_buffer.pop();
                }
                KeyCode::Char(c) => app.input_buffer.push(c),
                _ => {}
            },
            _ => {}
        }
    }
}

fn render(app: &App, frame: &mut Frame) {
    match app.mode {
        Mode::Tracker => {
            let names: String = app
                .combatants
                .iter()
                .enumerate()
                .map(|(i, c)| {
                    let prefix = if i == app.current_turn { ">> " } else { "" };
                    format!("{}{}: {}", prefix, c.name.as_str(), c.initiative)
                })
                .collect::<Vec<String>>()
                .join("\n");
            let text = Text::raw(names);
            frame.render_widget(text, frame.area());
        }
        Mode::AddingCombatant => {
            let input_buffer_text = Text::raw(format!("Enter name,initiative: {}", app.input_buffer.as_str()));
            frame.render_widget(input_buffer_text, frame.area());
        }
    }
}
