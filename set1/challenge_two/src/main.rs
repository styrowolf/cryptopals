extern crate hex;

fn main() {
    const x: &str = "1c0111001f010100061a024b53535009181c";
    const y: &str = "686974207468652062756c6c277320657965";
    let x_as_bytes = hex::decode(x).unwrap();
    let y_as_bytes = hex::decode(y).unwrap();
    let xored_bytes = xor(x_as_bytes, y_as_bytes);
    let xored_as_hex = hex::encode(xored_bytes);
    println!("{}", xored_as_hex);
    println!("{:?}", xored_as_hex == "746865206b696420646f6e277420706c6179");
}

fn xor(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();
    for i in 0..a.len() {
        let c = a.get(i).unwrap() ^ b.get(i).unwrap();
        output.push(c);
    }
    output
}