use crate::contract::{
    ApplicationCommands, Command, Event, Module, ProcessingResult, ProcessingResultPayload, Task,
};
use crate::event_manager::CommandManager;
use crate::server::server_module::ServerModule;
use ratatui::DefaultTerminal;

pub struct App {
    keep_running: bool,
    command_manager: CommandManager,
    modules: Vec<Box<dyn Module>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            keep_running: true,
            command_manager: CommandManager::new(),
            modules: vec![Box::new(ServerModule::new())],
        }
    }

    pub async fn run(&mut self, mut terminal: DefaultTerminal) -> Result<(), color_eyre::Report> {
        loop {
            let command = self.command_manager.next().await?;
            let vec = self.process_command(&command)?;
            self.command_manager.execute(vec);
            if !self.keep_running {
                break;
            }
        }
        Ok(())
    }

    fn process_command(
        &mut self,
        command: &Command,
    ) -> Result<Vec<Box<dyn Task>>, color_eyre::Report> {
        let mut processing_result = ProcessingResultPayload::new();
        for module in &mut self.modules {
            let res = module.process_command(&command)?;
            match res {
                ProcessingResult::Processed(payload) => {
                    processing_result.add_result(payload);
                    break;
                }
                _ => {}
            }
        }
        let mut commands = processing_result.get_commands();
        commands = self.process_application_commands(commands)?;
        commands.append(&mut self.process_events(processing_result.get_events()));

        let mut tasks: Vec<Box<dyn Task>> = vec![];
        for task in processing_result.get_tasks() {
            tasks.push(task);
        }
        for command in commands {
            self.process_command(&command)?;
        }
        Ok(tasks)
    }

    fn process_application_commands(
        &mut self,
        mut commands: Vec<Command>,
    ) -> Result<Vec<Command>, color_eyre::Report> {
        let mut i = 0;
        while i < commands.len() {
            if self.process_application_command(&commands[i])? {
                commands.remove(i);
            } else {
                i += 1;
            }
        }
        Ok(commands)
    }

    fn process_application_command(
        &mut self,
        command: &Command,
    ) -> Result<bool, color_eyre::Report> {
        match command {
            Command::Application(application_command) => match application_command {
                ApplicationCommands::Tick => Ok(true),
                ApplicationCommands::Quit => {
                    self.keep_running = false;
                    Ok(true)
                }
                ApplicationCommands::SetCommandTriggers {
                    command_trigger_payload,
                } => self
                    .command_manager
                    .set_command_triggers(command_trigger_payload.clone())
                    .map(|_| true),
            },
            _ => Ok(false),
        }
    }

    fn process_events(&mut self, events: Vec<Event>) -> Vec<Command> {
        let mut commands = vec![];
        for event in events {
            commands.append(&mut self.process_event(&event));
        }
        commands
    }

    fn process_event(&mut self, event: &Event) -> Vec<Command> {
        match event {
            _ => vec![],
        }
    }
}
