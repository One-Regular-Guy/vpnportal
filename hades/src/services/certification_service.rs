use anyhow::{Context, Result};
use openssl::asn1::Asn1Time;
use openssl::bn::BigNum;
use openssl::error;
use openssl::hash::MessageDigest;
use openssl::nid::Nid;
use openssl::pkey::{PKey, Private};
use openssl::rsa::Rsa;
use openssl::x509::extension::{
    AuthorityKeyIdentifier, BasicConstraints, ExtendedKeyUsage, KeyUsage, SubjectAlternativeName,
    SubjectKeyIdentifier,
};
use openssl::x509::{X509Builder, X509NameBuilder, X509};
use openssl::rand::rand_bytes;

pub fn certificate_execute(client_cn: impl Into<String>,ca_cert_pem: Vec<u8>, ca_key_pem: Vec<u8>) -> Result<(Vec<u8>, Vec<u8>), error::ErrorStack> {
    let client_cn = client_cn.into();
    let ca_cert = X509::from_pem(&ca_cert_pem)?;
    let ca_key = PKey::private_key_from_pem(&ca_key_pem)?;
    let (client_cert, client_key) = generate_signed_cert(
        &ca_cert,
        &ca_key,
        &client_cn.clone(),
        vec![&client_cn], // SAN opcional
        vec![],
        false,  // ClientAuth
        365,
    )?;
    let (cert, key) = (client_cert.to_pem()?, client_key.private_key_to_pem_pkcs8()?);
    Ok((cert, key))
}
 fn generate_signed_cert(
    ca_cert: &X509,
    ca_key: &PKey<Private>,
    common_name: &str,
    dns_sans: Vec<&str>,
    ip_sans: Vec<&str>,
    is_server: bool,
    days_valid: u32,
) -> Result<(X509, PKey<Private>), error::ErrorStack> {
    // Gerar nova chave RSA 4096 bits
    let rsa = Rsa::generate(4096)?;
    let key = PKey::from_rsa(rsa)?;

    // Nome do sujeito
    let mut name_builder = X509NameBuilder::new()?;
    name_builder.append_entry_by_nid(Nid::COMMONNAME, common_name)?;
    let subject_name = name_builder.build();

    // Validade
    let not_before = Asn1Time::days_from_now(0)?;
    let not_after = Asn1Time::days_from_now(days_valid)?;

    // Builder do certificado
    let mut builder = X509Builder::new()?;
    builder.set_version(2)?; // v3
    // Serial simples e único (use timestamp como base)
    let mut serial_buf = [0u8; 16];
    rand_bytes(&mut serial_buf)?;
    let serial = BigNum::from_slice(&serial_buf)?;
    builder.set_serial_number(&serial.to_asn1_integer()?.as_ref())?;
    builder.set_subject_name(&subject_name)?;
    builder.set_issuer_name(ca_cert.subject_name())?;
    builder.set_not_before(&not_before)?;
    builder.set_not_after(&not_after)?;
    builder.set_pubkey(&key)?;

    // Subject Key Identifier (usa contexto)
    let skid = SubjectKeyIdentifier::new()
        .build(&builder.x509v3_context(Some(ca_cert), None))?;
    builder.append_extension(skid)?;

    // Authority Key Identifier
    let akid = AuthorityKeyIdentifier::new()
        .keyid(true)
        .issuer(true)
        .build(&builder.x509v3_context(Some(ca_cert), None))?;
    builder.append_extension(akid)?;

    // Basic Constraints: não é CA
    let bc = BasicConstraints::new()
        .build()?;
    builder.append_extension(bc)?;

    // Key Usage
    let ku = KeyUsage::new()
        .digital_signature()
        .key_encipherment()
        .build()?;
    builder.append_extension(ku)?;

    // Extended Key Usage
    let mut eku_builder = &mut ExtendedKeyUsage::new();
    if is_server {
        eku_builder = eku_builder.server_auth();
    } else {
        eku_builder = eku_builder.client_auth();
    }
    let eku = eku_builder.build()?;
    builder.append_extension(eku)?;

    // Subject Alternative Name (se houver)
    if !dns_sans.is_empty() || !ip_sans.is_empty() {
        let mut san_builder = &mut SubjectAlternativeName::new();
        for dns in dns_sans {
            san_builder = san_builder.dns(dns);
        }
        for ip in ip_sans {
            san_builder = san_builder.ip(ip);
        }
        let san = san_builder.build(&builder.x509v3_context(Some(ca_cert), None))?;
        builder.append_extension(san)?;
    }

    // Assinar com a chave da CA
    builder.sign(ca_key, MessageDigest::sha256())?;

    let cert = builder.build();

    Ok((cert, key))
}