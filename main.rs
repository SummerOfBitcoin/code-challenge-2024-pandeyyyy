use miner::structs::{GasedTransaction, Transaction};
use miner::utils::print_hex_string;
use miner::{
    calculate_merkle_root, calculate_wtxid, create_block_header, create_coinbase_trx, get_compact_size, mine_block, populate, print_soln
};


use std::error::Error;
use std::fs::read_dir;

fn main() -> Result<(), Box<dyn Error>> {
    let dir_path = "../mempool";
    let mut txns: Vec<GasedTransaction> = Vec::new();

    for entry in read_dir(dir_path)? {
            let entry = entry?;
    let path = entry.path();

        if path.is_file() && path.extension().unwrap() == "json" {
            let tx = Transaction::parse_from_file(path.to_str().unwrap())?;
        let segwit_valid = tx.valid_trans();
                if segwit_valid.0 {
            let data = tx.get_data(segwit_valid.1);
            txns.push(GasedTransaction{gas:tx.calculate_gas(), weight:tx.calculate_tx_weight(), data, txid: tx.get_txid(), is_segwit: segwit_valid.1 });
                }
        }
    }

    txns.sort_by(|a, b| b.cmp(a));
        let (trans, mut txids) = populate(&txns);
    let wtxid_merkle = calculate_wtxid(trans);
    let (trx, crx) = create_coinbase_trx(wtxid_merkle);
    txids.push(crx);
    txids.reverse();
    let merkle_root = calculate_merkle_root(&txids);
    let block_header = create_block_header(merkle_root);
    let (block, _) = mine_block(&block_header);
    print_soln(&block, &trx, &txids);
    Ok(())
}
