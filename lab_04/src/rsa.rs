use num_bigint::{BigUint, BigInt, ToBigUint};
use primes::PrimeSet;
use rand::{Rng};

pub struct RSAEncryptor {
    private_key: BigUint,
    pub_key: BigUint,
    n: BigUint,
}

impl RSAEncryptor {
    pub fn new() -> Self {
        println!("Start");
        let (q, p) = Self::get_primes();

        println!("Primes: {} {}", p, q);
        let phi = Self::eiler(q.clone(), p.clone());

        println!("Phi: {}", phi);

        let n = p * q; // длина алфавита
        println!("N: {}", n);
        println!("N: {:?}", n.to_bytes_le());
        println!("N len: {}", n.to_bytes_le().len());

        let pub_key = Self::compute_public_key(phi.clone(), n.clone());
        println!("Public key: {}", pub_key);

        let private_key = Self::compute_private_key(pub_key.clone(), phi.clone());

        println!("Private key: {}", private_key);

        Self {
            private_key,
            pub_key,
            n
        }
    }

    pub fn get_n(&self) -> &BigUint {
        &self.n
    }
    pub fn encrypt(&self, message: BigUint) -> BigUint {
        return message.modpow(&self.pub_key, &self.n);
    }


    pub fn decrypt(&self, message: BigUint) -> BigUint {
        return message.modpow(&self.private_key, &self.n);
    }

    fn get_primes() -> (BigUint, BigUint) {
        let mut pset = PrimeSet::new();
        let mut rng = rand::thread_rng();

        let seed = (rng.gen::<f32>() * 100_000_000.0) as u64;
        println!("Seed: {}", seed);
        let (ix, n) = pset.find(seed);
        let (ix, n2) = pset.find(n + (rng.gen::<f32>() * 500.0) as u64);
        (BigUint::from(n), BigUint::from(n2))
    }

    fn eiler(first: BigUint, second: BigUint) -> BigUint {
        (first - BigUint::from(1 as u32)) * (second - BigUint::from(1 as u32))
    }

    fn compute_public_key(phi: BigUint, n: BigUint) -> BigUint {
        let mut nod = BigUint::from(0 as u32);
        let mut num = phi.clone() - BigUint::from(1 as u32);
        while nod != BigUint::from(1 as u32) && num < phi {
            num -= BigUint::from(1 as u32);
            nod = Self::nod(phi.clone(), num.clone());
        }
        return num;
    }

    fn compute_private_key(pub_key: BigUint, phi: BigUint) -> BigUint {
        let mut t = BigInt::from(0 as u32);
        let mut r = BigInt::from(phi.clone()); // остаток от деления
        let mut newt = BigInt::from(1 as u32);
        let mut newr = BigInt::from(pub_key.clone());

        while newr != BigInt::from(0 as u32) {
            let quotient = r.clone() / newr.clone();
            let tmpt = newt.clone();
            newt = t.clone() - quotient.clone() * newt.clone();
            t = tmpt.clone();

            let tmpr = newr.clone();
            newr = r.clone() - quotient.clone() * newr.clone();
            r = tmpr.clone();
        }

        if t < BigInt::from(0 as u32) {
            t += BigInt::from(phi.clone());
        }

        return t.to_biguint().unwrap();
    }

    fn nod(first: BigUint, second: BigUint) -> BigUint {
        let mut first = first;
        let mut second = second;
    
        if second > first {
            let q = second.clone();
            second = first.clone();
            first = q;
        }
        let mut r = BigUint::from(1 as u32);
    
        while second > BigUint::from(0 as u32) {
            r = first.clone() % second.clone();
            first = second.clone();
            second = r.clone();
        }
    
        first
    }
}