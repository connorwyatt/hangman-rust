use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

pub(crate) fn get_random_word() -> String {
    let mut rng = thread_rng();

    let index = (&mut rng).sample(Uniform::new(0, WORDS.len()));

    WORDS[index].clone().to_string()
}

const WORDS: [&str; 4] = ["rust", "rustacean", "cargo", "crate"];
