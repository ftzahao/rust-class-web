use rustls::{pki_types::PrivateKeyDer, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::{fs::File, io::BufReader};

pub fn load_rustls_config() -> rustls::ServerConfig {
    let cert_path = std::env::var("TLS_CERT_PATH").unwrap_or("./cert.pem".to_string());
    let key_path = std::env::var("TLS_KEY_PATH").unwrap_or("./key.pem".to_string());
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();

    // init server config builder with safe defaults
    let config = ServerConfig::builder().with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open(cert_path).unwrap());
    let key_file = &mut BufReader::new(File::open(key_path).unwrap());

    // convert files to key/cert objects
    let cert_chain = certs(cert_file).collect::<Result<Vec<_>, _>>().unwrap();
    let mut keys = pkcs8_private_keys(key_file)
        .map(|key| key.map(PrivateKeyDer::Pkcs8))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    // exit if no keys could be parsed
    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}
