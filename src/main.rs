use std::fs::File;
use std::io::Read;

#[derive(Debug)]
enum Token {
    Increment,
    Skip,
}

fn tokenize(file: &mut File, token_types: Vec<String>) -> Vec<Token> {
    let mut tokens_out: Vec<Token> = vec![];
    let token_types_lens: Vec<usize> = token_types.iter().map(|x| x.len()).collect();

    let mut file_read_buffer = [0; 256];
    let _ = file.read(&mut file_read_buffer).unwrap();
    let mut file_contents = String::new();
    file_read_buffer
        .as_slice()
        .read_to_string(&mut file_contents)
        .unwrap();

    // check the start of the input stream, check the word length, see if they match
    for token_info in token_types.iter().zip(token_types_lens.into_iter()) {
        let (token, token_len) = (token_info.0, token_info.1);
        let sl = &file_read_buffer[0..token_len];
        let sl = str::from_utf8(sl).unwrap();
        // file_read_buffer => [token_len..]
        let val = token.eq_ignore_ascii_case(sl);
        println!("{token} = {sl} ? {val}");
    }

    vec![]
}

fn main() -> std::io::Result<()> {
    let filename = "main.d2d";
    let mut file = File::open(filename)?;

    let token_types: Vec<String> = vec![String::from("woof"), String::from("awruff")];
    let res = tokenize(&mut file, token_types);
    println!("res = {res:?}");

    Ok(())
}
