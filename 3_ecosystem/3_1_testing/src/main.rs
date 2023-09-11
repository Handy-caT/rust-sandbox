use std::{cmp::Ordering, env, io};
use mockall::automock;

#[automock]
trait SecretNumberArgGetter {
    fn get_secret_number_arg(&self) -> String;
}

struct SecretNumberEnv {}
impl SecretNumberArgGetter for SecretNumberEnv {
    fn get_secret_number_arg(&self) -> String {
        env::args()
            .skip(1)
            .take(1)
            .last()
            .expect("No secret number is specified")
    }
}

#[automock]
trait GuessNumberGetter {
    fn get_guess_number(&self) -> String;
}

struct GuessNumberStdin {}
impl GuessNumberGetter for GuessNumberStdin {
    fn get_guess_number(&self) -> String {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        guess
    }
}


fn main() {
    let number_getter = SecretNumberEnv {};
    let guess_getter = GuessNumberStdin {};

    println!("Guess the number!");

    let secret_number = get_secret_number(&number_getter);

    loop {
        let is_win = guess_loop(secret_number, &guess_getter);
        if is_win {
            break;
        }
    }

}

fn guess_loop<G: GuessNumberGetter>(secret_number: u32, guess_getter: &G) -> bool {
    println!("Please input your guess.");

    let guess = match get_guess_number(guess_getter) {
        Some(n) => n,
        _ => return false,
    };

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => {
            println!("Too small!");
            false
        },
        Ordering::Greater => {
            println!("Too big!");
            false
        },
        Ordering::Equal => {
            println!("You win!");
            true
        }
    }
}

fn get_secret_number<G: SecretNumberArgGetter>(getter: &G) -> u32 {
    let secret_number = getter.get_secret_number_arg();
    secret_number
        .trim()
        .parse()
        .ok()
        .expect("Secret number is not a number")
}

fn get_guess_number<G: GuessNumberGetter>(getter: &G) -> Option<u32> {
    let mut guess = getter.get_guess_number();
    guess.trim().parse().ok()
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use crate::{MockGuessNumberGetter, MockSecretNumberArgGetter};

    proptest! {
        #[test]
        fn test_get_secret_number_positive_number(v in any::<u32>().prop_map(|v| v.to_string())) {
            let mut mock = MockSecretNumberArgGetter::new();
            let result = v.parse::<u32>().unwrap();
            mock.expect_get_secret_number_arg()
                .returning(move || v.clone());

            assert_eq!(crate::get_secret_number(&mock), result);
        }

        #[test]
        fn test_get_guess_number_positive_number(v in any::<u32>().prop_map(|v| v.to_string())) {
            let mut mock = MockGuessNumberGetter::new();
            let result = v.parse::<u32>().unwrap();
            mock.expect_get_guess_number()
                .returning(move || v.clone());

            assert_eq!(crate::get_guess_number(&mock), Some(result));
        }
    }

    #[test]
    #[should_panic(expected = "Secret number is not a number")]
    fn test_get_secret_number_negative_number() {
        let mut mock = MockSecretNumberArgGetter::new();
        mock.expect_get_secret_number_arg()
            .returning(move || String::from("-1"));

        crate::get_secret_number(&mock);
    }

    #[test]
    #[should_panic(expected = "Secret number is not a number")]
    fn test_get_secret_number_not_a_number() {
        let mut mock = MockSecretNumberArgGetter::new();
        mock.expect_get_secret_number_arg()
            .returning(move || String::from("String"));

        crate::get_secret_number(&mock);
    }

    #[test]
    fn test_get_guess_number_negative_number() {
        let mut mock = MockGuessNumberGetter::new();
        mock.expect_get_guess_number()
            .returning(move || String::from("-1"));

        assert_eq!(crate::get_guess_number(&mock), None);
    }

    #[test]
    fn test_get_guess_number_not_a_number() {
        let mut mock = MockGuessNumberGetter::new();
        mock.expect_get_guess_number()
            .returning(move || String::from("String"));

        assert_eq!(crate::get_guess_number(&mock), None);
    }

    #[test]
    fn test_guess_loop_bigger() {
        let mut mock = MockGuessNumberGetter::new();
        mock.expect_get_guess_number()
            .returning(move || String::from("2"));

        assert!(!crate::guess_loop(1, &mock));
    }

    #[test]
    fn test_guess_loop_smaller() {
        let mut mock = MockGuessNumberGetter::new();
        mock.expect_get_guess_number()
            .returning(move || String::from("0"));

        assert!(!crate::guess_loop(1, &mock));
    }

    #[test]
    fn test_guess_loop_equal() {
        let mut mock = MockGuessNumberGetter::new();
        mock.expect_get_guess_number()
            .returning(move || String::from("1"));

        assert!(crate::guess_loop(1, &mock));
    }

    #[test]
    fn test_guess_loop_not_a_number() {
        let mut mock = MockGuessNumberGetter::new();
        mock.expect_get_guess_number()
            .returning(move || String::from("String"));

        assert!(!crate::guess_loop(1, &mock));
    }

}

