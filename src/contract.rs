use crate::server::server_module::{ServerCommands, ServerEvents};
use ratatui::Frame;

#[derive(Clone)]
pub enum Command {
    Application(ApplicationCommands),
    ServerModule(ServerCommands),
    StubModule,
}

#[derive(Clone)]
pub enum ApplicationCommands {
    Tick,
    Quit,
    SetCommandTriggers {
        command_trigger_payload: CommandTriggerPayload,
    },
}

#[derive(Clone)]
pub struct CommandTriggerPayload {
    module_name: &'static str,
    command_triggers: Vec<CommandTrigger>,
}

impl CommandTriggerPayload {
    pub fn get_command_triggers(self) -> Vec<CommandTrigger> {
        self.command_triggers
    }
}

#[derive(Clone)]
pub struct CommandTrigger {
    command_name: &'static str,
    triggers: Vec<String>,
    command: Box<Command>,
}

#[derive(Clone)]
pub enum Event {
    ServerModule(ServerEvents),
    StubModule,
}

pub trait Task: Send {
    fn execute(&self) -> Result<Command, color_eyre::Report>;
}

pub enum ProcessingResult {
    NothingDone,
    Processed(ProcessingResultPayload),
}

pub struct ProcessingResultPayload {
    pub events: Vec<Event>,
    pub tasks: Vec<Box<dyn Task>>,
}

impl ProcessingResultPayload {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            tasks: Vec::new(),
        }
    }
    pub fn with_event(mut self, event: Event) -> Self {
        self.events.push(event);
        self
    }

    pub fn with_tasks(mut self, tasks: Box<dyn Task>) -> Self {
        self.tasks.push(tasks);
        self
    }

    pub fn add_result(&mut self, result: ProcessingResultPayload) {
        self.events.extend(result.events);
        self.tasks.extend(result.tasks);
    }
}

pub trait Module {
    fn name(&self) -> &'static str;

    fn can_process_command(&self, command: &Command) -> bool;

    fn process_command(&mut self, command: Command)
    -> Result<ProcessingResult, color_eyre::Report>;

    fn render(&self, frame: &mut Frame);
}
