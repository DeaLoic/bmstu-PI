use num_bigint::{BigUint, BigInt, ToBigUint};
use primes::PrimeSet;
use rand::{Rng};
use rsa::{PublicKey, RSAPrivateKey, RSAPublicKey, PaddingScheme};
use rand::rngs::OsRng;

pub struct RSASigner {
    private_key: RSAPrivateKey,
    public_key: RSAPublicKey,
    rng: OsRng,
}

impl RSASigner {
    pub fn new() -> Self {
        let mut rng = OsRng;
        let bits = 1024;
        let private_key = RSAPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let public_key = RSAPublicKey::from(&private_key);
        Self {
            private_key,
            public_key,
            rng,
        }

    }

    pub fn encrypt(&mut self, data: &[u8]) -> Vec<u8> {
        let padding = PaddingScheme::new_pkcs1v15_encrypt();
        println!("{:?}", data);
        self.private_key.decrypt(padding, data).expect("failed to decrypt")
    }
    
    pub fn decrypt(&mut self, enc_data: &[u8]) -> Vec<u8> {
        let padding = PaddingScheme::new_pkcs1v15_encrypt();
        self.public_key.encrypt(&mut self.rng, padding, enc_data).expect("failed to encrypt")
    }
}