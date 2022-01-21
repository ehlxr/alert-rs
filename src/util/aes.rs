use aes::cipher::consts::{U16, U32};
use aes::cipher::generic_array::GenericArray;
use aes::{Aes256, Block, BlockDecrypt, BlockEncrypt, NewBlockCipher};

pub fn encrypt(key: &str, data: &str) -> String {
    let key = GenericArray::from_slice(key.as_bytes());
    let cipher = Aes256::new(&key);

    let bytes = data.as_bytes();

    let pos = bytes.len();
    let mut buf = Vec::with_capacity(pos);
    buf.extend_from_slice(bytes);
    let mut block = Block::default();
    buf.extend_from_slice(&block[..pos - pos]);

    // let mut block = GenericArray::clone_from_slice(&bytes[0..16]);
    // let mut block = GenericArray::clone_from_slice(data.as_bytes());

    cipher.encrypt_block(&mut block);

    base64::encode(block.as_slice())
}

pub fn decrypt(key: &str, data: &str) -> String {
    let key = GenericArray::from_slice(key.as_bytes());
    let cipher = Aes256::new(&key);

    let mut bytes = base64::decode(data).unwrap();

    // let pos = bytes.len();
    // let mut block = Block::default();
    // bytes.extend_from_slice(&block[..pos - pos]);

    let mut block = GenericArray::clone_from_slice(&bytes[0..16]);

    cipher.decrypt_block(&mut block);

    println!("my bytes {:?}", &block);

    // "".to_string()
    String::from_utf8(block.to_vec()).unwrap()
}
