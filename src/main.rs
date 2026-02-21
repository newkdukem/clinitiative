use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::text::Text;
use ratatui::{DefaultTerminal, Frame};

struct Combatant {
    name: String,
    initiative: i32,
    hp: i32,
}

#[derive(Clone, Copy)]
enum Action {
    DealDamage,
    Heal,
    Remove,
    EditName,
    EditInitiative,
}

enum Mode {
    Tracker,
    AddingCombatant,
    SelectingCombatant { action: Action },
    InputValue { action: Action },
}

struct App {
    combatants: Vec<Combatant>,
    current_turn: usize,
    selected_target: usize,
    mode: Mode,
    input_buffer: String,
    should_quit: bool,
}

impl App {
    fn new() -> Self {
        let mut combatants = vec![
            Combatant {
                name: String::from("Frodo"),
                initiative: 8,
                hp: 30,
            },
            Combatant {
                name: String::from("Sam"),
                initiative: 16,
                hp: 35,
            },
            Combatant {
                name: String::from("Gollum"),
                initiative: 11,
                hp: 20,
            },
        ];

        combatants.sort_by(|a, b| b.initiative.cmp(&a.initiative));

        App {
            combatants,
            current_turn: 0,
            selected_target: 0,
            mode: Mode::Tracker,
            input_buffer: String::new(),
            should_quit: false,
        }
    }

    fn handle_key(&mut self, key_event: KeyEvent) {
        match &self.mode {
            Mode::Tracker => self.handle_tracker_key(key_event.code),
            Mode::AddingCombatant => self.handle_adding_combatant_key(key_event.code),
            Mode::SelectingCombatant { action } => {
                let action = *action;
                self.handle_selecting_combatant_key(key_event.code, action)
            }
            Mode::InputValue { action } => {
                let action = *action;
                self.handle_input_value_key(key_event.code, action)
            }
        }
    }

