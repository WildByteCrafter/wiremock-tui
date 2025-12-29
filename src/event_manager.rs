use crate::contract::{ApplicationCommands, Command, CommandTrigger, CommandTriggerPayload, Task};
use crate::server::server_module::ServerCommands;
use color_eyre::eyre::OptionExt;
use color_eyre::Report;
use futures::StreamExt;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

const TICK_FPS: f64 = 30.0;

pub struct CommandManager {
    command_receiver: UnboundedReceiver<Command>,
    trigger_sender: UnboundedSender<CommandTriggerPayload>,
    task_sender: UnboundedSender<Box<dyn Task>>,
}

impl CommandManager {
    pub fn new() -> Self {
        let (command_sender, command_receiver) = mpsc::unbounded_channel::<Command>();
        let (trigger_sender, trigger_receiver) = mpsc::unbounded_channel::<CommandTriggerPayload>();
        let (task_sender, task_receiver) = mpsc::unbounded_channel::<Box<dyn Task>>();
        let mut task = CommandManagerTask::new(command_sender, trigger_receiver, task_receiver);
        let manager = CommandManager {
            command_receiver,
            task_sender,
            trigger_sender,
        };
        tokio::spawn(async move { task.run().await });
        manager
    }

    pub fn set_command_triggers(&self, payload: CommandTriggerPayload) -> Result<(), Report> {
        self.trigger_sender
            .send(payload)
            .map_err(|_| color_eyre::Report::msg("Failed to send command"))
    }

    pub fn execute(&self, tasks: Vec<Box<dyn Task>>) -> Result<(), Report> {
        for task in tasks {
            self.task_sender
                .send(task)
                .map(drop)
                .map_err(|_| color_eyre::Report::msg("Failed to send task"))?;
        }
        Ok(())
    }

    pub async fn next(&mut self) -> color_eyre::Result<Command> {
        self.command_receiver
            .recv()
            .await
            .ok_or_eyre("Failed to receive event")
    }
}

struct CommandManagerTask {
    trigger_receiver: UnboundedReceiver<CommandTriggerPayload>,
    task_receiver: UnboundedReceiver<Box<dyn Task>>,
    command_sender: UnboundedSender<Command>,
    command_triggers: Vec<CommandTrigger>,
}

impl CommandManagerTask {
    fn new(
        command_sender: UnboundedSender<Command>,
        trigger_receiver: UnboundedReceiver<CommandTriggerPayload>,
        task_receiver: UnboundedReceiver<Box<dyn Task>>,
    ) -> Self {
        CommandManagerTask {
            trigger_receiver,
            command_sender,
            task_receiver,
            command_triggers: vec![],
        }
    }
    async fn run(&mut self) -> Result<(), Report> {
        let tick_rate = Duration::from_secs_f64(1.0 / TICK_FPS);
        let mut reader = crossterm::event::EventStream::new();
        let mut tick = tokio::time::interval(tick_rate);
        self.send(Command::ServerModule(ServerCommands::ShowServerSelection));
        loop {
            let tick_delay = tick.tick();
            let crossterm_event = reader.next();
            tokio::select! {
              _ = self.command_sender.closed() => {
                break;
              }
              blub = self.trigger_receiver.recv() => {
                    match blub {
                        None => {}
                        Some(payload) => {
                            self.command_triggers.append(payload.get_command_triggers().as_mut())
                        }
                    }
              }
              task_option = self.task_receiver.recv() => {
                 match task_option {
                            Some(task) => {
                                let sender = self.command_sender.clone();
                                tokio::spawn(async move {
                                    if let Ok(result) = task.execute() {
                                        let _ = sender.send(result);
                                    }
                                });
                            }
                            None => {}
                        }
              }
              _ = tick_delay => {
                self.send(Command::Application(ApplicationCommands::Tick));
              }
            };
        }
        Ok(())
    }
    

    fn send(&self, event: Command) {
        // Ignores the result because shutting down the app drops the receiver, which causes the send
        // operation to fail. This is expected behavior and should not panic.
        let _ = self.command_sender.send(event);
    }
}
