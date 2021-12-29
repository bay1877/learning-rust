use rsa::{PublicKey, RsaPrivateKey, RsaPublicKey, PaddingScheme};
use rand::rngs::OsRng;
use std::io;

fn main() {    
    let mut rng = OsRng;
    let bits = 3072;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);

    println!("Enter your name to say hello: ");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    // Encrypt
    let data = guess.as_bytes().to_vec();
    println!("name data: {}", String::from_utf8_lossy(&data[..]));
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let enc_data = public_key.encrypt(&mut rng, padding, &data[..]).expect("failed to encrypt");
    println!("\nEncrypted data: {}\n", String::from_utf8_lossy(&enc_data[..]));

    // Decrypt
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let dec_data = private_key.decrypt(padding, &enc_data).expect("failed to decrypt");
    assert_eq!(&data[..], &dec_data[..]);
    println!("Decrypted data: {}", String::from_utf8_lossy(&dec_data[..]));
}
