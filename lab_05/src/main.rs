use std::io::stdin;
use std::str;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use hex_literal::hex;
use sha2::{Sha256, Sha512, Digest};

mod rsa_m;
use rsa_m::RSASigner;
use num_bigint::BigUint;

fn main() {
    let mut choose = 1;
    let mut buf = String::default();
    println!("Start");
    let mut rsa = RSASigner::new();
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
            1 => sign(&mut rsa),
            2 => check_sign(&mut rsa),
            _ => (),
        }
        println!("\n");
        buf = String::default();
    }
}

fn print_menu() {
    println!("0. Exit");
    println!("1. Sign file");
    println!("2. Check sign");
}


fn sign(rsa: &mut RSASigner) {
    let mut buf = String::default();
    println!("Insert file for sign: ");
    stdin().read_line(&mut buf).expect("Expected data");
    let res = buf.trim();
    let path = Path::new(res);
    let display = path.display();

    let mut encr_buf = String::from(res);
    encr_buf.push_str("_sign");
    

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => {println!("couldn't open {}: {}", display, why); return}
        Ok(file) => file,
    };
    
    let hash = count_hash(&file);

    let mut new_file = File::create(Path::new(&encr_buf)).expect("Error in create encrypted file");
    let sign = rsa.decrypt(hash.as_slice());
    new_file.write_all(sign.as_slice());
    new_file.sync_all();
}

fn check_sign(rsa: &mut RSASigner) {
    let mut buf = String::default();
    println!("Insert file for check sign: ");
    stdin().read_line(&mut buf).expect("Expected data");
    let res = buf.trim();
    let path = Path::new(res);
    let display = path.display();

    let mut encr_buf = String::from(res);
    encr_buf.push_str("_sign");
    

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => {println!("couldn't open {}: {}", display, why); return}
        Ok(file) => file,
    };

    let hash = count_hash(&file);

    let mut file = match File::open(&encr_buf) {
        Err(why) => {println!("couldn't open file with sign {}: {}", display, why); return}
        Ok(file) => file,
    };

    let mut buffer = [0 as u8; 1024];
    let count = file.read(&mut buffer).unwrap();

    let hash_from_sign = rsa.encrypt(&buffer[..count]);

    if hash_from_sign == hash.as_slice() {
        println!("Correct sign!");
    } else {
        println!("Incorrect sign!");
    }
}

fn count_hash(file: &File) -> Vec<u8> {
    sha256_digest(file)
}

fn sha256_digest<R: Read>(mut reader: R) -> Vec<u8> {
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer).unwrap();
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }

    hasher.finalize().to_vec()
}