use crate::contract::contract_module::Command;

#[derive(Clone)]
pub struct CommandTriggerPayload {
    pub module_name: &'static str,
    pub command_triggers: Vec<CommandTrigger>,
}

impl CommandTriggerPayload {
    pub fn get_command_triggers(self) -> Vec<CommandTrigger> {
        self.command_triggers
    }
}

#[derive(Clone)]
pub struct CommandTrigger {
    pub command_name: &'static str,
    pub triggers: Vec<String>,
    pub command: Command,
}