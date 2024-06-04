use std::{io::Bytes, str::from_utf8, string};

use base64::STANDARD;
use magic_crypt::{new_magic_crypt, MagicCrypt, MagicCryptTrait};

// mod encrypt;

#[tokio::main]
async fn main() {
    // let msg = b"MjEzNDk1";
    // println!("{}",String::from_utf8(encrypt::decrypt(msg,
    // b"37911490979715163134003223491201", b"3134003223491201").unwrap()).unwrap());
    // println!("{}",String::from_utf8(encrypt::encrypt(msg,
    // b"37911490979715163134003223491201", b"3134003223491201").unwrap()).unwrap());

    // println!("{:?}\n{:?}\n{:?}\n",hex::decode("37911490979715163134003223491201").unwrap(),
    //     128,
    //     hex::decode(b"3134003223491201").unwrap());

    let hex_key = hex::decode("37911490979715163134003223491201").unwrap();
    let hex_iv = hex::decode("3134003223491201").unwrap();

    let mc = MagicCrypt::new(
        hex_key,
        magic_crypt::SecureBit::Bit128,
        Some(hex_iv)
    );

    // let base64 = mc.decrypt_bytes_to_bytes(&base64::decode_config("MjEzNDk1", STANDARD).unwrap()).unwrap();
    let base64_str = mc.encrypt_bytes_to_base64(&hex::encode(b"MjEzNDk1"));
    // base64::cox
    println!("{:?}\n{}", base64::encode(base64_str), "x");

    // println!("{}", from_utf8(&base64).unwrap());
    // println!("{:?}", from_utf8(&base64))
}
