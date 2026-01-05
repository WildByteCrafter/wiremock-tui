use crate::contract::contract_module::{ApplicationCommands, Command, Task};
use crate::server::server_module::ServerCommands;
use color_eyre::eyre::OptionExt;
use color_eyre::Report;
use crossterm::event::Event;
use futures::StreamExt;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

const TICK_FPS: f64 = 30.0;

pub struct CommandManager {
    command_receiver: UnboundedReceiver<Command>,
    task_sender: UnboundedSender<Box<dyn Task>>,
}

impl CommandManager {
    pub fn new() -> Self {
        let (command_sender, command_receiver) = mpsc::unbounded_channel::<Command>();
        let (task_sender, task_receiver) = mpsc::unbounded_channel::<Box<dyn Task>>();
        let mut task = CommandManagerTask::new(command_sender, task_receiver);
        let manager = CommandManager {
            command_receiver,
            task_sender,
        };
        tokio::spawn(async move { task.run().await });
        manager
    }

    pub fn execute(&self, tasks: Vec<Box<dyn Task>>) -> Result<(), Report> {
        for task in tasks {
            self.task_sender
                .send(task)
                .map(drop)
                .map_err(|_| Report::msg("Failed to send task"))?;
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
    task_receiver: UnboundedReceiver<Box<dyn Task>>,
    command_sender: UnboundedSender<Command>,
}

impl CommandManagerTask {
    fn new(
        command_sender: UnboundedSender<Command>,
        task_receiver: UnboundedReceiver<Box<dyn Task>>,
    ) -> Self {
        CommandManagerTask {
            command_sender,
            task_receiver,
        }
    }
    async fn run(&mut self) -> Result<(), Report> {
        let tick_rate = Duration::from_secs_f64(1.0 / TICK_FPS);
        let mut reader = crossterm::event::EventStream::new();
        let mut tick = tokio::time::interval(tick_rate);
        self.send(Command::ServerModule(ServerCommands::ShowServerSelection));
        loop {
            let tick_delay = tick.tick();
            let cross_term_event = reader.next();
            tokio::select! {
              _ = self.command_sender.closed() => {
                break;
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
                Some(Ok(evt)) = cross_term_event => {
                    match evt{
                        Event::Key(key_event) => {
                            let new_command = Command::Application(ApplicationCommands::ProcessInput { input: key_event });
                            self.command_sender.send(new_command)?;
                        }
                        _ => {}
                    }

              }
              _ = tick_delay => {
                self.send(Command::Application(ApplicationCommands::Tick));
              }
            }
        }
        Ok(())
    }

    fn send(&self, event: Command) {
        // Ignores the result because shutting down the app drops the receiver, which causes the send
        // operation to fail. This is expected behavior and should not panic.
        let _ = self.command_sender.send(event);
    }
}
