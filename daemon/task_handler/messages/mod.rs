use std::time::Duration;

use log::warn;

use pueue_lib::network::message::*;

use crate::task_handler::TaskHandler;

mod group;
mod kill;
mod pause;
mod send;
mod start;

impl TaskHandler {
    /// Some client instructions require immediate action by the task handler
    /// This function is also responsible for waiting
    pub fn receive_messages(&mut self) {
        // Sleep for a few milliseconds. We don't want to hurt the CPU.
        let timeout = Duration::from_millis(200);
        if let Ok(message) = self.receiver.recv_timeout(timeout) {
            self.handle_message(message);
        };
    }

    fn handle_message(&mut self, message: Message) {
        match message {
            Message::Pause(message) => self.pause(message.tasks, message.children, message.wait),
            Message::Start(message) => self.start(message.tasks, message.children),
            Message::Kill(message) => {
                self.kill(message.tasks, message.children, true, message.signal)
            }
            Message::Send(message) => self.send(message.task_id, message.input),
            Message::Reset(message) => self.reset(message.children),
            Message::Group(message) => self.handle_group_message(message),
            Message::DaemonShutdown(shutdown) => {
                self.initiate_shutdown(shutdown);
            }
            _ => warn!("Received unhandled message {message:?}"),
        }
    }
}
