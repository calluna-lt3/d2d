use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use std::{panic, str, vec};

#[derive(Debug)]
enum Token {
    Increment,
    Skip,
}

//

fn tokenize(input: &mut Vec<&str>, increment_keywords: Vec<&str>, skip_keywords: Vec<&str>) -> Vec<Token> {
    let mut tokens_out: Vec<Token> = vec![];
    let token_types_lens: Vec<usize> = increment_keywords.iter().map(|x| x.len()).collect();

    tokens_out
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
        // TODO: case insensitive
        if word == "good" {
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

    let words: Vec<&str> = words.collect();

    println!("words: {:?}", words);
    println!("incr: {:?}", incr_keywords);
    println!("skip: {:?}", skip_keywords);

    Ok(())
}
