use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::rand_core::SeedableRng;
use argon2::password_hash::{Salt, SaltString};
use rand::distributions::Alphanumeric;
use rand::Rng;
use secrecy::{ExposeSecret, SecretString};

const SALT_LENGTH: u8 = 16;

pub fn generate_salt(length: u8) -> String {
    let mut rng = rand_hc::Hc128Rng::from_entropy();
    let mut salt = String::new();

    for _ in 0..length {
        let char = rng.sample(Alphanumeric) as char;
        salt.push(char);
    }

    salt
}

pub fn hash_password<S: AsRef<str>>(password: S) -> Result<String, ()> {
    let password = password.as_ref();

    let argon2 = Argon2::default();
    let salt_string = generate_salt(SALT_LENGTH);
    let salt = Salt::try_from(salt_string.as_str());
    if salt.is_err() {
        return Err(())
    }
    let salt = salt.unwrap();

    let hash = argon2.hash_password(password.as_bytes(), salt);
    match hash {
        Ok(hash) => {
            Ok(hash.to_string())
        }
        Err(_) => {
            Err(())
        }
    }
}

pub fn validate_password_hash<S: AsRef<str>>(password: S, hash: S) -> bool {
    let argon2 = Argon2::default();
    let hash = PasswordHash::try_from(hash.as_ref()).unwrap();
    let res = argon2.verify_password(password.as_ref().as_bytes(), &hash);
    match res {
        Ok(_) => {
            true
        }
        Err(_) => {
            false
        }
    }
}