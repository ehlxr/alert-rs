use aes::Aes256;
use block_modes::block_padding::ZeroPadding;
use block_modes::{BlockMode, Cbc};
use rand::seq::SliceRandom;

type AesCbc = Cbc<Aes256, ZeroPadding>;
// type AesCbc = Cbc<Aes256, Pkcs7>;

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

pub fn encrypt(key: &str, data: &str) -> String {
    let iv_str = gen_ascii_chars(16);
    let iv = iv_str.as_bytes();
    let cipher = AesCbc::new_from_slices(key.as_bytes(), iv).unwrap();
    let ciphertext = cipher.encrypt_vec(data.as_bytes());
    let mut buffer = bytebuffer::ByteBuffer::from_bytes(iv);
    buffer.write_bytes(&ciphertext);
    base64::encode(buffer.to_bytes())
}

pub fn decrypt(key: &str, data: &str) -> String {
    let bytes = base64::decode(data).unwrap();
    let cipher = AesCbc::new_from_slices(key.as_bytes(), &bytes[0..16]).unwrap();
    String::from_utf8(cipher.decrypt_vec(&bytes[16..]).unwrap()).unwrap()
}
