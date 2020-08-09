extern crate base64;
extern crate hex;


fn main() {
    println!("{}", hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string()));
    println!("{:?}", assert_eq!(hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string()), "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string()))
}

fn hex_to_base64(hex_string: String) -> String {
    let decoded_string = hex::decode(hex_string).unwrap();
    let encoded_string = base64::encode(decoded_string);
    encoded_string
}