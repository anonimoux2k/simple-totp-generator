use base32::Alphabet::*;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use time::OffsetDateTime;

fn main() {
    let secret = "THISISASECRETKEY";
    let totp = generate_totp(secret, 60);
    println!("Your TOTP is: {}", totp);
}

fn generate_totp(secret: &str, time_step: i64) -> String {
    let secret_bytes = base32::decode(Rfc4648 { padding: false }, secret).expect("Invalid base32 string");
    
    let time = OffsetDateTime::now_utc().unix_timestamp() / time_step;
    let mut time_bytes = [0u8; 8];
    for (i, byte) in time.to_be_bytes().iter().enumerate() {
        time_bytes[i] = *byte;
    }

    let mut mac = Hmac::<Sha256>::new_from_slice(&secret_bytes).expect("Invalid key length");
    mac.update(&time_bytes);
    let result = mac.finalize().into_bytes();
    let offset = (result[result.len() - 1] & 0xf) as usize;
    
    // Compliant with RFC6238
    let code = (
        ((result[offset] & 0x7f) as u32) << 24
            | ((result[offset + 1] & 0xff) as u32) << 16
            | ((result[offset + 2] & 0xff) as u32) << 8
            | (result[offset + 3] & 0xff) as u32
    ) % 1_000_000;

    format!("{:06}", code)
}