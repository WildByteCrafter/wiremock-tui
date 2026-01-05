use crate::cmd_input::cmd_input_module::{CmdInputCommands, CmdInputModule};
use crate::command_manager::CommandManager;
use crate::contract::contract_module::{ApplicationCommands, Command, Event, Module, Task};
use crate::contract::contract_processing::{ProcessingResult, ProcessingResultPayload};
use crate::server::server_module::{ServerEvents, ServerModule};
use crate::stub::stub_module::StubModule;
use color_eyre::Report;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::DefaultTerminal;
use std::collections::HashMap;

pub struct App {
    keep_running: bool,
    command_manager: CommandManager,
    modules: HashMap<&'static str, Box<dyn Module>>,
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
        let mut modules: HashMap<&'static str, Box<dyn Module>> = HashMap::new();

        let server_module = ServerModule::new();
        modules.insert(server_module.name(), Box::new(server_module));

        let stub_module = StubModule::new();
        modules.insert(stub_module.name(), Box::new(stub_module));

        let cmd_input_module = CmdInputModule::new();
        modules.insert(cmd_input_module.name(), Box::new(cmd_input_module));

        Self {
            keep_running: true,
            command_manager: CommandManager::new(),
            modules,
            app_state: AppState::Starting,
        }
    }

    pub async fn run(&mut self, mut terminal: DefaultTerminal) -> Result<(), Report> {
        loop {
            let command = self.command_manager.next().await?;
            let vec = self.process_command(command)?;
            self.command_manager.execute(vec)?;
            terminal.draw(|e| self.draw_frame(e))?;
            if !self.keep_running {
                break;
            }
        }
        Ok(())
    }

    fn draw_frame(&mut self, frame: &mut ratatui::Frame) {
        let area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(0),    // Main Content (takes the rest)
                Constraint::Length(1), // Footer
            ])
            .split(frame.area());

        let mainarea = area[1];

        if let Some(module) = self.modules.get_mut(self.app_state.active_module_for_ui()) {
            module.render(frame, mainarea);
        }
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
            for module in self.modules.values_mut() {
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
            Event::ServerModule(ServerEvents::ServerSelected { server: _ }) => {
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
                let new_command =
                    Command::CmdInputModule(CmdInputCommands::RegisterCommandTriggers {
                        command_trigger_payload,
                    });
                Ok(ProcessingResult::Processed(
                    ProcessingResultPayload::new().with_command(new_command),
                ))
            }
            Command::Application(ApplicationCommands::ProcessInput { input }) => {
                let new_command = Command::CmdInputModule(CmdInputCommands::ProcessInput { input });
                Ok(ProcessingResult::Processed(
                    ProcessingResultPayload::new().with_command(new_command),
                ))
            }
            _ => Ok(ProcessingResult::NothingDone),
        }
    }
}
