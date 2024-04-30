use serde::Deserialize;
use serde_json::from_reader;
use std::cmp::Ordering;
use std::fs::File;
use crate::{calculate_txid, get_compact_size, serialize_transation, validate_transaction};
use crate::utils::get_current_unix_timestamp_u32;

#[derive(Debug, Deserialize)]
pub struct Prevout {
    pub scriptpubkey: String,
    pub scriptpubkey_asm: String,
    pub scriptpubkey_type: String,
    pub scriptpubkey_address: String,
    pub value: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Witness(pub Vec<String>);

#[derive(Debug, Deserialize)]
pub struct Vin {
    pub txid: String,
    pub vout: u32,
    pub prevout: Prevout,
    pub scriptsig: String,
    pub scriptsig_asm: String,
    pub witness: Option<Witness>,
    pub is_coinbase: bool,
    pub sequence: u32,
}

#[derive(Debug, Deserialize)]
pub struct Vout {
    pub scriptpubkey: String,
    pub scriptpubkey_asm: String,
    pub scriptpubkey_type: String,
    pub scriptpubkey_address: Option<String>,
    pub value: u64,
}

#[derive(Debug, Deserialize)]
pub struct Transaction {
    pub version: u32,
    pub locktime: u32,
    pub vin: Vec<Vin>,
    pub vout: Vec<Vout>,
}

#[derive(Debug, Deserialize, Eq)]
pub struct GasedTransaction {
    pub gas: u64,
    pub weight: u32,
    pub data: Vec<u8>,
    pub txid: Vec<u8>,
    pub is_segwit: bool,
}

impl Transaction {
    pub fn parse_from_file(file_path: &str) -> Result<Transaction, std::io::Error> {
        let file = File::open(file_path)?;
        let transaction: Transaction = from_reader(file)?;
        Ok(transaction)
    }

    pub fn valid_trans(&self) -> (bool, bool) {
        let gas = self.calculate_gas();
        if gas <= 0 {
            return (false, false);
        }
        if self.locktime != 0 {
            if self.locktime < 499999999 {
                return (false, false);
            } else {
                let unixtime = get_current_unix_timestamp_u32();
                if self.locktime > unixtime {
                    return (false, false);
                }
            }
        }
        match validate_transaction(self) {
            Ok(flag) => return (true, flag),
            Err(_) => return (false, false),
        }
    }

    pub fn get_data(&self, issegwit: bool) -> Vec<u8> {
        return serialize_transation(self, issegwit);
    }

    pub fn get_txid(&self) -> Vec<u8> {
        return calculate_txid(self);
    }

    pub fn calculate_gas(&self) -> u64 {
        let mut input = 0;
        let mut output = 0;
        for vin in self.vin.iter() {
            input += vin.prevout.value;
        }
        for vout in self.vout.iter() {
            output += vout.value;
        }
        input - output
    }

    pub fn calculate_tx_weight(&self) -> u32 {
        let mut base_size = 4;
        base_size += get_compact_size(self.vin.len()).len();
        base_size += get_compact_size(self.vout.len()).len();
        base_size += 4;

        let mut witness_size = 0;

        for vin in &self.vin {
            base_size += 32;
            base_size += 4;
            base_size += get_compact_size(vin.scriptsig.len() / 2).len();
            base_size += vin.scriptsig.len() / 2;
            base_size += 4;

            if let Some(witness) = &vin.witness {
                witness_size += 2;
                witness_size += get_compact_size(witness.0.len()).len();
                for element in &witness.0 {
                    witness_size += get_compact_size(element.len() / 2).len();
                    witness_size += element.len() / 2
                }
            }
        }

        for vout in &self.vout {
            base_size += 8;
            base_size += get_compact_size(vout.scriptpubkey.len() / 2).len();
            base_size += vout.scriptpubkey.len() / 2;
        }

        let weight = 4 * base_size + witness_size;
        weight as u32
    }
}

impl PartialEq for GasedTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.gas == other.gas && self.weight == other.weight
    }
}

impl Ord for GasedTransaction {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.gas > other.gas {
            return Ordering::Greater;
        } else if self.gas < other.gas {
            return Ordering::Less;
        }
        if self.weight < other.weight {
            return Ordering::Less;
        } else if self.weight > other.weight {
            return Ordering::Greater;
        }
        Ordering::Equal
    }
}

impl PartialOrd for GasedTransaction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
