use secp256k1::{Message, PublicKey, Secp256k1, Signature};
use sha2::{Digest, Sha256};
use std::convert::TryInto;

struct Transaction {
    version: u32,
    vin: Vec<Vin>,
    vout: Vec<u8>,
    locktime: u32,
}

struct Vin {
    txid: String,
    vout: u32,
    script_sig: String,
    sequence: u32,
    prevout: Vin,
}

fn prepare_sig_msg(tx: &Transaction, index: usize) -> Vec<u8> {
    let mut sig_msg = Vec::new();
    let mut hasher = Sha256::new();
    hasher.update(&(tx.version.to_le_bytes()));
    hasher.update(&(tx.locktime.to_le_bytes()));
    sig_msg.extend_from_slice(&hasher.finalize());
    sig_msg.extend_from_slice(&(index as u32).to_le_bytes());
    sig_msg
}

fn ripemd160_hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = ripemd160::Ripemd160::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

fn double_sha256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash = hasher.finalize();
    let mut hasher = Sha256::new();
    hasher.update(&hash);
    hasher.finalize().to_vec()
}

fn construct_p2wpkh_scriptcode(pubkey: &[u8]) -> Vec<u8> {
    let mut script_code = vec![0x00, 0x14];
    script_code.extend_from_slice(&pubkey);
    script_code
}

