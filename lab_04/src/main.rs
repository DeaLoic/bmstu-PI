use std::io::stdin;
use std::str;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod rsa;
use rsa::RSAEncryptor;
use num_bigint::BigUint;

fn main() {
    let mut choose = 1;
    let mut buf = String::default();
    println!("Start");
    let mut rsa = RSAEncryptor::new();
    println!("Created");
    while choose != 0 {
        print_menu();
        stdin().read_line(&mut buf).expect("Expected choose");
        let res = buf.trim();
        buf = String::from(res);
        println!("o{}o", buf);
        let res_choose = buf.parse();
        if let Err(res) = res_choose {
            println!("{} Expected choose. Try again\n", res);
            buf = String::default();
            continue;
        }

        choose = res_choose.unwrap();
        match choose {
            1 => encrypt(&mut rsa),
            2 => decrypt(&mut rsa),
            3 => encrypt_file(&mut rsa),
            4 => decrypt_file(&mut rsa),
            _ => (),
        }
        println!("\n");
        buf = String::default();
    }
}

fn print_menu() {
    println!("0. Exit");
    println!("1. Encrypt string");
    println!("2. Decrypt string");
    println!("3. Encrypt file");
    println!("4. Decrypt file");
}

fn encrypt(rsa: &mut RSAEncryptor) {
    let mut buf = String::default();
    println!("Insert line for encrypt: ");
    stdin().read_line(&mut buf).expect("Expected data");
    let res = buf.trim();
    let q: Vec<u32> = res.as_bytes().iter().map(|&e| e as u32).collect();
    let digit = BigUint::from_slice(q.as_ref());

    println!{"Parsed: {:?} {}", digit, digit};
    let encrypted = rsa.encrypt(digit);

    println!("Encrypted: {}", encrypted);
    println!("Encrypted BE: {:?}", encrypted.to_bytes_be().as_slice());
    println!("Encrypted LE: {:?}", encrypted.to_bytes_le().as_slice());
}

fn decrypt(rsa: &mut RSAEncryptor) {
    let mut buf = String::default();
    println!("Insert line for encrypt: ");
    stdin().read_line(&mut buf).expect("Expected data");
    let res = buf.trim();
    let q: Vec<u32> = res.as_bytes().iter().map(|&e| e as u32).collect();
    let digit = BigUint::from_slice(q.as_ref());

    println!{"Parsed: {:?} {}", digit, digit};
    let encrypted = rsa.decrypt(digit);

    println!("Decrypted: {}", encrypted);
    println!("Decrypted LE: {:?}", encrypted.to_bytes_le().as_slice());
    println!("Decrypted BE: {:?}", encrypted.to_bytes_be().as_slice());
}


fn encrypt_file(rsa: &mut RSAEncryptor) {
    let mut buf = String::default();
    println!("Insert file for encrypt: ");
    stdin().read_line(&mut buf).expect("Expected data");
    let res = buf.trim();
    let path = Path::new(res);
    let display = path.display();

    let mut encr_buf = String::from(res);
    encr_buf.push_str("_encr");
    

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => {println!("couldn't open {}: {}", display, why); return}
        Ok(file) => file,
    };
    // Read the file contents into a string, returns `io::Result<usize>`
    let mut is_end = false;
    let mut new_file = File::create(Path::new(&encr_buf)).expect("Error in create encrypted file");
    while !is_end {
        let mut buffer = vec![0 as u8; rsa.get_n().to_bytes_be().len() - 1];
        let mut readed_size;
        if let Ok(size) = file.read(&mut buffer) {
            readed_size = size;
            println!("Readed {} bytes", size);
            if size == 0 {
                is_end = true;
                continue;
            }
        } else {
            println!("Error in read");
            is_end = true;
            return
        }
        let encr_content = rsa.encrypt(BigUint::from_bytes_le(&buffer[..readed_size]));
        //println!("{}", encr_content);
        let mut writable_content = encr_content.to_bytes_le();
        while readed_size == rsa.get_n().to_bytes_be().len() - 1 && writable_content.len() < rsa.get_n().to_bytes_be().len() {
            writable_content.push(0);
        }
        println!("Write {} bytes", writable_content.len());
        new_file.write_all(writable_content.as_slice());
    }
    new_file.sync_all();
}

fn decrypt_file(rsa: &mut RSAEncryptor) {
    let mut buf = String::default();
    println!("Insert file for decrypt: ");
    stdin().read_line(&mut buf).expect("Expected data");
    let res = buf.trim();
    let path = Path::new(res);
    let display = path.display();

    let mut encr_buf = String::from(res);
    encr_buf.push_str("_decr");
    

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => {println!("couldn't open {}: {}", display, why); return}
        Ok(file) => file,
    };
    let mut is_end = false;
    let mut new_file = File::create(Path::new(&encr_buf)).expect("Error in create encrypted file");
    let mut readed_size = 0;
    let mut writed_len = 0;
    while !is_end {
        // Read the file contents into a string, returns `io::Result<usize>`
        let mut buffer = vec![0 as u8; rsa.get_n().to_bytes_be().len()];
        if let Ok(size) = file.read(&mut buffer) {
            println!("Readed {} bytes", size);
            if size == 0 {
                is_end = true;
                continue;
            } else if readed_size == rsa.get_n().to_bytes_be().len() {
                let mut writable_content = Vec::<u8>::new();
                while writable_content.len() + writed_len < rsa.get_n().to_bytes_be().len() - 1 {
                    writable_content.push(0);
                }
                new_file.write_all(writable_content.as_slice());
            }
            readed_size = size;
        } else {
            println!("Error in read");
            is_end = true;
            return
        }
        let encr_content = rsa.decrypt(BigUint::from_bytes_le(&buffer[..readed_size]));
        //println!("{}", encr_content);
        let mut writable_content = encr_content.to_bytes_le();
        writed_len = writable_content.len();
        println!("Write {} bytes", writed_len);
        new_file.write_all(writable_content.as_slice());
    }
    new_file.sync_all();
}