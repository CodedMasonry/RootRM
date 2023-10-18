use std::net::SocketAddr;

use crate::Command;

/// Quic server handling
pub struct QuicHandler;

pub fn add_commands() -> Vec<Box<dyn Command + Send + Sync>> {
    vec![Box::new(QuicHandler)]
}

impl crate::Command for QuicHandler {
    fn run(&self, mut args: std::str::SplitWhitespace) -> Result<(), Box<dyn std::error::Error>> {
        let lhost: SocketAddr = args
            .next()
            .unwrap_or_else(|| {
                println!("No local adress specified.\nUsing 0.0.0.0:8000");
                "0.0.0.0:8000"
            })
            .parse()?;

        tokio::spawn(async move {
            match make_thread(lhost).await {
                Ok(_) => println!("Quic Listener Stopped"),
                Err(e) => eprintln!("[-] Server failed: {:#?}", e),
            };
        });
        Ok(())
    }

    fn help(&self) {
        todo!()
    }

    fn name(&self) -> String {
        "quic_listener".to_string()
    }
}

async fn make_thread(_addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}