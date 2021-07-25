use std::error::Error;
use xorf::Filter;
use dialetto::HP;


// fn insert(word: &str, filter: &Xor8) {
//     let mut h = FnvHasher::default();
//     h.write(word.as_bytes());
//     let i = h.finish();

//     filter.

// }


fn main() -> Result<(), Box<dyn Error>> {
    let filter = dialetto::make_filter("en_full.txt")?;

    word_check("this", &filter);
    word_check("bonjour", &filter);  // Really need to clean this wordlist
    word_check("alabaster", &filter);
    word_check("catastrophe", &filter);
    word_check("aklw4relkhgh", &filter);
    word_check("a", &filter);
    word_check("ab", &filter);
    word_check("aba", &filter);
    word_check("abab", &filter);
    word_check("ababa", &filter);
    word_check("ababt", &filter);
    word_check("ababababab", &filter);
    word_check("caa", &filter);
    word_check("cac", &filter);
    word_check("xx", &filter);

    dialetto::encode_to_file(&filter, "encoded.bin")?;

    // Test that it loads
    let decoded = dialetto::decode_from_file("encoded.bin")?;
    println!("{}", decoded.contains(&"bonjour".to_string()));

    Ok(())
}

fn word_check(word: &str, filter: &HP) {
    println!("Is \"{}\" found? {:?}", word, &filter.contains(&word.to_string()));

}
