use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::iter::Zip;

extern crate base64;
extern crate ordered_float;

fn main() {
    let text = read("6.txt");
    let bytes = base64::decode(text).unwrap();
    let keysize_hamming: BTreeMap<ordered_float::OrderedFloat<f64>, u32> = BTreeMap::new();
    for keysize in 2..=40usize {
        let block_one = &bytes[0..keysize];
        let block_two = &bytes[keysize..keysize*2];
    }
}

/*
For binary strings a and b the Hamming distance is equal to the number of ones (population count) in a XOR b.
*/

fn calculate_hamming_distance(a: Vec<u8>, b: Vec<u8>) -> u32 {
    let mut distance: u32 = 0;
    let xored = xor(a, b);
    for i in xored {
        distance += i.count_ones();
    }
    distance
}

fn xor(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();
    for i in 0..a.len() {
        let c = a.get(i).unwrap() ^ b.get(i).unwrap();
        output.push(c);
    }
    output
}

fn read(filename: &str) -> String {
    let mut file = File::open(&filename).unwrap();
    let mut text = String::new();
    let _ = file.read_to_string(&mut text);
    text
}

fn divide_into_blocks(stream: Vec<u8>, length: u32) -> Vec<Vec<u8>> {
    let mut stream = VecDeque::from(stream);
    
    let mut blocks: Vec<Vec<u8>> = Vec::new();
    loop {
        if !stream.is_empty() {
            let mut block: Vec<u8> = Vec::new();
            let mut len = block.len();
            while len < length as usize {
                let i = match stream.pop_front() {
                    Some(i) => i,
                    None => break
                };
                block.push(i);
                len += 1;
            }
            blocks.push(block);
        } else {
            break
        }
    }
    blocks

}

fn blocks_of_bytes(blocks: Vec<Vec<u8>>, block_length: u32) -> Vec<Vec<u8>> {
    let mut blocks_result: Vec<Vec<u8>> = Vec::new();
    let mut blocks_vecdeque: Vec<VecDeque<u8>> = Vec::new();
    for i in blocks {
        let block: VecDeque<u8> = VecDeque::from_iter()
    }
    for i in 0..block_length {
        let mut block: Vec<u8> = Vec::new();
        for j in
    }
    blocks
}
