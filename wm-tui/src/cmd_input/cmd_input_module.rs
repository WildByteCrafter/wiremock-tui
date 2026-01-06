use crate::contract::contract_module::{Command, Module};
use crate::contract::contract_processing::ProcessingResult::NothingDone;
use crate::contract::contract_processing::{ProcessingResult, ProcessingResultPayload};
use crate::contract::contract_trigger::{ActiveForMode, CommandTrigger, CommandTriggerPayload};
use color_eyre::Report;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, TableState};
use ratatui::Frame;

#[derive(Clone, Debug)]
pub enum CmdInputCommands {
    RegisterCommandTriggers {
        command_trigger_payload: CommandTriggerPayload,
    },
    ProcessInput {
        input: KeyEvent,
    },
}

#[derive(Clone, Debug, PartialEq)]
enum InputMode {
    Navigation,
    AdvancedCommand,
}

#[derive(Clone, Debug)]
pub struct CmdInputModule {
    command_triggers: Vec<CommandTrigger>,
    mode: InputMode,
    input_buffer: Vec<KeyEvent>,
    show_help: bool,
    help_selected_index: usize,
}

impl CmdInputModule {
    pub fn new() -> Self {
        Self {
            command_triggers: Vec::new(),
            mode: InputMode::Navigation,
            input_buffer: Vec::new(),
            show_help: false,
            help_selected_index: 0,
        }
    }
}

const MODULE_NAME: &'static str = "cmd_input";

impl Module for CmdInputModule {
    fn name(&self) -> &'static str {
        MODULE_NAME
    }

    fn can_process_command(&self, command: &Command) -> bool {
        match command {
            Command::CmdInputModule(_) => true,
            _ => false,
        }
    }

    fn process_command(&mut self, command: Command) -> Result<ProcessingResult, Report> {
        if !self.can_process_command(&command) {
            return Ok(NothingDone);
        }
        match command {
            Command::CmdInputModule(CmdInputCommands::RegisterCommandTriggers {
                command_trigger_payload,
            }) => self.handle_register_command_triggers(command_trigger_payload),
            Command::CmdInputModule(CmdInputCommands::ProcessInput { input }) => {
                self.handle_process_input(input)
            }
            _ => Ok(NothingDone),
        }
    }

    fn render(&self, frame: &mut Frame, rect: Rect) {
        self.render_command_input(frame, rect);
        self.render_help_modal(frame);
    }
}

impl CmdInputModule {
    fn render_command_input(&self, frame: &mut Frame, rect: Rect) {
        if self.mode == InputMode::AdvancedCommand {
            let input_string = self.get_input_string();
            let display_text = format!(":{}", input_string);
            let paragraph = Paragraph::new(display_text).style(Style::default().fg(Color::Yellow));

            frame.render_widget(paragraph, rect);
        }
    }

