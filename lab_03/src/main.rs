use std::io::stdin;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod aes;
use aes::Encryptor;
use num_bigint::{BigUint};
use num_traits::cast::ToPrimitive;

fn main() {
    let mut choose = 1;
    let mut buf = String::default();
    let mut encryptor = Encryptor::new("masecrkey123456X");
    while choose != 0 {
        print_menu();
        stdin().read_line(&mut buf).expect("Expected choose");
        let res = buf.trim();
        buf = String::from(res);
        let res_choose = buf.parse();
        if let Err(res) = res_choose {
            println!("{} Expected choose. Try again\n", res);
            buf = String::default();
            continue;
        }

        choose = res_choose.unwrap();
        match choose {
            1 => encrypt_file(&mut encryptor),
            2 => decrypt_file(&mut encryptor),
            _ => (),
        }
        println!("\n");
        buf = String::default();
    }
}

fn print_menu() {
    println!("0. Exit");
    println!("1. Encrypt file");
    println!("2. Decrypt file");
}

fn encrypt_file(encryptor: &mut Encryptor) {
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
        let mut buffer = vec![0 as u8; 16];
        let mut readed_size;
        if let Ok(size) = file.read(&mut buffer) {
            readed_size = size;
            println!("Readed {} bytes", size);
            if size != 16 {
                is_end = true;
            }
        } else {
            println!("Error in read");
            is_end = true;
            return
        }
        let mut cur_size = readed_size;
        while cur_size < 16 {
            buffer[cur_size] = 0;
            cur_size += 1;
        }
        let encr_content = aes::encrypt_AES128(encryptor, &buffer[..cur_size]);
        println!("Write {} bytes", encr_content.len());
        new_file.write_all(encr_content.as_slice());
        if is_end {
            let last_chunk_size = BigUint::from(readed_size);
            new_file.write_all(last_chunk_size.to_bytes_le().as_slice());
        }
    }
    new_file.sync_all();
}

fn decrypt_file(encryptor: &mut Encryptor) {
    let mut buf = String::default();
    println!("Insert file for encrypt: ");
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
    // Read the file contents into a string, returns `io::Result<usize>`
    let mut is_end = false;
    let mut new_file = File::create(Path::new(&encr_buf)).expect("Error in create encrypted file");
    let mut last_writable: Vec<u8> = Vec::default();
    while !is_end {
        let mut buffer = vec![0 as u8; 16];
        let mut readed_size;
        if let Ok(size) = file.read(&mut buffer) {
            readed_size = size;
            println!("Readed {} bytes", size);
            if size != 16 {
                is_end = true;
                let last_chunk_size = BigUint::from_bytes_le(&buffer[..size]);
                new_file.write_all(&last_writable.as_slice()[..last_chunk_size.to_usize().unwrap()]);
            } else {
                if last_writable.len() != 0 {
                    println!("Writed {} bytes", last_writable.len());
                    new_file.write_all(last_writable.as_slice());
                }
            }
            if is_end {
            }
        } else {
            println!("Error in read");
            is_end = true;
            return
        }
        let encr_content = aes::decrypt_AES128(encryptor, &buffer);
        last_writable = encr_content;
    }
    new_file.sync_all();
}