    fn handle_tracker_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Down => {
                self.current_turn = (self.current_turn + 1) % self.combatants.len();
            }
            KeyCode::Up => {
                self.current_turn =
                    (self.current_turn + self.combatants.len() - 1) % self.combatants.len();
            }
            KeyCode::Char('a') => {
                self.mode = Mode::AddingCombatant;
                self.input_buffer = String::new();
            }
            KeyCode::Char('d')
            | KeyCode::Char('h')
            | KeyCode::Char('r')
            | KeyCode::Char('e')
            | KeyCode::Char('i') => {
                let action = match code {
                    KeyCode::Char('d') => Action::DealDamage,
                    KeyCode::Char('h') => Action::Heal,
                    KeyCode::Char('r') => Action::Remove,
                    KeyCode::Char('e') => Action::EditName,
                    KeyCode::Char('i') => Action::EditInitiative,
                    _ => unreachable!(),
                };
                self.mode = Mode::SelectingCombatant { action };
                self.selected_target = 0;
                self.input_buffer = String::new();
            }
            KeyCode::Char('q') => self.should_quit = true,
            _ => {}
        }
    }

    fn handle_adding_combatant_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Enter => {
                self.mode = Mode::Tracker;
                let input: Vec<&str> = self.input_buffer.split(',').map(|s| s.trim()).collect();
                if input.len() == 3 {
                    let name = input[0].trim().to_string();
                    let initiative = input[1].parse::<i32>().unwrap_or(0);
                    let hp = input[2].parse::<i32>().unwrap_or(0);

                    self.combatants.push(Combatant {
                        name,
                        initiative,
                        hp,
                    });
                }
                self.input_buffer = String::new();
            }
            KeyCode::Char(c) => self.input_buffer.push(c),
            KeyCode::Backspace => {
                self.input_buffer.pop();
            }
            KeyCode::Esc => {
                self.mode = Mode::Tracker;
                self.input_buffer = String::new();
            }
            _ => {}
        }
    }

    fn handle_selecting_combatant_key(&mut self, code: KeyCode, action: Action) {
        match code {
            KeyCode::Down => {
                self.selected_target = (self.selected_target + 1) % self.combatants.len();
            }
            KeyCode::Up => {
                self.selected_target =
                    (self.selected_target + self.combatants.len() - 1) % self.combatants.len();
            }
            KeyCode::Enter => match action {
                Action::Remove => {
                    self.combatants.remove(self.selected_target);
                    if self.combatants.is_empty() {
                        self.mode = Mode::AddingCombatant;
                        self.input_buffer = String::new();
                    } else {
                        if self.current_turn >= self.combatants.len() {
                            self.current_turn = self.combatants.len().saturating_sub(1);
                        }
                        self.mode = Mode::Tracker;
                    }
                    self.selected_target = 0;
                }
                _ => {
                    self.mode = Mode::InputValue { action };
                    self.input_buffer = String::new();
                }
            },
            KeyCode::Esc => {
                self.mode = Mode::Tracker;
                self.selected_target = 0;
                self.input_buffer = String::new();
            }
            _ => {}
        }
    }

    fn handle_input_value_key(&mut self, code: KeyCode, action: Action) {
        match code {
            KeyCode::Enter => {
                let value = self.input_buffer.trim().parse::<i32>().unwrap_or(0);
                match action {
                    Action::DealDamage => self.combatants[self.selected_target].hp -= value,
                    Action::Heal => self.combatants[self.selected_target].hp += value,
                    Action::Remove => {
                        self.combatants.remove(self.selected_target);
                    }
                    Action::EditName => {
                        self.combatants[self.selected_target].name = self.input_buffer.clone()
                    }
                    Action::EditInitiative => {
                        self.combatants[self.selected_target].initiative = value
                    }
                }
                self.mode = Mode::Tracker;
                self.selected_target = 0;
                self.input_buffer = String::new();
            }
            KeyCode::Char(c) => self.input_buffer.push(c),
            KeyCode::Backspace => {
                self.input_buffer.pop();
            }
            KeyCode::Esc => {
                self.mode = Mode::Tracker;
                self.selected_target = 0;
                self.input_buffer = String::new();
            }
            _ => {}
        }
    }

    fn render(&self, frame: &mut Frame) {
        match &self.mode {
            Mode::Tracker => {
                let names: String = self
                    .combatants
                    .iter()
                    .enumerate()
                    .map(|(i, c)| {
                        let prefix = if i == self.current_turn { ">" } else { "" };
                        format!("{}{}: {} | {}", prefix, c.name.as_str(), c.initiative, c.hp)
                    })
                    .collect::<Vec<String>>()
                    .join("\n");
                let text = Text::raw(names);
                frame.render_widget(text, frame.area());
            }
            Mode::AddingCombatant => {
                let input_buffer_text =
                    Text::raw(format!("Enter name,initiative, hp: {}", self.input_buffer));
                frame.render_widget(input_buffer_text, frame.area());
            }
            Mode::SelectingCombatant { action } => {
                let title = match action {
                    Action::DealDamage => "Select target for damage",
                    Action::Heal => "Select target to heal",
                    Action::Remove => "Select target to remove",
                    Action::EditName => "Select target to edit name",
                    Action::EditInitiative => "Select target to edit initiative",
                };

                let names: String = self
                    .combatants
                    .iter()
                    .enumerate()
                    .map(|(i, c)| {
                        let prefix = if i == self.selected_target { ">" } else { "" };
                        format!("{}{}", prefix, c.name.as_str())
                    })
                    .collect::<Vec<String>>()
                    .join("\n");
                let text = Text::raw(format!("{}:\n{}", title, names));
                frame.render_widget(text, frame.area());
            }
            Mode::InputValue { action } => {
                // Add prompt text based on the action
                let prompt = match action {
                    Action::DealDamage => "Deal damage: ".to_string(),
                    Action::Heal => "Heal amount: ".to_string(),
                    Action::EditInitiative => "New initiative: ".to_string(),
                    Action::EditName => "New name: ".to_string(),
                    Action::Remove => unreachable!(),
                };
                let text = Text::raw(format!("{}{}", prompt, self.input_buffer));
                frame.render_widget(text, frame.area());
            }
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
            app.render(frame);
        })?;

        if let Event::Key(key) = event::read()? {
            app.handle_key(key);
        }
        if app.should_quit {
            break Ok(());
        }
    }
}
