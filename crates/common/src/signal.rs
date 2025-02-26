// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use std::fmt::{Display, Formatter, Write};
use tokio::sync::broadcast::{Receiver, Sender};

#[derive(Clone, Debug, PartialEq)]
pub enum SignalKind {
    Shutdown,
    Terminate(String),
}

impl Display for SignalKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SignalKind::Shutdown => f.write_str("shutdown signal"),
            SignalKind::Terminate(reason) => {
                f.write_fmt(format_args!("termination signal: {reason}"))
            }
        }
    }
}

pub struct Signal {
    sender: Sender<SignalKind>,
    receiver: Receiver<SignalKind>,
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
        self.sender.send(SignalKind::Shutdown).unwrap();
    }

    pub fn terminate(&self, reason: impl Into<String>) {
        self.sender
            .send(SignalKind::Terminate(reason.into()))
            .unwrap();
    }

    pub async fn recv(&mut self) -> SignalKind {
        self.receiver.recv().await.unwrap()
    }

    pub async fn recv_maybe(&mut self) -> Option<SignalKind> {
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
        assert_eq!(received, SignalKind::Shutdown);

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
            SignalKind::Terminate("For that reason".to_string())
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

        assert_eq!(received1, SignalKind::Shutdown);
        assert_eq!(received2, SignalKind::Shutdown);

        sender_task.await.unwrap();
    }

    #[tokio::test]
    async fn test_receive_after_shutdown() {
        let signal = Signal::new();
        let mut receiver = signal.clone();

        signal.shutdown();

        let received = receiver.recv().await;
        assert_eq!(received, SignalKind::Shutdown);
    }

    #[tokio::test]
    async fn test_receive_after_terminate() {
        let signal = Signal::new();
        let mut receiver = signal.clone();

        signal.terminate("It's all over");

        let received = receiver.recv().await;
        assert_eq!(received, SignalKind::Terminate("It's all over".to_string()));
    }
}
