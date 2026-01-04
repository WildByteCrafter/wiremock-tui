use crate::contract::{
    ApplicationCommands, Command, Event, Module, ProcessingResult, ProcessingResultPayload, Task,
};
use crate::command_manager::CommandManager;
use crate::server::server_module::{ServerEvents, ServerModule};
use color_eyre::Report;
use ratatui::DefaultTerminal;

pub struct App {
    keep_running: bool,
    command_manager: CommandManager,
    modules: Vec<Box<dyn Module>>,
    app_state: AppState,
}

pub enum AppState {
    Starting,
    ServerSelection,
    ShowingStubs,
}

impl AppState {
    pub fn active_module_for_ui(&self) -> &str {
        match self {
            AppState::Starting => "application",
            AppState::ServerSelection => "server",
            AppState::ShowingStubs => "stub",
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            keep_running: true,
            command_manager: CommandManager::new(),
            modules: vec![Box::new(ServerModule::new())],
            app_state: AppState::Starting,
        }
    }

    pub async fn run(&mut self, mut terminal: DefaultTerminal) -> Result<(), Report> {
        loop {
            let command = self.command_manager.next().await?;
            let vec = self.process_command(command)?;
            self.command_manager.execute(vec)?;
            self.render_ui(&mut terminal)?;
            if !self.keep_running {
                break;
            }
        }
        Ok(())
    }

    fn render_ui(&mut self, terminal: &mut DefaultTerminal) -> Result<(), Report> {
        for module in &mut self.modules {
            if module.name().eq(self.app_state.active_module_for_ui()) {
                terminal.draw(|e| module.render(e))?;
            }
        }
        Ok(())
    }

    fn process_commands(&mut self, commands: Vec<Command>) -> Result<Vec<Box<dyn Task>>, Report> {
        let mut tasks: Vec<Box<dyn Task>> = Vec::new();
        for command in commands {
            tasks.extend(self.process_command(command)?);
        }
        Ok(tasks)
    }

    fn process_command(&mut self, command: Command) -> Result<Vec<Box<dyn Task>>, Report> {
        let mut processing_result = ProcessingResultPayload::new();
        // First, try to process with App itself
        let mut handled = false;
        if self.can_process_command(&command) {
            if let ProcessingResult::Processed(payload) =
                self.process_application_command(command.clone())?
            {
                processing_result.add_result(payload);
                handled = true;
            }
        }
        // If not handled by App, check the other modules
        if !handled {
            for module in &mut self.modules {
                if !module.can_process_command(&command) {
                    continue;
                }
                let res = module.process_command(command.clone())?;
                if let ProcessingResult::Processed(payload) = res {
                    processing_result.add_result(payload);
                    break;
                }
            }
        }
        self.evaluate_processing_result(processing_result)
    }

    pub fn evaluate_processing_result(
        &mut self,
        processing_result_payload: ProcessingResultPayload,
    ) -> Result<Vec<Box<dyn Task>>, Report> {
        let ProcessingResultPayload {
            events,
            tasks,
            commands,
        } = processing_result_payload;
        let mut res_commands = vec![];
        res_commands.extend(commands);
        res_commands.extend(self.process_events(events)?);
        let mut res_tasks: Vec<Box<dyn Task>> = vec![];
        res_tasks.extend(tasks);
        res_tasks.extend(self.process_commands(res_commands)?);
        Ok(res_tasks)
    }

    fn process_events(&mut self, events: Vec<Event>) -> Result<Vec<Command>, Report> {
        let mut commands: Vec<Command> = Vec::new();
        for event in events {
            commands.extend(self.process_event(event)?);
        }
        Ok(commands)
    }

    fn process_event(&mut self, event: Event) -> Result<Vec<Command>, Report> {
        match event {
            Event::ServerModule(ServerEvents::ServerSelected { server }) => {
                Ok(vec![Command::Application(ApplicationCommands::Quit)])
            }
            Event::ServerModule(ServerEvents::ServerSelectionReadyForDisplay) => {
                self.app_state = AppState::ServerSelection;
                Ok(vec![])
            }
            _ => Ok(vec![]),
        }
    }

    // Local command processing
    fn can_process_command(&self, command: &Command) -> bool {
        match command {
            Command::Application(_) => true,
            _ => false,
        }
    }

    fn process_application_command(
        &mut self,
        command: Command,
    ) -> Result<ProcessingResult, Report> {
        if !self.can_process_command(&command) {
            return Ok(ProcessingResult::NothingDone);
        }
        match command {
            Command::Application(ApplicationCommands::Tick) => {
                Ok(ProcessingResult::Processed(ProcessingResultPayload::new()))
            }
            Command::Application(ApplicationCommands::Quit) => {
                self.keep_running = false;
                Ok(ProcessingResult::Processed(ProcessingResultPayload::new()))
            }
            Command::Application(ApplicationCommands::SetCommandTriggers {
                command_trigger_payload,
            }) => {
                self.command_manager
                    .set_command_triggers(command_trigger_payload)
                    .map(|_| true)?;
                Ok(ProcessingResult::Processed(ProcessingResultPayload::new()))
            }
            _ => Ok(ProcessingResult::NothingDone),
        }
    }
}
