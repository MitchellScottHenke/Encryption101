use chacha20::cipher::{KeyIvInit, StreamCipher};
use chacha20::{ChaCha20};
use rfd::FileDialog;
use std::fs::{File};
use std::io::{Read, Write};
use std::io;

fn encrypt_file(input_file: &str, output_file: &str, key: &[u8], nonce: &[u8]) {
    let mut cipher = ChaCha20::new(key.into(), nonce.into());

    let mut input = File::open(input_file).expect("Failed to open input file");
    let mut output = File::create(output_file).expect("Failed to create output file");

    let mut buffer = [0u8; 64];
    loop {
        let bytes_read = input.read(&mut buffer).expect("Failed to read from input file");
        if bytes_read == 0 {
            break;
        }

        cipher.apply_keystream(&mut buffer[..bytes_read]);
        output.write_all(&buffer[..bytes_read]).expect("Failed to write to output file");
    }
}

fn decrypt_file(input_file: &str, output_file: &str, key: &[u8], nonce: &[u8]) {
    let mut cipher = ChaCha20::new(key.into(), nonce.into());

    let mut input = File::open(input_file).expect("Failed to open input file");
    let mut output = File::create(output_file).expect("Failed to create output file");

    let mut buffer = [0u8; 64];
    loop {
        let bytes_read = input.read(&mut buffer).expect("Failed to read input file");
        if bytes_read == 0{
            break;
        }

        cipher.apply_keystream(&mut buffer[..bytes_read]);
        output.write_all(&buffer[..bytes_read]).expect("Failed to write to output file");
    }
}

fn pad_key(key: &[u8]) -> [u8; 32] {
    let mut padded_key = [0u8; 32];
    let len = key.len().min(32);
    padded_key[..len].copy_from_slice(&key[..len]);
    padded_key
}

fn main() {
    let key = b"correct horse battery staple";
    let padded_key = pad_key(key);
    let nonce = b"unique nonce";
    print!("{:?}", padded_key);


    println!("Select an option:");
    println!("1. Encrypt a file");
    println!("2. Decrypt a file");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice = choice.trim();

    match choice {
        "1" => {
            let input_file = FileDialog::new()
                .add_filter("Text files", &["txt"])
                .pick_file()
                .expect("No file selected")
                .display()
                .to_string();

            let output_file = "encrypted.txt";
            encrypt_file(&input_file, output_file, &padded_key, nonce);
            println!("File encrypted successfully!");
        }
        "2" => {
            let input_file = FileDialog::new()
                .add_filter("Text files", &["txt"])
                .pick_file()
                .expect("No file selected")
                .display()
                .to_string();

            let output_file = "decrypted.txt";
            decrypt_file(&input_file, output_file, &padded_key, nonce);
            println!("File decrypted successfully!");
        }
        _ => println!("Invalid choice! Please select 1 or 2."),
    }
}
