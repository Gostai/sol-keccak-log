extern crate keccak_rust;

use keccak_rust::*;

//const YOUR_INPUT_BYTES: [Byte; 12] = [72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 33];

fn main() {
    let input = b"Hello world";
    let mut keccak = Keccak::new(SecurityLevel::SHA256, StateBitsWidth::F1600);
    keccak.append(& input[..]);
    println!("{:?}", keccak.hash());
    let b64 = base64::encode(keccak.hash());
    println!("{}", b64);
    
    
}
