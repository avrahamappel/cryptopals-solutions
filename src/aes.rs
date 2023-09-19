fn round_keys(initial_key: &[u8]) -> impl Iterator<Item = Vec<u8>> {
    todo!()
}

fn add_round_key(mut state: Vec<u8>, round_key: &[u8]) {
    if state.len() != round_key.len() {
        panic!("State and round key must be the same size");
    }

    todo!()
}

fn sub_bytes(mut state: Vec<u8>) {
    todo!()
}

fn shift_rows(mut state: Vec<u8>) {
    todo!()
}

fn mix_columns(mut state: Vec<u8>) {
    todo!()
}

// Steps:
// KeyExpansion
// 1st round
// AddRoundKey
// 2nd to 9th round (for 128)
// 1. SubBytes
// 2. ShiftRows
// 3. MixColumns
// 4. AddRoundKey
// 10th round
// 1. SubBytes
// 2. ShiftRows
// 3. AddRoundKey

/// Encode a cipher using AES
pub fn encode(cipher: &[u8], key: &[u8]) -> Option<Vec<u8>> {
    let mut state = cipher.to_vec();

    for (i, rk) in round_keys(key).take(11).enumerate() {
        if i != 0 {
            sub_bytes(state);
            shift_rows(state);

            if i != 10 {
                mix_columns(state);
            }
        }

        add_round_key(state, &rk);
    }

    Some(state)
}

// To decode, just follow the steps in the opposite order
