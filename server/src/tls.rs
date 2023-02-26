use rcgen::generate_simple_self_signed;

use tonic::transport::{Certificate, Identity, ServerTlsConfig};

#[allow(dead_code)]
pub(crate) fn get_tls_config() -> Result<ServerTlsConfig, Box<dyn std::error::Error>> {
    // Generate a certificate that's valid for "localhost" and "hello.world.example"
    let subject_alt_names = vec!["localhost".to_string(), "127.0.0.1".to_string()];

    let cert = generate_simple_self_signed(subject_alt_names)?;

    let pem = cert.serialize_pem()?;
    let key = cert.serialize_private_key_pem();

    let identity = Identity::from_pem(pem.clone(), key);

    Ok(ServerTlsConfig::new()
        .identity(identity)
        .client_ca_root(Certificate::from_pem(pem)))
}
