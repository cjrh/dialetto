pub mod errors;

use fnv::FnvHasher;
use xorf::{Filter, HashProxy, Xor8, Xor32, Xor16 as Filt};
use std::fs::File;
use std::io::{BufReader, BufRead};
use errors::DialettoError;
use bincode::deserialize;

pub type HP = HashProxy<String, FnvHasher, Filt>;

pub fn make_filter(filename: &str) -> Result<HP, errors::DialettoError> {
    // Reading words from language wordlist
    let f = File::open(filename)?;
    let tokens = BufReader::new(f)
        .lines()
        .filter_map(Result::ok)
        .filter_map(|line|
            line.split_ascii_whitespace().next().map(|v| v.to_string())
        )
        .collect::<Vec<_>>();

    let filter: HP = HashProxy::from(&tokens);
    Ok(filter)
}

pub fn encode(filter: &HP) -> Result<Vec<u8>, DialettoError> {
    Ok(bincode::serialize(filter)?)
}

pub fn decode(data: &[u8]) -> Result<HP, DialettoError> {
    Ok(bincode::deserialize(data)?)
}

pub fn encode_to_file(filter: &HP, filename: &str) -> Result<(), DialettoError> {
    let encoded = encode(filter)?;
    std::fs::write(filename, encoded)?;
    Ok(())
}

pub fn decode_from_file(filename: &str) -> Result<HP, DialettoError> {
    let data = std::fs::read(filename)?;
    let decoded = deserialize(&data)?;
    Ok(decoded)
}