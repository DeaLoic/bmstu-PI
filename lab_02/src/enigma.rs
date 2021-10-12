use std::char;
use rand::prelude::*;


pub struct Enigma {
    rotors: Rotors,
}

impl Enigma {
    pub fn new() -> Self {
        Self {
            rotors: Rotors::new(),
        }
    }

    pub fn set_rotors(&mut self, first: u8, second: u8, third: u8) {
        self.rotors.set(first, second, third);
    }

    pub fn encrypt(&mut self, message: String) -> String {
        let mut encrypted_message = String::default();
        for char_to_en in message.chars() {
            encrypted_message.push(self.encrypt_letter(char_to_en as u8) as char);
        }
        encrypted_message
    }

    pub fn encrypt_file(&mut self, message: &Vec::<u8>) -> Vec::<u8> {
        let mut encrypted_message: Vec::<u8> = Vec::new();
        let mut i = 0;
        for byte in message {
            if i % 10000 == 0 {
                println!("{}", i);
            }
            i+=1;
            encrypted_message.push(self.encrypt_letter(*byte));
        }
        encrypted_message
    }

    fn encrypt_letter(&mut self, letter: u8) -> u8 {
        self.rotors.encrypt(letter)
    }
}

#[derive(Debug)]
pub struct Rotors {
    rotors: Vec<Rotor>,
    reflector: Reflector,
}

impl Rotors {
    pub fn new() -> Self {
        let mut rotors = Vec::new();
        rotors.push(Rotor::new());
        rotors.push(Rotor::new());
        rotors.push(Rotor::new());

        Self {
            rotors,
            reflector: Reflector::new(),
        }
    }

    pub fn set(&mut self, first: u8, second: u8, third: u8) {
        self.rotors[0].set(first as u8);
        self.rotors[1].set(second as u8);
        self.rotors[2].set(third as u8);
    }

    pub fn encrypt(&mut self, letter: u8) -> u8 {
        let mut encrypted_letter = letter as u8;

        // print!("Encrypt letter: {}; ", encrypted_letter);
        for rotor in &self.rotors {
            encrypted_letter = rotor.forward(encrypted_letter);
        }
        // print!("Encrypt letter st1: {}; ", encrypted_letter);
        encrypted_letter = self.reflector.reflect(encrypted_letter);
        // print!("Encrypt letter refl: {}; ", encrypted_letter);
        self.rotors.reverse();
        for rotor in &self.rotors {
            encrypted_letter = rotor.backward(encrypted_letter);
        }
        self.rotors.reverse();
        // print!("Encrypt letter st2: {}; ", encrypted_letter);

        for rotor in &mut self.rotors {
            if !rotor.click() {
                break;
            }
        }
        encrypted_letter
    }
}

#[derive(Debug)]
pub struct Reflector {
}

impl Reflector {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn reflect(&self, letter: u8) -> u8 {
        if letter % 2 == 0 {
            letter + 1
        } else {
            letter - 1
        }
    }
}

#[derive(Debug)]
pub struct Rotor {
    vecr: Vec<u8>,
    start: u8,
    current: u8,
}

impl Rotor {
    pub fn new() -> Self {
        let mut vecr: Vec<u8> = Vec::new();
        for i in 0..=255 {
            vecr.push(i);
        }

        let mut rng = rand::thread_rng();
        vecr.shuffle(&mut rng);

        Self {
            vecr,
            start: 0,
            current: 0,
        }
    }

    pub fn set(&mut self, start: u8) {
        self.start = start;
        self.current = start;
    }

    pub fn forward(&self, encrypt: u8) -> u8 {
        self.vecr[(encrypt as usize + self.current as usize) % (u8::MAX as usize + 1)]
    }

    pub fn backward(&self, encrypt: u8) -> u8 {
        let mut res = 0;
        for i in 0..=255 {
            if self.vecr[(i + self.current as usize) % (u8::MAX as usize + 1)] == encrypt {
                res = i;
                break;
            }
        }
        return res as u8;

    }

    pub fn click(&mut self) -> bool {
        self.current = ((self.current as u32 + 1 as u32) % (u8::MAX as u32 + 1)) as u8;
        self.current == self.start
    }
}