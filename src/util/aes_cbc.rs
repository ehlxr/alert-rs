use std::error::Error;

use aes::Aes256;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use rand::seq::SliceRandom;
use sha2::{Digest, Sha256};

// type AesCbc = Cbc<Aes256, NoPadding>;
type AesCbc = Cbc<Aes256, Pkcs7>;

// 随机字符串的元素
const BASE_STR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

// 为IV生成随机字符串
fn gen_ascii_chars(size: usize) -> String {
    let mut rng = &mut rand::thread_rng();
    String::from_utf8(
        BASE_STR
            .as_bytes()
            .choose_multiple(&mut rng, size)
            .cloned()
            .collect(),
    )
    .unwrap()
}

#[allow(dead_code)]
pub fn encrypt(key: &str, data: &str) -> Result<String, Box<dyn Error>> {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    let key = hasher.finalize();

    let iv_str = gen_ascii_chars(16);
    let iv = iv_str.as_bytes();
    let cipher = AesCbc::new_from_slices(&key, &iv)?;
    let ciphertext = cipher.encrypt_vec(data.as_bytes());
    let mut buffer = bytebuffer::ByteBuffer::from_bytes(iv);
    buffer.write_bytes(&ciphertext);
    Ok(base64::encode(buffer.to_bytes()))
}

pub fn decrypt(key: &str, data: &str) -> Result<String, Box<dyn Error>> {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    let key = hasher.finalize();

    let bytes = base64::decode(data)?;
    let iv = &bytes[..16];
    let cipher = AesCbc::new_from_slices(&key, &iv)?;
    let ciphertext = &bytes[16..];
    let decrypted_ciphertext = cipher.decrypt_vec(ciphertext)?;

    Ok(String::from_utf8(decrypted_ciphertext.to_vec())?)
}
