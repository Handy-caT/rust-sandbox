use std::fs::File;
use std::io;
use argon2::{Argon2, Error, PasswordHash, PasswordHasher};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use rand::{Rng, SeedableRng};
use sha3::{Digest, Sha3_256};
use sha3::digest::Output;

fn generate_password(length: u8, charset: &[u8]) -> String {

    let mut password = String::new();

    for _ in 0..length {
        let index = rand::random::<usize>() % charset.len();
        password.push(charset[index] as char);
    }

    password
}

fn select_rand_val<T>(slice: &[T]) -> &T {
    let index = rand::random::<usize>() % slice.len();
    &slice[index]
}

fn new_access_token() -> String {
    let charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                  abcdefghijklmnopqrstuvwxyz\
                  0123456789";

    let mut rng = rand_hc::Hc128Rng::from_entropy();
    let mut token = String::new();

    for _ in 0..32 {
        let index = rng.gen::<usize>() % charset.len();
        token.push(charset[index] as char);
    }

    token
}

fn get_file_hash(filename: &String) -> Output<Sha3_256>{
    let mut file = File::open(filename).unwrap();
    let mut hasher = Sha3_256::new();
    io::copy(&mut file, &mut hasher).unwrap();
    let hash = hasher.finalize();

    hash
}

fn hash_password<'a>(password: &String) -> Result<String, ()>{
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    let password = password.as_bytes();
    let password_hash = argon2.hash_password(password, &salt);

    match password_hash {
        Ok(hash) => {
            Ok(hash.to_string())
        }
        Err(_) => {
            Err(())
        }
    }


}

fn main() {
   let charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                  abcdefghijklmnopqrstuvwxyz\
                  0123456789)(*&^%$#@!~";
    let password = generate_password(16, charset);
    println!("{}", password);

    let slice = &[1, 2, 3, 4, 5];
    let val = select_rand_val(slice);
    println!("{}", val);

    for _ in 0..10 {
        let token = new_access_token();
        println!("{}", token);
    }

    let filename = String::from("/Users/maksim/Documents/Gapopa/rust-sandbox/3_ecosystem/3_7_rand_crypto/test.txt");
    let hash = get_file_hash(&filename);
    println!("{:x}", hash);

    let pass_hash = hash_password(&password).unwrap();
    println!("{}", pass_hash)
}
