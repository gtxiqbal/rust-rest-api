use crate::utils::error::ErrorApp;
use std::env;
use std::str::{from_utf8, from_utf8_unchecked, Utf8Error};
use base64::{DecodeError, Engine};
use rsa::pkcs1::{DecodeRsaPublicKey, DecodeRsaPrivateKey, EncodeRsaPublicKey, EncodeRsaPrivateKey};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};

pub fn generate_rsa_key() {
    let resource_path = env::var("RESOURCES_PATH").unwrap_or("resources".to_string());
    let mut rnd = rand::thread_rng();
    let priv_key = RsaPrivateKey::new(&mut rnd, 2048).unwrap();
    priv_key.write_pkcs1_der_file(format!("{resource_path}/private")).unwrap();

    let pub_key = RsaPublicKey::from(&priv_key);
    pub_key.write_pkcs1_der_file(format!("{resource_path}/public")).unwrap();
}

pub fn encrypt(data: &str) -> Result<String, ErrorApp> {
    let resource_path = env::var("RESOURCES_PATH").unwrap_or("resources".to_string());
    let pub_key = match RsaPublicKey::read_pkcs1_der_file(format!("{resource_path}/public")) {
        Ok(result) => result,
        Err(err) => {
            return Err(ErrorApp::OtherErr(err.to_string()))
        },
    };

    let mut rnd = rand::thread_rng();
    let result_encrypt  = match pub_key.encrypt(&mut rnd, Pkcs1v15Encrypt, data.as_bytes()) {
        Ok(result) => result,
        Err(err) => {
            return Err(ErrorApp::OtherErr(err.to_string()))
        },
    };

    Ok(base64::engine::general_purpose::STANDARD_NO_PAD.encode(result_encrypt))
}

pub fn decrypt(data: &str) -> Result<String, ErrorApp> {
    let result_data = match base64::engine::general_purpose::STANDARD_NO_PAD.decode(data.as_bytes()) {
        Ok(result) => result,
        Err(err) => return Err(ErrorApp::OtherErr(err.to_string())),
    };

    let resource_path = env::var("RESOURCES_PATH").unwrap_or("resources".to_string());
    let priv_key = match RsaPrivateKey::read_pkcs1_der_file(format!("{resource_path}/private")) {
        Ok(result) => result,
        Err(err) => return Err(ErrorApp::OtherErr(err.to_string())),
    };

    let result_decrypt = match priv_key.decrypt(Pkcs1v15Encrypt, result_data.as_slice()) {
        Ok(result) => result,
        Err(err) => return Err(ErrorApp::OtherErr(err.to_string())),
    };

    match from_utf8(&result_decrypt) {
        Ok(result) => Ok(result.to_string()),
        Err(err) => Err(ErrorApp::OtherErr(err.to_string()))
    }
}