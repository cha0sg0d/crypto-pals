use base64;
use hex;

// From prompt: Always operate on raw bytes, never on encoded strings

fn main() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    // checks for errors on converting hex to raw bytes. If none, proceeds.
    if let Ok(raw) = hex::decode(input) {
        // grabs raw bytes from Result() object
        println!("raw {:?}", raw);
        // passes raw bytes to base64 encoding
        let b64 = base64::encode(raw); // converts bytes to b64 string
        println!("b64 {}", b64);
    } else {
        println!("err");
    }
}
