extern crate hex;
fn main() {
    const KEY: &str = "ICE";
    const TEXT: &str = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let stream = expand_key(KEY.as_bytes().to_vec(), TEXT.as_bytes().len());
    let xored = xor(stream, TEXT.as_bytes().to_vec());

    let hex = hex::encode(xored);
    println!("{}", hex);
    println!("{:?}", assert_eq!(hex, "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"))
}

fn xor(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();
    for i in 0..a.len() {
        let c = a.get(i).unwrap() ^ b.get(i).unwrap();
        output.push(c);
    }
    output
}

fn expand_key(k: Vec<u8>, length: usize) -> Vec<u8> {
    let mut expanded: Vec<u8> = k.clone();
    loop {
        for i in &k {
            if expanded.len() < length {
                expanded.push(*i);
            } else {
                return expanded
            }
        }
    }
}