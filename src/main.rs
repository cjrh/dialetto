use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;
use std::iter::FromIterator;

use fnv::FnvHasher;

use std::collections::hash_map::DefaultHasher;
use xorf::{Filter, HashProxy, Xor8, Xor32, Xor16 as Filt};

// fn insert(word: &str, filter: &Xor8) {
//     let mut h = FnvHasher::default();
//     h.write(word.as_bytes());
//     let i = h.finish();

//     filter.

// }

type HP = HashProxy<String, FnvHasher, Filt>;


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let f = File::open("en_full.txt")?;
    let tokens = BufReader::new(f)
        .lines()
        .filter_map(Result::ok)
        // .take(10)
        .filter_map(|line| 
            line.split_ascii_whitespace().next().map(|v| v.to_string())
        )
        .collect::<Vec<_>>();

    let s: HashSet<_> = tokens.iter().collect::<HashSet<_>>();
    println!("abab => {}", s.contains(&"abab".to_string()));

    let filter: HP = HashProxy::from(&tokens);

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

    let encoded = bincode::serialize(&filter)?;
    std::fs::write("encoded.bin", encoded)?;

    // Test that it loads
    let decoded: HP = bincode::deserialize(&std::fs::read("encoded.bin")?)?;
    println!("{}", decoded.contains(&"bonjour".to_string()));

    Ok(())
}

fn word_check(word: &str, filter: &HP) {
    println!("Is \"{}\" found? {:?}", word, &filter.contains(&word.to_string()));

}
