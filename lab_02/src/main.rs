use std::io::stdin;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod enigma;
use enigma::Enigma;

fn main() {
    let mut choose = 1;
    let mut buf = String::default();
    let mut enigma = Enigma::new();
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
            1 => call_set_rotors(&mut enigma),
            2 => encrypt(&mut enigma),
            3 => encrypt_file(&mut enigma),
            _ => (),
        }
        println!("\n");
        buf = String::default();
    }
}

fn print_menu() {
    println!("0. Exit");
    println!("1. Set rotors");
    println!("2. Encrypt string");
    println!("3. Encrypt file");
}

fn call_set_rotors(enigma: &mut Enigma) {
    let mut buf = String::default();

    println!("Insert start symbol for rotor 1: ");
    stdin().read_line(&mut buf).expect("Expected choose");
    let res = buf.trim();
    buf = String::from(res);
    let chars = buf.chars();
    let first = chars.collect::<Vec<char>>()[0];

    let mut buf = String::default();
    println!("Insert start symbol for rotor 2: ");
    stdin().read_line(&mut buf).expect("Expected choose");
    let res = buf.trim();
    buf = String::from(res);
    let chars = buf.chars();
    let second = chars.collect::<Vec<char>>()[0];

    let mut buf = String::default();
    println!("Insert start symbol for rotor 3: ");
    stdin().read_line(&mut buf).expect("Expected choose");
    let res = buf.trim();
    buf = String::from(res);
    let chars = buf.chars();
    let third = chars.collect::<Vec<char>>()[0];

    enigma.set_rotors(first as u8, second as u8, third as u8);
}

fn encrypt(enigma: &mut Enigma) {
    let mut buf = String::default();
    println!("Insert line for encrypt: ");
    stdin().read_line(&mut buf).expect("Expected data");
    let res = buf.trim();
    buf = String::from(res);
    let encrypted = enigma.encrypt(buf);
    println!("Encrypted: {}", encrypted);
}

fn encrypt_file(enigma: &mut Enigma) {
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
    let mut buffer = Vec::<u8>::new();
    if let Ok(size) = file.read_to_end(&mut buffer) {
        println!("Read {} bytes", size);
    } else {
        println!("Error in read");
        return
    }
    let encr_content = enigma.encrypt_file(&buffer);
    //println!("{}", encr_content);
    let mut new_file = File::create(Path::new(&encr_buf)).expect("Error in create encrypted file");
    new_file.write_all(encr_content.as_ref());
    new_file.sync_all();
}