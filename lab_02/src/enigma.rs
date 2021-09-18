std::char::{char};



pub struct Enigma {
    rotors: Rotors,
    panel: Panel,
}

impl Enigma {
    pub fn new() -> Self {
        Self {
            rotors: Rotors::new(),
            panel: Panel::new(),
        }
    }

    pub fn set_rotors(&self, first: char, second: char, third: char) {
        self.rotors.set(first, second, third);
    }

    pub fn add_comutate(&self, first: char, second: char) -> bool {
        self.panel.add_comutate(first, second)
    }

    pub fn encrypt(&self, message: String) -> String {
        let encrypted_message = String::default();
        for char_to_en in message.chars() {
            encrypted_message.add(self.encrypt_letter(char_to_en).to_str());
        }
        encrypted_message
    }

    fn encrypt_letter(&self, letter: char) -> char {
        let mut encrypted_letter = self.panel.swap(letter);
        encrypt_letter = self.rotors.encrypt();
        self.panel.swap(encrypt_letter)
    }
}

pub struct Rotors {
    rotors: Vec<Rotor>,
    reflector: Rotor,
}

impl Rotors {
    pub fn new() -> Self {
        let rotors = Vec::new();
        rotors.add(Rotor::new());
        rotors.add(Rotor::new());
        rotors.add(Rotor::new());

        Self {
            rotors,
            reflector: Rotor::new(),
        }
    }

    pub fn encrypt(&self, letter: char) -> char {
        let mut encrypted_letter = letter;
        for rotor in self.rotors {
            encrypted_letter = rotor.encrypt(encrypted_letter);
        }
        encrypted_letter = self.reflector.encrypt(encrypted_letter);
        for rotor in self.rotors.reverse() {
            encrypted_letter = rotor.encrypt(encrypted_letter);
        }
        encrypted_letter
    }
}