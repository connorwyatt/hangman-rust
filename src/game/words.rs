use crate::game::allowed_letters::ALLOWED_LETTER_RANGE;
use rand::{distributions::Uniform, thread_rng, Rng};

pub(crate) fn random_word(minimum_word_length: usize) -> String {
    let mut rng = thread_rng();

    let words = read_words(minimum_word_length);

    let index = rng.sample(Uniform::new(0, words.len()));

    words[index].clone().to_string()
}

fn read_words(minimum_word_length: usize) -> Vec<String> {
    let str = include_str!("words.txt");

    str.split(&['\r', '\n'][..])
        .filter(|line| line.len() >= minimum_word_length)
        .map(|line| line.trim().to_uppercase())
        .filter(|line| line.chars().all(|x| ALLOWED_LETTER_RANGE.contains(&x)))
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn getting_a_random_word_returns_a_word() {
        assert!(!random_word(4).is_empty());
    }
}
