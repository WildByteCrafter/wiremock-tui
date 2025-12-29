use crate::server::server_module::{ServerCommands, ServerEvents};
use ratatui::widgets::Widget;

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
    pub fn get_command_triggers(&self) -> Vec<CommandTrigger> {
        self.command_triggers.clone()
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
    Application(ApplicationEvents),
    ServerModule(ServerEvents),
    StubModule,
}

#[derive(Clone)]
pub enum ApplicationEvents {}

pub trait Task: Send {
    fn execute(&self) -> Result<Command, color_eyre::Report>;
}

pub enum ProcessingResult {
    NothingDone,
    Processed(ProcessingResultPayload),
}

pub struct ProcessingResultPayload {
    events: Vec<Event>,
    commands: Vec<Command>,
    tasks: Vec<Box<dyn Task>>,
}

impl ProcessingResultPayload {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            commands: Vec::new(),
            tasks: Vec::new(),
        }
    }
    pub fn with_event(mut self, event: Event) -> Self {
        self.events.push(event);
        self
    }

    pub fn with_command(mut self, command: Command) -> Self {
        self.commands.push(command);
        self
    }

    pub fn with_tasks(mut self, tasks: Box<dyn Task>) -> Self {
        self.tasks.push(tasks);
        self
    }

    pub fn add_result(&mut self, result: ProcessingResultPayload) {
        self.events.extend(result.events);
        self.commands.extend(result.commands);
        self.tasks.extend(result.tasks);
    }

    pub fn get_events(&self) -> Vec<Event> {
        self.events.clone()
    }

    pub fn get_commands(&self) -> Vec<Command> {
        self.commands.clone()
    }

    pub fn get_tasks(self) -> Vec<Box<dyn Task>> {
        self.tasks
    }
}

pub trait Module {
    fn process_command(&mut self, command: &Command) -> Result<ProcessingResult, color_eyre::Report>;

    fn main_widget(&self) -> Option<Box<dyn Widget>>;
}
