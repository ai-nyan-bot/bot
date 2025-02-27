// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use log::error;
use std::fmt::{Display, Formatter};
use std::process::exit;
use tokio::sync::broadcast::{Receiver, Sender};

#[derive(Clone, Debug, PartialEq)]
pub enum SignalType {
    Shutdown,
    Terminate(String),
}

impl Display for SignalType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SignalType::Shutdown => f.write_str("shutdown signal"),
            SignalType::Terminate(reason) => {
                f.write_fmt(format_args!("termination signal: {reason}"))
            }
        }
    }
}

pub struct Signal {
    sender: Sender<SignalType>,
    receiver: Receiver<SignalType>,
}

impl Clone for Signal {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            receiver: self.sender.subscribe(),
        }
    }
}

impl Default for Signal {
    fn default() -> Self {
        Self::new()
    }
}

impl Signal {
    pub fn new() -> Self {
        let (sender, receiver) = tokio::sync::broadcast::channel(1);
        Self { sender, receiver }
    }

    pub fn shutdown(&self) {
        self.sender.send(SignalType::Shutdown).unwrap();
    }

    pub fn terminate(&self, reason: impl Into<String>) {
        self.sender
            .send(SignalType::Terminate(reason.into()))
            .unwrap();
    }

    pub async fn recv(&mut self) -> SignalType {
        self.receiver.recv().await.unwrap_or_else(|_| {
            error!("Failed to receive signal - exit");
            exit(-1)
        })
    }

    pub async fn recv_maybe(&mut self) -> Option<SignalType> {
        self.receiver.try_recv().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::task;
    use tokio::time::{self, Duration};

    #[tokio::test]
    async fn test_signal_shutdown() {
        let signal = Signal::new();
        let mut receiver = signal.clone();

        let sender_task = task::spawn(async move {
            time::sleep(Duration::from_millis(1)).await;
            signal.shutdown();
        });

        let received = receiver.recv().await;
        assert_eq!(received, SignalType::Shutdown);

        sender_task.await.unwrap();
    }

    #[tokio::test]
    async fn test_signal_terminate() {
        let signal = Signal::new();
        let mut receiver = signal.clone();

        let sender_task = task::spawn(async move {
            time::sleep(Duration::from_millis(1)).await;
            signal.terminate("For that reason");
        });

        let received = receiver.recv().await;
        assert_eq!(
            received,
            SignalType::Terminate("For that reason".to_string())
        );

        sender_task.await.unwrap();
    }

    #[tokio::test]
    async fn test_multiple_receivers() {
        let signal = Signal::new();
        let mut receiver1 = signal.clone();
        let mut receiver2 = signal.clone();

        let sender_task = task::spawn(async move {
            time::sleep(Duration::from_millis(1)).await;
            signal.shutdown();
        });

        let received1 = receiver1.recv().await;
        let received2 = receiver2.recv().await;

        assert_eq!(received1, SignalType::Shutdown);
        assert_eq!(received2, SignalType::Shutdown);

        sender_task.await.unwrap();
    }

    #[tokio::test]
    async fn test_receive_after_shutdown() {
        let signal = Signal::new();
        let mut receiver = signal.clone();

        signal.shutdown();

        let received = receiver.recv().await;
        assert_eq!(received, SignalType::Shutdown);
    }

    #[tokio::test]
    async fn test_receive_after_terminate() {
        let signal = Signal::new();
        let mut receiver = signal.clone();

        signal.terminate("It's all over");

        let received = receiver.recv().await;
        assert_eq!(received, SignalType::Terminate("It's all over".to_string()));
    }
}
