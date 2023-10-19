use std::net::SocketAddr;

use async_trait::async_trait;

use crate::Command;

/// Quic server handling
pub struct QuicHandler;

pub fn add_commands() -> Vec<Box<dyn Command + Send + Sync>> {
    vec![Box::new(QuicHandler)]
}

#[async_trait]
impl crate::Command for QuicHandler {
    async fn run(&self, mut _args: std::str::SplitWhitespace<'_>) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn help(&self) {
        todo!()
    }

    fn name(&self) -> String {
        "quic_listener".to_string()
    }
}

/*
async fn make_thread(_addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
*/