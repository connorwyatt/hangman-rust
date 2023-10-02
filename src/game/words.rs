use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

pub(crate) fn get_random_word() -> String {
    let mut rng = thread_rng();

    let words = read_words();

    let index = rng.sample(Uniform::new(0, words.len()));

    words[index].clone().to_string()
}

const MINIMUM_WORD_LENGTH: i8 = 4;

fn read_words() -> Vec<String> {
    let str = include_str!("words.txt");

    str.split(&['\r', '\n'][..])
        .map(|line| line.trim().to_string())
        .filter(|line| line.len() >= (MINIMUM_WORD_LENGTH as usize))
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn getting_a_random_word_returns_a_word() {
        assert!(!get_random_word().is_empty());
    }
}
