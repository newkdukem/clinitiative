use crossterm::event::{self, Event, KeyCode};
use ratatui::text::Text;
use ratatui::{DefaultTerminal, Frame};

struct Combatant {
    name: String,
    initiative: i32,
}

struct App {
    combatants: Vec<Combatant>,
    current_turn: usize,
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
        match event::read()? {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Down => {
                    app.current_turn = (app.current_turn + 1) % app.combatants.len();
                }
                KeyCode::Up => {
                    app.current_turn =
                        (app.current_turn + app.combatants.len() - 1) % app.combatants.len();
                }
                KeyCode::Char('q') => break Ok(()),
                _ => {}
            },
            _ => {}
        }
    }
}

fn render(app: &App, frame: &mut Frame) {
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