    fn render_help_modal(&self, frame: &mut Frame) {
        if self.show_help {
            let area = centered_rect(60, 60, frame.area());
            frame.render_widget(Clear, area);

            let header_cells = ["Module", "Command", "Triggers"]
                .iter()
                .map(|h| Cell::from(*h).style(Style::default().add_modifier(Modifier::BOLD)));
            let header = Row::new(header_cells)
                .style(Style::default().bg(Color::Blue))
                .height(1);

            let rows = self.command_triggers.iter().map(|t| {
                let triggers_str = self.format_triggers(&t.triggers);

                Row::new(vec![
                    Cell::from(t.module_name),
                    Cell::from(t.command_name),
                    Cell::from(triggers_str),
                ])
            });

            let mut table_state = TableState::default();
            table_state.select(Some(self.help_selected_index));

            let table = Table::new(
                rows,
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(40),
                    Constraint::Percentage(40),
                ],
            )
            .header(header)
            .block(Block::default().borders(Borders::ALL).title("Help"))
            .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED));

            frame.render_stateful_widget(table, area, &mut table_state);
        }
    }

    fn get_input_string(&self) -> String {
        self.input_buffer
            .iter()
            .filter_map(|key_event| {
                if let KeyCode::Char(c) = key_event.code {
                    Some(c)
                } else {
                    None
                }
            })
            .collect()
    }

    fn format_triggers(&self, triggers: &[KeyEvent]) -> String {
        triggers
            .iter()
            .map(|ev| match ev.code {
                KeyCode::Char(c) => c.to_string(),
                KeyCode::Enter => "Enter".to_string(),
                KeyCode::Esc => "Esc".to_string(),
                KeyCode::Backspace => "Backspace".to_string(),
                _ => format!("{:?}", ev.code),
            })
            .collect::<Vec<_>>()
            .join("")
    }
    fn handle_register_command_triggers(
        &mut self,
        payload: CommandTriggerPayload,
    ) -> Result<ProcessingResult, Report> {
        let module_name = payload.module_name;
        self.command_triggers
            .retain(|t| t.module_name != module_name);

        for mut trigger in payload.command_triggers {
            trigger.module_name = module_name;
            self.command_triggers.push(trigger);
        }

        Ok(ProcessingResult::Processed(ProcessingResultPayload::new()))
    }

    fn handle_process_input(&mut self, input: KeyEvent) -> Result<ProcessingResult, Report> {
        let mut result_payload = ProcessingResultPayload::new();

        if self.show_help {
            self.handle_help_navigation(input);
            return Ok(ProcessingResult::Processed(result_payload));
        }

        match self.mode {
            InputMode::Navigation => self.handle_navigation_mode(input, &mut result_payload),
            InputMode::AdvancedCommand => {
                self.handle_advanced_command_mode(input, &mut result_payload)
            }
        }
        Ok(ProcessingResult::Processed(result_payload))
    }

    fn handle_help_navigation(&mut self, input: KeyEvent) {
        if self.command_triggers.is_empty() {
            if let KeyCode::Esc = input.code {
                self.show_help = false;
            }
            return;
        }

        match input.code {
            KeyCode::Char('j') => {
                self.help_selected_index = (self.help_selected_index + 1) % self.command_triggers.len();
            }
            KeyCode::Char('k') => {
                if self.help_selected_index == 0 {
                    self.help_selected_index = self.command_triggers.len() - 1;
                } else {
                    self.help_selected_index -= 1;
                }
            }
            KeyCode::Esc => {
                self.show_help = false;
            }
            _ => {}
        }
    }

    fn handle_navigation_mode(&mut self, input: KeyEvent, result_payload: &mut ProcessingResultPayload) {
        if let KeyCode::Char(':') = input.code {
            self.mode = InputMode::AdvancedCommand;
            self.input_buffer.clear();
        } else {
            let triggers = self.command_triggers.iter().filter(|t| {
                matches!(
                    t.active_for_mode,
                    ActiveForMode::Navigation
                )
            });

            for trigger in triggers {
                if trigger.triggers.len() == 1 && trigger.triggers[0] == input {
                    let old_payload = std::mem::replace(result_payload, ProcessingResultPayload::new());
                    *result_payload = old_payload.with_command(trigger.command.clone());
                }
            }
        }
    }

    fn handle_advanced_command_mode(
        &mut self,
        input: KeyEvent,
        result_payload: &mut ProcessingResultPayload,
    ) {
        match input.code {
            KeyCode::Enter => {
                let input_string = self.get_input_string();

                if input_string == "help" {
                    self.show_help = true;
                    self.mode = InputMode::Navigation;
                    self.input_buffer.clear();
                    return;
                }

                let matching_trigger = self.command_triggers.iter().find(|t| {
                    matches!(t.active_for_mode, ActiveForMode::AdvancedCommand)
                        && t.triggers == self.input_buffer
                });

                if let Some(trigger) = matching_trigger {
                    let old_payload =
                        std::mem::replace(result_payload, ProcessingResultPayload::new());
                    *result_payload = old_payload.with_command(trigger.command.clone());
                    self.mode = InputMode::Navigation;
                    self.input_buffer.clear();
                } else {
                    // Reset on invalid command instead of panicking
                    self.mode = InputMode::Navigation;
                    self.input_buffer.clear();
                }
            }
            KeyCode::Backspace => {
                self.input_buffer.pop();
            }
            KeyCode::Esc => {
                self.mode = InputMode::Navigation;
                self.input_buffer.clear();
            }
            _ => {
                self.input_buffer.push(input);
            }
        }
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract::contract_trigger::{ActiveForMode, CommandTrigger};
    use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

    #[test]
    fn test_register_command_triggers_replaces_existing_from_same_module() {
        let mut module = CmdInputModule::new();
        let module_name = "test_module";

        let trigger1 = CommandTrigger {
            module_name,
            command_name: "cmd1",
            active_for_mode: ActiveForMode::Navigation,
            triggers: vec![KeyEvent {
                code: KeyCode::Char('a'),
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            }],
            command: Command::CmdInputModule(CmdInputCommands::ProcessInput {
                input: KeyEvent {
                    code: KeyCode::Char('a'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                },
            }),
        };

        let payload1 = CommandTriggerPayload {
            module_name,
            command_triggers: vec![trigger1],
        };

        module
            .process_command(Command::CmdInputModule(
                CmdInputCommands::RegisterCommandTriggers {
                    command_trigger_payload: payload1,
                },
            ))
            .unwrap();

        assert_eq!(module.command_triggers.len(), 1);
        assert_eq!(module.command_triggers[0].command_name, "cmd1");

        let trigger2 = CommandTrigger {
            module_name: "other_module", // should be overwritten by payload.module_name
            command_name: "cmd2",
            active_for_mode: ActiveForMode::Navigation,
            triggers: vec![KeyEvent {
                code: KeyCode::Char('b'),
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            }],
            command: Command::CmdInputModule(CmdInputCommands::ProcessInput {
                input: KeyEvent {
                    code: KeyCode::Char('b'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                },
            }),
        };

        let payload2 = CommandTriggerPayload {
            module_name,
            command_triggers: vec![trigger2],
        };

        module
            .process_command(Command::CmdInputModule(
                CmdInputCommands::RegisterCommandTriggers {
                    command_trigger_payload: payload2,
                },
            ))
            .unwrap();

        assert_eq!(module.command_triggers.len(), 1);
        assert_eq!(module.command_triggers[0].command_name, "cmd2");
        assert_eq!(module.command_triggers[0].module_name, module_name);
    }

    #[test]
    fn test_register_command_triggers_appends_for_different_modules() {
        let mut module = CmdInputModule::new();

        let payload1 = CommandTriggerPayload {
            module_name: "module1",
            command_triggers: vec![CommandTrigger {
                module_name: "module1",
                command_name: "cmd1",
                active_for_mode: ActiveForMode::Navigation,
                triggers: vec![],
                command: Command::CmdInputModule(CmdInputCommands::ProcessInput {
                    input: KeyEvent::new(KeyCode::Char('1'), KeyModifiers::NONE),
                }),
            }],
        };

        module
            .process_command(Command::CmdInputModule(
                CmdInputCommands::RegisterCommandTriggers {
                    command_trigger_payload: payload1,
                },
            ))
            .unwrap();

        let payload2 = CommandTriggerPayload {
            module_name: "module2",
            command_triggers: vec![CommandTrigger {
                module_name: "module2",
                command_name: "cmd2",
                active_for_mode: ActiveForMode::Navigation,
                triggers: vec![],
                command: Command::CmdInputModule(CmdInputCommands::ProcessInput {
                    input: KeyEvent::new(KeyCode::Char('2'), KeyModifiers::NONE),
                }),
            }],
        };

        module
            .process_command(Command::CmdInputModule(
                CmdInputCommands::RegisterCommandTriggers {
                    command_trigger_payload: payload2,
                },
            ))
            .unwrap();

        assert_eq!(module.command_triggers.len(), 2);
    }

    #[test]
    fn test_navigation_mode_trigger() {
        let mut module = CmdInputModule::new();
        let module_name = "test_module";
        let trigger_key = KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE);
        let command = Command::ServerModule(crate::server::server_module::ServerCommands::ServerSelectionUp);

        let trigger = CommandTrigger {
            module_name,
            command_name: "up",
            active_for_mode: ActiveForMode::Navigation,
            triggers: vec![trigger_key],
            command: command.clone(),
        };

        module.process_command(Command::CmdInputModule(CmdInputCommands::RegisterCommandTriggers {
            command_trigger_payload: CommandTriggerPayload {
                module_name,
                command_triggers: vec![trigger],
            }
        })).unwrap();

        let result = module.process_command(Command::CmdInputModule(CmdInputCommands::ProcessInput {
            input: trigger_key,
        })).unwrap();

        if let ProcessingResult::Processed(payload) = result {
            assert_eq!(payload.commands.len(), 1);
        } else {
            panic!("Expected ProcessingResult::Processed");
        }
    }

    #[test]
    fn test_transition_to_advanced_command_mode() {
        let mut module = CmdInputModule::new();
        let colon_key = KeyEvent::new(KeyCode::Char(':'), KeyModifiers::NONE);

        module.process_command(Command::CmdInputModule(CmdInputCommands::ProcessInput {
            input: colon_key,
        })).unwrap();

        assert_eq!(module.mode, InputMode::AdvancedCommand);
    }

    #[test]
    fn test_advanced_command_mode_trigger() {
        let mut module = CmdInputModule::new();
        let module_name = "test_module";
        let trigger_keys = vec![
            KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Char('u'), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE),
        ];
        let command = Command::Application(crate::contract::contract_module::ApplicationCommands::Quit);

        let trigger = CommandTrigger {
            module_name,
            command_name: "quit",
            active_for_mode: ActiveForMode::AdvancedCommand,
            triggers: trigger_keys.clone(),
            command: command.clone(),
        };

        module.process_command(Command::CmdInputModule(CmdInputCommands::RegisterCommandTriggers {
            command_trigger_payload: CommandTriggerPayload {
                module_name,
                command_triggers: vec![trigger],
            }
        })).unwrap();

        // Enter AdvancedCommand mode
        module.process_command(Command::CmdInputModule(CmdInputCommands::ProcessInput {
            input: KeyEvent::new(KeyCode::Char(':'), KeyModifiers::NONE),
        })).unwrap();

        // Type 'quit'
        for key in trigger_keys {
            module.process_command(Command::CmdInputModule(CmdInputCommands::ProcessInput {
                input: key,
            })).unwrap();
        }

        // Press Enter
        let result = module.process_command(Command::CmdInputModule(CmdInputCommands::ProcessInput {
            input: KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE),
        })).unwrap();

        if let ProcessingResult::Processed(payload) = result {
            assert_eq!(payload.commands.len(), 1);
            assert_eq!(module.mode, InputMode::Navigation);
            assert!(module.input_buffer.is_empty());
        } else {
            panic!("Expected ProcessingResult::Processed");
        }
    }

    #[test]
    fn test_advanced_command_mode_resets_on_invalid_command() {
        let mut module = CmdInputModule::new();

        // Enter AdvancedCommand mode
        module.process_command(Command::CmdInputModule(CmdInputCommands::ProcessInput {
            input: KeyEvent::new(KeyCode::Char(':'), KeyModifiers::NONE),
        })).unwrap();

        // Type something random
        module.process_command(Command::CmdInputModule(CmdInputCommands::ProcessInput {
            input: KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE),
        })).unwrap();

        // Press Enter - should NOT panic now, but reset to Navigation
        let _ = module.process_command(Command::CmdInputModule(CmdInputCommands::ProcessInput {
            input: KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE),
        }));

        assert_eq!(module.mode, InputMode::Navigation);
        assert!(module.input_buffer.is_empty());
    }

    #[test]
    fn test_help_modal_navigation() {
        let mut module = CmdInputModule::new();
        let module_name = "test_module";
        let trigger1 = CommandTrigger {
            module_name,
            command_name: "cmd1",
            active_for_mode: ActiveForMode::Navigation,
            triggers: vec![KeyEvent::new(KeyCode::Char('1'), KeyModifiers::NONE)],
            command: Command::Application(crate::contract::contract_module::ApplicationCommands::Quit),
        };
        let trigger2 = CommandTrigger {
            module_name,
            command_name: "cmd2",
            active_for_mode: ActiveForMode::Navigation,
            triggers: vec![KeyEvent::new(KeyCode::Char('2'), KeyModifiers::NONE)],
            command: Command::Application(crate::contract::contract_module::ApplicationCommands::Quit),
        };

        module.process_command(Command::CmdInputModule(CmdInputCommands::RegisterCommandTriggers {
            command_trigger_payload: CommandTriggerPayload {
                module_name,
                command_triggers: vec![trigger1, trigger2],
            }
        })).unwrap();

        // Enter help
        module.show_help = true;
        assert_eq!(module.help_selected_index, 0);

        // Press 'j' to go down
        module.process_command(Command::CmdInputModule(CmdInputCommands::ProcessInput {
            input: KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE),
        })).unwrap();
        assert_eq!(module.help_selected_index, 1);

        // Press 'j' again to wrap around
        module.process_command(Command::CmdInputModule(CmdInputCommands::ProcessInput {
            input: KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE),
        })).unwrap();
        assert_eq!(module.help_selected_index, 0);

        // Press 'k' to wrap around to bottom
        module.process_command(Command::CmdInputModule(CmdInputCommands::ProcessInput {
            input: KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE),
        })).unwrap();
        assert_eq!(module.help_selected_index, 1);

        // Press 'k' to go up
        module.process_command(Command::CmdInputModule(CmdInputCommands::ProcessInput {
            input: KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE),
        })).unwrap();
        assert_eq!(module.help_selected_index, 0);

        // Press 'Esc' to close
        module.process_command(Command::CmdInputModule(CmdInputCommands::ProcessInput {
            input: KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
        })).unwrap();
        assert!(!module.show_help);

        // Press random key should not close anymore
        module.show_help = true;
        module.process_command(Command::CmdInputModule(CmdInputCommands::ProcessInput {
            input: KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE),
        })).unwrap();
        assert!(module.show_help);
    }
}
