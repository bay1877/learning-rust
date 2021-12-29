use rsa::{PublicKey, RsaPrivateKey, RsaPublicKey, PaddingScheme};
use rand::rngs::OsRng;

fn main() {    
    let mut rng = OsRng;
    let bits = 3072;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);

    // Encrypt
    let data = b"hello world";
    println!("data: {}", String::from_utf8_lossy(&data[..]));

    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let enc_data = public_key.encrypt(&mut rng, padding, &data[..]).expect("failed to encrypt");
    println!("\nencrypted data: {}\n", String::from_utf8_lossy(&enc_data[..]));

    // Decrypt
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let dec_data = private_key.decrypt(padding, &enc_data).expect("failed to decrypt");
    assert_eq!(&data[..], &dec_data[..]);
    println!("\ndecrypted data: {}\n", String::from_utf8_lossy(&dec_data[..]));
}

