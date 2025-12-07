use crate::model::{Command, Message, ModelTrait};
use crate::stub::model::StubCommand::ReadAllStubs;
use crate::wire_mock;
use async_trait::async_trait;
use std::error::Error;
use std::time::Duration;
use thiserror::Error;
use tokio::sync::broadcast::Sender;
use tokio::time::interval;

pub struct StubModel {
    pub selected_server_url: Option<String>,
    pub event_sender: Sender<Message>,
    pub command_sender: Sender<Command>,
    pub stubs: Vec<wire_mock::client::StubMapping>,
    pub selected_stub_index: usize,
    pub scroll_offset: usize,
    pub refresh_task: Option<tokio::task::JoinHandle<()>>,
}

#[async_trait]
impl ModelTrait<StubMsg, StubCommand> for StubModel {
    async fn apply_event(&mut self, event: StubMsg) -> Result<(), Box<dyn Error>> {
        match event {
            StubMsg::SelectNext => {
                self.select_next_stub();
                Ok(())
            }
            StubMsg::SelectPrevious => {
                self.select_previous_stub();
                Ok(())
            }
            StubMsg::ScrollDetailsUp => {
                self.scroll_details_up();
                Ok(())
            }
            StubMsg::ScrollDetailsDown => {
                self.scroll_details_down();
                Ok(())
            }
            StubMsg::DeleteSelectedRequested => {
                self.command_sender
                    .send(Command::Stub(StubCommand::DeleteSelectedStub))?;
                Ok(())
            }
            StubMsg::ReadAllStubsRequested => {
                self.command_sender.send(Command::Stub(ReadAllStubs))?;
                Ok(())
            }
            StubMsg::ToggleAutoRefreshStubsRequested => {
                self.toggle_auto_refresh_stubs();
                Ok(())
            }
        }
    }

    async fn handle_command(&mut self, command: StubCommand) -> Result<(), Box<dyn Error>> {
        match command {
            ReadAllStubs => self.read_all_stubs(),
            StubCommand::DeleteSelectedStub => self.delete_selected_stub(),
        }
    }
}

impl StubModel {
    pub fn new(event_sender: Sender<Message>, command_sender: Sender<Command>) -> Self {
        Self {
            selected_server_url: None,
            event_sender,
            command_sender,
            stubs: vec![],
            selected_stub_index: 0,
            scroll_offset: 0,
            refresh_task: None,
        }
    }

    fn read_all_stubs(&mut self) -> Result<(), Box<dyn Error>> {
        if self.selected_server_url.is_none() {
            return Err(Box::new(StubError::NoServerSelected));
        }
        let res = wire_mock::client::get_all_stubs(&self.selected_server_url.as_ref().unwrap())?;
        self.stubs = res.mappings;
        Ok(())
    }

    fn select_next_stub(&mut self) {
        if self.stubs.is_empty() {
            return;
        }
        self.selected_stub_index = (self.selected_stub_index + 1).min(self.stubs.len() - 1);
        self.scroll_offset = 0;
    }

    fn select_previous_stub(&mut self) {
        self.selected_stub_index = self.selected_stub_index.saturating_sub(1);
        self.scroll_offset = 0; // Reset scroll when changing stub
    }

    fn scroll_details_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    fn scroll_details_down(&mut self) {
        self.scroll_offset += 1;
    }

    fn toggle_auto_refresh_stubs(&mut self) {
        if let Some(task) = self.refresh_task.take() {
            task.abort();
            return;
        }
        let sender = self.event_sender.clone();
        let task = tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(1));
            interval.tick().await;
            loop {
                interval.tick().await;
                if sender
                    .send(Message::Stub(StubMsg::ReadAllStubsRequested))
                    .is_err()
                {
                    break;
                }
            }
        });
        self.refresh_task = Some(task);
    }

    fn delete_selected_stub(&mut self) -> Result<(), Box<dyn Error>> {
        if self.stubs.is_empty() || self.selected_server_url.is_none() {
            return Ok(());
        }
        let idx = self.selected_stub_index.min(self.stubs.len() - 1);
        if let Some(stub) = self.stubs.get(idx) {
            let id = stub.id.clone();
            // Perform delete on server
            wire_mock::client::delete_stub(self.selected_server_url.as_ref().unwrap(), &id)?;
            // Remove locally
            self.stubs.remove(idx);
            // Adjust selection
            if self.stubs.is_empty() {
                self.selected_stub_index = 0;
                self.scroll_offset = 0;
            } else {
                if idx >= self.stubs.len() {
                    self.selected_stub_index = self.stubs.len() - 1;
                }
                self.scroll_offset = 0;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum StubCommand {
    ReadAllStubs,
    DeleteSelectedStub,
}

#[derive(Clone, Debug)]
pub enum StubMsg {
    SelectNext,
    SelectPrevious,
    ScrollDetailsUp,
    ScrollDetailsDown,
    ToggleAutoRefreshStubsRequested,
    DeleteSelectedRequested,
    ReadAllStubsRequested,
}

#[derive(Error, Debug)]
pub enum StubError {
    #[error("No server selected")]
    NoServerSelected,
}
