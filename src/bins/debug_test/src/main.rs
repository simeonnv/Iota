use crypto::{
    kem::ml_kem1024::{
        decrypt_ml_kem1024::decrypt_ml_kem1024, encrypt_ml_kem1024::encrypt_ml_kem1024,
        generate_ml_kem1024_key_pair::generate_ml_kem1024_key_pair,
    },
    sign::dilithium3::generate_dilithium3_key_pair::generate_dilithium3_key_pair,
};

fn main() {
    let message: Vec<u8> = vec![69, 67, 6, 9];
    let keypair = generate_ml_kem1024_key_pair().unwrap();
    let (kem_ciphertext, ciphertext) = encrypt_ml_kem1024(&message, &keypair.public_key).unwrap();
    let decrypt_message =
        decrypt_ml_kem1024(&ciphertext, &kem_ciphertext, &keypair.private_key).unwrap();

    println!("before encrypt: {:?}, len: {}", message, message.len());
    println!("after encrypt: {:?}, len: {}", ciphertext, ciphertext.len());
    // println!(
    //     "kem_ciphertext: {:?}, len: {}",
    //     kem_ciphertext,
    //     kem_ciphertext.len()
    // );
    println!(
        "after decrypt: {:?}, len: {}",
        decrypt_message,
        decrypt_message.len()
    );

    println!("Hello, world!");
}
