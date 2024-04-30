use sha2::{Digest, Sha256};
use ripemd160::Ripemd160;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn c_hash160(data: &[u8]) -> [u8; 20] {
    let mut s = Sha256::new();
    s.update(data);
    let r = s.finalize();

    let mut r160 = Ripemd160::new();
    r160.update(&r);
    let r160_result = r160.finalize();

    let mut h160 = [0; 20];
    h160.copy_from_slice(&r160_result[..]);
    h160
}

pub fn d_sha256(data: &[u8]) -> Vec<u8> {
    let mut h1 = Sha256::new();
    h1.update(data);
    let f_hash = h1.finalize();

    let mut h2 = Sha256::new();
    h2.update(&f_hash);
    h2.finalize().to_vec()
}

pub fn s_d_hash(data: &[u8]) -> Vec<u8> {
    let mut h1 = Sha256::new();
    h1.update(data);
    let f_hash = h1.finalize().to_vec();

    let mut h2 = Sha256::new();
    h2.update(&f_hash);
    h2.finalize().to_vec()
}

pub fn r160_hash(data: &[u8]) -> Vec<u8> {
    let mut h = Ripemd160::new();
    h.update(data);
    h.finalize().to_vec()
}

pub fn get_unix_ts_u32() -> u32 {
    let n = SystemTime::now();
    let t: u32 = n.duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs() as u32;
    t
}

pub fn print_hex_string(bytes: &[u8]) {
    let h = bytes.iter().map(|&b| format!("{:02x}", b)).collect::<String>();
    println!("{}", h);
}
