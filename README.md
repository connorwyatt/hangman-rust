# Hangman

First program written in Rust.

## Rules

- You get 9 lives.
- You will be shown a blanked out word and asked to guess a letter from the word.
- If your guess is incorrect, you lose a life.
- If your lives hit 0, you lose the game.
- If you guess all the letters in the word, you win the game.

## Playing the game

First, you must have Rust installed.

Next, build the executable:

```
cargo build --release
```

Finally, run the executable from `./target/release`. It will be called `hangman`.

_Note: the extension for the executable will be dependent on your platform._

## Using a custom word list

You can replace the [words list](./src/game/words.txt) with a custom words list.

_Note: you will need to rebuild the game to see this take effect._

### Example lists

- [https://www.mit.edu/~ecprice/wordlist.10000](https://www.mit.edu/~ecprice/wordlist.10000)
