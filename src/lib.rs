use std::collections::HashMap;

// Name Assignment (variables and constants)
// TODO: Assign the current bitcoin mining reward
pub const MINING_REWARD: f64 = 12.5;
// TODO: Assign the current block height
pub const CURRENT_BLOCK_HEIGHT: u64 = 700_000;
// TODO: Assign the number of satoshis in one Bitcoin
pub const BTC_TO_SATS: u64 = 100_000_000;

#[derive(Debug, Clone, PartialEq)]
pub struct Utxo {
    pub txid: String,
    pub vout: u32,
    pub value: u64,
}

/// Calculate the total Bitcoin reward for a given number of mined blocks.
pub fn calculate_total_reward(_blocks_mined: u64) -> f64 {
    // TODO: Multiply blocks_mined by MINING_REWARD and return result
    _blocks_mined as f64 * MINING_REWARD
}

/// Return true if the transaction fee is between 0.00001 and 0.01 BTC.
pub fn is_valid_tx_fee(_fee: f64) -> bool {
    // TODO: Check if fee is between 0.00001 and 0.01 BTC (inclusive)
    _fee >= 0.00001 && _fee <= 0.01
}

/// Return true if the wallet balance is greater than 50.0 BTC.
pub fn is_large_balance(_balance: f64) -> bool {
    // TODO: Compare balance to 50.0 and return result
    _balance > 50.0
}

/// Return the priority of a transaction ("high", "medium", "low") based on fee rate.
pub fn tx_priority(_size_bytes: u64, _fee_btc: f64) -> &'static str {
    // TODO: Calculate fee rate (fee_btc / size_bytes) and use if/else if/else
    // High: > 0.00005, Medium: > 0.00001, otherwise Low
    let _fee_rate = _fee_btc / _size_bytes as f64;
    if _fee_rate > 0.00005 {
        "high"
    } else if _fee_rate > 0.00001 {
        "medium"
    } else {
        "low"
    }
}


/// Return true if the network string equals "mainnet" (case-insensitive).
pub fn is_mainnet(_network: &str) -> bool {
    // TODO: Convert network to lowercase and compare with "mainnet"
    _network.to_lowercase() == "mainnet"
}

/// Return true if value is in the inclusive range 100..=200.
pub fn is_in_range(_value: i64) -> bool {
    // TODO: Check if 100 <= value <= 200
    _value >= 100 && _value <= 200
}

/// Return true if both references point to the exact same object in memory.
pub fn is_same_wallet<T>(_wallet1: &T, _wallet2: &T) -> bool {
    // TODO: Use std::ptr::eq to compare reference identity
    std::ptr::eq(_wallet1, _wallet2)
}

/// Normalize a Bitcoin address by trimming whitespace and lowercasing.
pub fn normalize_address(_address: &str) -> String {
    // TODO: Trim leading/trailing whitespace and convert to lowercase
    _address.trim().to_lowercase()
}

/// Append a new UTXO to the list and return the updated list.
pub fn add_utxo(_utxos: Vec<Utxo>, _new_utxo: Utxo) -> Vec<Utxo> {
    // TODO: Push new_utxo into utxos and return it
    let mut _updated_utxos = _utxos;
    _updated_utxos.push(_new_utxo);
    _updated_utxos
}

/// Find the first transaction with a fee greater than 0.005 BTC.
pub fn find_high_fee(_fee_list: &[f64]) -> Option<(usize, f64)> {
    // TODO: Iterate with enumerate and return the first (index, fee) where fee > 0.005
    for (i, &_fee) in _fee_list.iter().enumerate() {
        if _fee > 0.005 {
            return Some((i, _fee));
        }
    }
    None
}

/// Return basic wallet details as a tuple of (name, balance).
pub fn get_wallet_details() -> (String, f64) {
    // TODO: Return a tuple with wallet name and balance
    ("satoshi_wallet".to_string(), 50.0)
}

/// Get the status of a transaction from the mempool or "not found".
pub fn get_tx_status(_tx_pool: &HashMap<String, String>, _txid: &str) -> String {
    // TODO: Look up txid in tx_pool, returning the status or "not found"
    match _tx_pool.get(_txid) {
        Some(status) => status.clone(),
        None => "not found".to_string(),
    }
}

