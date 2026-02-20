use ratatui::{DefaultTerminal, Frame};
use ratatui::text::Text;

struct Combatant {
    name: String,
    initiative: i32,
}

struct App {
    combatants: Vec<Combatant>,
}

impl App {
    fn new() -> Self {
        App {
            combatants: vec![
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
            ],
        }
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let app = App::new();
    loop {
        terminal.draw(|frame| {
            // capture app by reference
            let app = &app;
            render(app, frame);
        })?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(app: &App, frame: &mut Frame) {
    let names: String = app
        .combatants
        .iter()
        .map(|c| format!("{}: {}",c.name.as_str(), c.initiative))
        .collect::<Vec<String>>()
        .join("\n");
    let text = Text::raw(names);

    frame.render_widget(text, frame.area());
}
