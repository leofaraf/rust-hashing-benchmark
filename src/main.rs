use std::time::Instant;

use argon2::{password_hash::{rand_core::OsRng, SaltString}, PasswordHasher};

macro_rules! algo_benchmark {
    ($name:literal, || $process:expr) => {
        println!();
        println!("Algorithm: {}", $name);
        let start = Instant::now();
        
        $process
        
        println!("Benchmark result: {}ms", start.elapsed().as_millis());
        println!();
    }
}

fn main() {
    algo_benchmark!("bcrypt", || {
        let _ = pwhash::bcrypt::hash("test");
    });
    
    algo_benchmark!("unix", || {
        let _ = pwhash::unix::crypt("test", "test").unwrap();
    });

    algo_benchmark!("sha256 (DEPRECATED)", || {
        let _ = pwhash::sha256_crypt::hash("test");
    });

    algo_benchmark!("sha512", || {
        let _ = pwhash::sha512_crypt::hash("test");
    });

    algo_benchmark!("sha1", || {
        let _ = pwhash::sha1_crypt::hash("test");
    });

    algo_benchmark!("md5 (DEPRECATED)", || {
        let _ = pwhash::md5_crypt::hash("test");
    });

    algo_benchmark!("bsdi (DEPRECATED)", || {
        let _ = pwhash::bsdi_crypt::hash("test");
    });

    algo_benchmark!("argon2 (hash_password)", || {
        let _ = 
            argon2::Argon2::default().hash_password(b"test", &SaltString::generate(&mut OsRng)).unwrap();
    });
}