/// Destructure wallet_info and format a status string.
pub fn unpack_wallet_info(_wallet_info: (String, f64)) -> String {
    // TODO: Destructure the tuple into (name, balance) and format the result
    // Expected format: "Wallet <name> has balance: <balance> BTC"
    let (_name, _balance) = _wallet_info;
    format!("Wallet {} has balance: {} BTC", _name, _balance)
}

/// Convert BTC to satoshis (1 BTC = 100,000,000 sats).
pub fn calculate_sats(btc: f64) -> u64 {
    // TODO: Multiply btc by BTC_TO_SATS and return as u64
    (btc * BTC_TO_SATS as f64) as u64
}

/// Generate a mock Bitcoin address of length 32 with the given prefix.
pub fn generate_address(_prefix: &str) -> String {
    // TODO: Build a random suffix of (32 - prefix.len()) chars from [a-z0-9]
    // TODO: Concatenate prefix + suffix and return
    let _suffix_length = 32 - _prefix.len();
    let _suffix: String = (0.._suffix_length)
        .map(|_| {
            let _chars = b"abcdefghijklmnopqrstuvwxyz0123456789";
            let _idx = rand::random::<usize>() % _chars.len();
            _chars[_idx] as char
        })
        .collect();
    format!("{}{}", _prefix, _suffix)
}

/// Validate a Bitcoin block height. Returns (is_valid, message).
pub fn validate_block_height(_height: i64) -> (bool, String) {
    // TODO: Check that height is not negative
    // TODO: Check that height is within a realistic range (<= 1_000_000)
    // TODO: Return (true, "Valid block height") otherwise
    if _height < 0 {
        (false, "Block height cannot be negative".to_string())
    } else if _height > 1_000_000 {
        (false, "Block height is unrealistic".to_string())
    } else {
        (true, "Valid block height".to_string())
    }
}

/// Compute the block reward (in sats) for each block height based on the halving schedule.
pub fn halving_schedule(_blocks: &[u64]) -> HashMap<u64, u64> {
    // TODO: Base reward is 50 * 100_000_000 sats; halving interval is 210_000 blocks
    // TODO: For each block: halvings = block / 210_000; reward = base >> halvings
    // TODO: Insert (block, reward) into the result HashMap

    let mut _rewards = HashMap::new();
    let _base_reward = 50 * BTC_TO_SATS; // 50 BTC in sats
    let _halving_interval = 210_000;

    for &_block in _blocks {
        let _halvings = _block / _halving_interval;
        let _reward = _base_reward >> _halvings;
        _rewards.insert(_block, _reward);
    }

    _rewards
}

/// Find the UTXO with the smallest value that meets or exceeds target.
pub fn find_utxo_with_min_value(_utxos: &[Utxo], _target: u64) -> Option<Utxo> {
    // TODO: Filter UTXOs to those with value >= target
    // TODO: Return the one with the smallest value, or None if none qualify
    _utxos
        .iter()
        .filter(|_utxo| _utxo.value >= _target)
        .min_by_key(|_utxo| _utxo.value)
        .cloned()
}

/// Create a UTXO map from txid, vout, and arbitrary extra string fields.
pub fn create_utxo(
    _txid: &str,
    _vout: u32,
    _extra: HashMap<String, String>,
) -> HashMap<String, String> {
    // TODO: Build a base map with "txid" and "vout" (as string)
    // TODO: Merge extra into the base map and return
    let mut _utxo_map = HashMap::new();
    _utxo_map.insert("txid".to_string(), _txid.to_string());
    _utxo_map.insert("vout".to_string(), _vout.to_string());
    _utxo_map.extend(_extra);
    _utxo_map
}

// Implement extract_tx_version function below
pub fn extract_tx_version(_raw_tx_hex: &str) -> Result<u32, String> {
    if _raw_tx_hex.len() < 8 {
        return Err("Transaction data too short".to_string());
    }

    let _version_hex = &_raw_tx_hex[0..8];
    let _bytes = hex::decode(_version_hex).map_err(|e| format!("Hex decode error: {}", e))?;
    let _version = u32::from_le_bytes([_bytes[0], _bytes[1], _bytes[2], _bytes[3]]);
    Ok(_version)
}
