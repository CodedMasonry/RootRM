use std::path::PathBuf;

use tokio::{fs, io};

use anyhow::{bail, Context, Result};
use async_trait::async_trait;
use rustls::{self, Certificate, PrivateKey};
use tracing::{error, info, info_span};

use crate::{parse_flags, Command};

/// Quic server handling
pub struct QuicHandler;

pub fn add_commands() -> Vec<Box<dyn Command + Send + Sync>> {
    vec![Box::new(QuicHandler)]
}

#[async_trait]
impl crate::Command for QuicHandler {
    async fn run(&self, mut args: std::str::SplitWhitespace<'_>) -> Result<()> {
        let _address = args.next().context("Expected IP; got None")?;
        let flags = parse_flags(args).await;

        let temp_key = match flags.get("key") {
            Some(i) => Some(PathBuf::from(i)),
            None => None,
        };
        let temp_cert = match flags.get("cert") {
            Some(i) => Some(PathBuf::from(i)),
            None => None,
        };

        let (cert, key) = parse_keys_and_certs(temp_key, temp_cert).await?;
        let server_crypto = rustls::ServerConfig::builder().with_safe_defaults().with_no_client_auth().with_single_cert(cert, key)?;


        Ok(())
    }

    fn help(&self) {
        todo!()
    }

    fn name(&self) -> String {
        "quic_listener".to_string()
    }
}

/// Attempts to key certificates and private keys from a file; if no file is found it will generate self-signed
async fn parse_keys_and_certs(
    key: Option<PathBuf>,
    cert: Option<PathBuf>,
) -> Result<(Vec<Certificate>, PrivateKey)> {
    if let (Some(key_path), Some(cert_path)) = (key, cert) {
        let key = fs::read(key_path.clone())
            .await
            .context("failed to read private key")?;
        let key = if key_path.extension().map_or(false, |x| x == "der") {
            rustls::PrivateKey(key)
        } else {
            let pkcs8 = rustls_pemfile::pkcs8_private_keys(&mut &*key)
                .context("malformed PKCS #8 private key")?;
            match pkcs8.into_iter().next() {
                Some(x) => rustls::PrivateKey(x),
                None => {
                    let rsa = rustls_pemfile::rsa_private_keys(&mut &*key)
                        .context("malformed PKCS #1 private key")?;
                    match rsa.into_iter().next() {
                        Some(x) => rustls::PrivateKey(x),
                        None => {
                            anyhow::bail!("no private keys found");
                        }
                    }
                }
            }
        };
        let cert_chain = fs::read(cert_path.clone())
            .await
            .context("failed to read certificate chain")?;
        let cert_chain = if cert_path.extension().map_or(false, |x| x == "der") {
            vec![rustls::Certificate(cert_chain)]
        } else {
            rustls_pemfile::certs(&mut &*cert_chain)
                .context("invalid PEM-encoded certificate")?
                .into_iter()
                .map(rustls::Certificate)
                .collect()
        };

        Ok((cert_chain, key))
    } else {
        let dirs = directories_next::ProjectDirs::from("org", "quinn", "quinn-examples").unwrap();
        let path = dirs.data_local_dir();
        let cert_path = path.join("cert.der");
        let key_path = path.join("key.der");
        let (cert, key) = match fs::read(&cert_path)
            .await
            .and_then(|x| Ok((x, std::fs::read(&key_path)?)))
        {
            Ok(x) => x,
            Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
                info!("generating self-signed certificate");
                let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
                let key = cert.serialize_private_key_der();
                let cert = cert.serialize_der().unwrap();
                fs::create_dir_all(path)
                    .await
                    .context("failed to create certificate directory")?;
                fs::write(&cert_path, &cert)
                    .await
                    .context("failed to write certificate")?;
                fs::write(&key_path, &key)
                    .await
                    .context("failed to write private key")?;
                (cert, key)
            }
            Err(e) => {
                bail!("failed to read certificate: {}", e);
            }
        };

        let key = rustls::PrivateKey(key);
        let cert = rustls::Certificate(cert);
        Ok((vec![cert], key))
    }
}
