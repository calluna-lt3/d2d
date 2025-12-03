use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use std::{panic, str, vec};

#[derive(Debug)]
enum Token {
    Increment,
    Skip,
    Whitespace,
}

// this tokenizer is GREEDY !!
// it will read like so: abc > ab > a
fn tokenize(
    input: &mut Vec<&str>,
    increment_keywords: Vec<&str>,
    skip_keywords: Vec<&str>,
) -> Vec<Token> {
    let mut tokens_out: Vec<Token> = vec![];
    let mut incr_lens: Vec<usize> = increment_keywords.iter().map(|x| x.len()).collect();
    incr_lens.sort_by(|a, b| (a - b).cmp(a));
    let mut skip_lens: Vec<usize> = skip_keywords.iter().map(|x| x.len()).collect();
    skip_lens.sort_by(|a, b| (a - b).cmp(a));

    let mut words = input.into_iter();
    while let Some(word) = words.next() {
        let mut word_bytes = word.as_bytes();
        while word_bytes.len() > 0 {
            let mut valid = false;

            // check for incrementers
            for len in &incr_lens {
                let bytes_left = word_bytes.len();
                if *len > bytes_left || bytes_left == 0 {
                    continue;
                }

                let cur_index = &word_bytes[..*len];
                let cur_word = str::from_utf8(cur_index).unwrap();
                if increment_keywords.contains(&cur_word) {
                    valid = true;
                    word_bytes = &word_bytes[*len..];
                    tokens_out.push(Token::Increment);
                    break;
                }
            }

            // check for skips
            for len in &skip_lens {
                let bytes_left = word_bytes.len();
                if *len > bytes_left || bytes_left == 0 {
                    continue;
                }

                let cur_index = &word_bytes[..*len];
                let cur_word = str::from_utf8(cur_index).unwrap();

                if skip_keywords.contains(&cur_word) {
                    valid = true;
                    word_bytes = &word_bytes[*len..];
                    tokens_out.push(Token::Skip);
                    break;
                }
            }

            if !valid {
                let mut unrecognized_chars = String::new();
                word_bytes.read_to_string(&mut unrecognized_chars).unwrap();
                panic!("ERROR: Unrecognized sequence of characters: `{unrecognized_chars}`");
            }
        }

        tokens_out.push(Token::Whitespace);
    }

    tokens_out
}

fn interpret(tokens: Vec<Token>) {
    let mut values: Vec<u8> = vec![0];
    let mut index: usize = 0;
    let mut scalar = 1;
    tokens.iter().for_each(|token| {
        match token {
            Token::Increment => {
                values[index] += scalar;

                let val = scalar.checked_mul(2);
                if let Some(x) = val {
                    scalar = x;
                } else {
                    panic!("ERROR: pls dont increment that much ^-^ max value is {}", u8::MAX);
                }
            },
            Token::Skip => {
                let val = scalar.checked_mul(2);
                if let Some(x) = val {
                    scalar = x;
                } else {
                    panic!("ERROR: pls dont increment that much ^-^ max value is {}", u8::MAX);
                }
            },
            Token::Whitespace => {
                scalar = 1;
                index += 1;
                values.push(0);
            }
        }
    });

    println!("{values:?}");
    values.iter().for_each(|x| {
        print!("{}", char::from(*x));
        println!("");
    });
}


fn main() -> std::io::Result<()> {
    let filename = "main.d2d";
    let mut file = File::open(filename)?;

    let mut file_buffer = String::new();
    let _ = file.read_to_string(&mut file_buffer).unwrap();

    let words: VecDeque<&str> = file_buffer.split_whitespace().collect();

    let mut incr_keywords: Vec<&str> = vec![];
    let mut skip_keywords: Vec<&str> = vec![];
    let mut incr = true;

    // get incr/skip keywords
    let mut words = words.into_iter();
    while let Some(word) = words.next() {
        if word.eq_ignore_ascii_case("good") {
            words.next();
            break;
        }

        if incr {
            let len = word.len();
            let bytes = word.as_bytes();
            let last = bytes[len - 1];
            match last {
                b',' | b'.' => {
                    incr_keywords.push(str::from_utf8(&bytes[..(len - 1)]).unwrap());
                    incr = false;
                }
                _ => incr_keywords.push(word),
            }
        } else {
            if incr_keywords.contains(&word) {
                panic!("ERROR: `{word}` cannot increment and skip at the same time.");
            }
            skip_keywords.push(word);
        }
    }

    let mut words: Vec<&str> = words.collect();
    let tokens = tokenize(&mut words, incr_keywords, skip_keywords);
    interpret(tokens);

    Ok(())
}