fn main() {
    let tx = Transaction {
        version: 1,
        vin: vec![Vin {
            txid: "0".to_string(),
            vout: 0xffffffff,
            script_sig: "3045022100ed77b44ed6b9e193f92ee563b7c3869a18bc6454edc822a65d3ac7e574bb97b30220678762fe995e3f1c1d2ba848806c1a6e4c993fe18a4e585271848ae7a8fd150101".to_string(),
            sequence: 0xffffffff,
            prevout: Vin {
                txid: "".to_string(),
                vout: 0,
                script_sig: "".to_string(),
                sequence: 0,
                prevout: Vin {
                    txid: "".to_string(),
                    vout: 0,
                    script_sig: "".to_string(),
                    sequence: 0,
                    prevout: Vin {
                        txid: "".to_string(),
                        vout: 0,
                        script_sig: "".to_string(),
                        sequence: 0,
                        prevout: Vin {
                            txid: "".to_string(),
                            vout: 0,
                            script_sig: "".to_string(),
                            sequence: 0,
                            prevout: Vin {
                                txid: "".to_string(),
                                vout: 0,
                                script_sig: "".to_string(),
                                sequence: 0,
                                prevout: Vin {
                                    txid: "".to_string(),
                                    vout: 0,
                                    script_sig: "".to_string(),
                                    sequence: 0,
                                    prevout: Vin {
                                        txid: "".to_string(),
                                        vout: 0,
                                        script_sig: "".to_string(),
                                        sequence: 0,
                                        prevout: Vin {
                                            txid: "".to_string(),
                                            vout: 0,
                                            script_sig: "".to_string(),
                                            sequence: 0,
                                            prevout: Vin {
                                                txid: "".to_string(),
                                                vout: 0,
                                                script_sig: "".to_string(),
                                                sequence: 0,
                                                prevout: Vin {
                                                    txid: "".to_string(),
                                                    vout: 0,
                                                    script_sig: "".to_string(),
                                                    sequence: 0,
                                                    prevout: Vin {
                                                        txid: "".to_string(),
                                                        vout: 0,
                                                        script_sig: "".to_string(),
                                                        sequence: 0,
                                                        prevout: Vin {
                                                            txid: "".to_string(),
                                                            vout: 0,
                                                            script_sig: "".to_string(),
                                                            sequence: 0,
                                                            prevout: Vin {
                                                                txid: "".to_string(),
                                                                vout: 0,
                                                                script_sig: "".to_string(),
                                                                sequence: 0,
                                                                prevout: Vin {
                                                                    txid: "".to_string(),
                                                                    vout: 0,
                                                                    script_sig: "".to_string(),
                                                                    sequence: 0,
                                                                    prevout: Vin {
                                                                        txid: "".to_string(),
                                                                        vout: 0,
                                                                        script_sig: "".to_string(),
                                                                        sequence: 0,
                                                                        prevout: Vin {
                                                                            txid: "".to_string(),
                                                                            vout: 0,
                                                                            script_sig: "".to_string(),
                                                                            sequence: 0,
                                                                            prevout: Vin {
                                                                                txid: "".to_string(),
                                                                                vout: 0,
                                                                                script_sig: "".to_string(),
                                                                                sequence: 0,
                                                                                prevout: Vin {
                                                                                    txid: "".to_string(),
                                                                                    vout: 0,
                                                                                    script_sig: "".to_string(),
                                                                                    sequence: 0,
                                                                                    prevout: Vin {
                                                                                        txid: "".to_string(),
                                                                                        vout: 0,
                                                                                        script_sig: "".to_string(),
                                                                                        sequence: 0,
                                                                                        prevout: Vin {
                                                                                            txid: "".to_string(),
                                                                                            vout: 0,
                                                                                            script_sig: "".to_string(),
                                                                                            sequence: 0,
                                                                                            prevout: Vin {
                                                                                                txid: "".to_string(),
                                                                                                vout: 0,
                                                                                                script_sig: "".to_string(),
                                                                                                sequence: 0,
                                                                                                prevout: Vin {
                                                                                                    txid: "".to_string(),
                                                                                                    vout: 0,
                                                                                                    script_sig: "".to_string(),
                                                                                                    sequence: 0,
                                                                                                    prevout: Vin {
                                                                                                        txid: "".to_string(),
                                                                                                        vout: 0,
                                                                                                        script_sig: "".to_string(),
                                                                                                        sequence: 0,
                                                                                                        prevout: Vin {
                                                                                                            txid: "".to_string(),
                                                                                                            vout: 0,
                                                                                                            script_sig: "".to_string(),
                                                                                                            sequence: 0,
                                                                                                            prevout: Vin {
                                                                                                                txid: "".to_string(),
                                                                                                                vout: 0,
                                                                                                                script_sig: "".to_string(),
                                                                                                                sequence: 0,
                                                                                                            },
                                                                                                        },
                                                                                                    },
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                    },
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                        },
                                                    },
                                                },
                                            },
                                        },
                                    },
                                },
                            },
                        },
                    },
                },
            },
        }],
        vout: Vec::new(),
        locktime: 0,
    };

    let secp = Secp256k1::new();
    let mut hash_vec = Vec::new();
    let mut i = 0;
    while i < tx.vin.len() {
        let mut sig_hash = prepare_sig_msg(&tx, i);
        hash_vec.append(&mut sig_hash);
        i += 1;
    }

    let privkey_hex = "1122334455667788112233445566778811223344556677881122334455667788";
    let privkey_bytes = hex::decode(privkey_hex).unwrap();
    let privkey = secp256k1::SecretKey::from_slice(&privkey_bytes).unwrap();

    let pubkey = PublicKey::from_secret_key(&secp, &privkey).serialize_uncompressed();

    let mut sig_vec = Vec::new();
    let mut i = 0;
    while i < tx.vin.len() {
        let sig_hash = prepare_sig_msg(&tx, i);
        let sig = secp.sign_recoverable(&Message::from_slice(&sig_hash).unwrap(), &privkey);
        let rec_id = sig.unwrap().recid.serialize();
        let rec_id_with_v = if rec_id < 4 { rec_id + 27 } else { rec_id + 35 };
        let rec_sig = sig.unwrap().signature.serialize_compact();
        let mut rec_sig_vec = Vec::new();
        rec_sig_vec.extend_from_slice(&rec_sig);
        rec_sig_vec.push(rec_id_with_v);
        sig_vec.append(&mut rec_sig_vec);
        i += 1;
    }

    println!("ScriptSig: {}", hex::encode(sig_vec));

    let pubkey_hash = ripemd160_hash(&double_sha256(&pubkey));
    let script_code = construct_p2wpkh_scriptcode(&pubkey);

    let mut sig_hasher = Sha256::new();
    sig_hasher.update(&[0x00, 0x01, 0x00, 0x00]);
    sig_hasher.update(&double_sha256(&[0x19, 0x76, 0xa9, 0x14]));
    sig_hasher.update(&pubkey_hash);
    sig_hasher.update(&[0x88, 0xac]);

    let sig_hash = sig_hasher.finalize();
    let mut message = Message::from_slice(&sig_hash[..]).unwrap();

    let pubkey_bytes = PublicKey::from_slice(&pubkey).unwrap();
    let mut recovered_pubkey = secp.recover(&message, &Signature::from_compact(&sig_vec[0..64]).unwrap()).unwrap();
    recovered_pubkey.normalize();
    assert_eq!(recovered_pubkey.serialize_uncompressed().to_vec(), pubkey_bytes.serialize_uncompressed().to_vec());

    let txid = tx.get_id();
    let sig_hash = prepare_sig_msg(&tx, 0);
    let mut signature = sig_vec.clone();
    signature.pop();
    let signature = secp256k1::Signature::from_compact(&signature).unwrap();
    let pubkey = secp256k1::PublicKey::from_slice(&pubkey).unwrap();
    let verified = secp256k1::verify(&secp256k1::Message::parse_slice(&sig_hash).unwrap(), &signature, &pubkey);

    println!("Signature verification: {}", verified);

    let serialized_tx = tx.serialize();

    println!("Transaction ID: {}", txid);
    println!("Serialized Transaction: {}", serialized_tx);

    let tx_hash = double_sha256(&serialized_tx);

    println!("Transaction Hash: {}", hex::encode(&tx_hash));
}
