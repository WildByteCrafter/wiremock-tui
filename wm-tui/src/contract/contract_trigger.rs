use crate::contract::contract_module::Command;
use crossterm::event::KeyEvent;

#[derive(Clone,Debug)]
pub struct CommandTriggerPayload {
    pub module_name: &'static str,
    pub command_triggers: Vec<CommandTrigger>,
}

impl CommandTriggerPayload {
    pub fn get_command_triggers(self) -> Vec<CommandTrigger> {
        self.command_triggers
    }
}

#[derive(Clone,Debug)]
pub struct CommandTrigger {
    pub module_name: &'static str,
    pub command_name: &'static str,
    pub active_for_mode: ActiveForMode,
    pub triggers: Vec<KeyEvent>,
    pub command: Command,
}

#[derive(Clone,Debug)]
pub enum ActiveForMode {
    Navigation,
    AdvancedCommand,
}
