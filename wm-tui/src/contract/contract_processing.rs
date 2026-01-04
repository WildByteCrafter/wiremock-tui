use crate::contract::contract_module::{Command, Event, Task};
pub enum ProcessingResult {
    NothingDone,
    Processed(ProcessingResultPayload),
}

pub struct ProcessingResultPayload {
    pub events: Vec<Event>,
    pub commands: Vec<Command>,
    pub tasks: Vec<Box<dyn Task>>,
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
}