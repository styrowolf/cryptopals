use std::collections::BTreeMap;
use std::collections::HashMap;
extern crate hex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::iter::Zip;

fn main() {
    const FILE_DIR: &str = "4.txt";
    let lines = read_lines(FILE_DIR);
    let mut texts: Vec<String> = Vec::new();
    let mut ciphertexts: Vec<String> = Vec::new();
    for i in lines {
        let a = i.clone();
        let hex_bytes = hex::decode(i).unwrap();
        let length = hex_bytes.len();
        for j in 0..=255 {
            ciphertexts.push(a.to_string());
            let xor_bytes: Vec<u8> = vec![j; length];
            texts.push(String::from_utf8_lossy(&xor(hex_bytes.to_vec(), xor_bytes)).into_owned());
        }
    }
    let mut iterator = ciphertexts.iter().zip(texts.iter());

    let scored_plaintexts = score(iterator);
    for (i, j) in scored_plaintexts {
        println!("score: {}, ciphertext: {}, plaintext: {}", i, j.0, j.1);
    }
    
}

fn xor(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();
    for i in 0..a.len() {
        let c = a.get(i).unwrap() ^ b.get(i).unwrap();
        output.push(c);
    }
    output
}

fn score(s: Zip<std::slice::Iter<std::string::String>, std::slice::Iter<std::string::String>>) -> BTreeMap<u128, (String, String)> {
    
    let letters: Vec<&str> = vec!["a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z"];
    let scores: Vec<u128> = vec![8497,1492,2202,4253,11162,2228,2015,6094,7546,0153,1292,4025,2406,6749,7507,1929,0095,7587,6327,9356,2758,0978,2560,0150,1994,0077];
    let mut result: BTreeMap<u128, (String, String)> = BTreeMap::new();
    let mut letter_score: HashMap<&str, u128> = HashMap::new();
    for i in 0..letters.len() {
        letter_score.insert(letters.get(i).unwrap(), *scores.get(i).unwrap());
    }
    for i in s {
        let mut score: u128 = 0;
        for j in i.1.chars() {
            let mut buf: [u8; 4] = [0; 4];
            let char_as_str = j.encode_utf8(&mut buf);
            score += letter_score.get(char_as_str).unwrap_or(&0u128);
        }
        result.insert(score, (i.0.to_string(),i.1.to_string()));
    }
    
    result
}

fn read_lines(filename: &str) -> Vec<String> {
    let file = File::open(&filename).unwrap();

    let reader = io::BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        lines.push(line);
    }
    lines
}