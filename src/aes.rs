// fn add_round_key<S>(state: &mut [u8; S], round_key: &[u8; S]) {
fn add_round_key(mut state: Vec<u8>, round_key: &[u8]) {
    // TODO this can probably be replaced with a const param
    assert!(
        state.len() == round_key.len(),
        "State and round key must be the same size"
    );

    for (i, byte) in state.iter_mut().enumerate() {
        *byte ^= round_key[i];
    }
}

fn sub_bytes(mut state: Vec<u8>) {
    todo!()
}

const STATE_LEN: usize = 16;

/// Shift the rows of the input to the left in increasing order
///
/// 1 5 9  13     1  5  9 13
/// 2 6 10 14 --> 6 10 14  2
/// 3 7 11 15    11 15  3  7
/// 4 8 12 16    16  4  8 12
///
/// So each time we subtract 3 from the index, wrapping to 16
fn shift_rows(state: &mut [u8; STATE_LEN]) {
    let mut shifted = vec![0; STATE_LEN];
    let mut index = 0;

    for byte in &mut *state {
        shifted[index] = *byte;
        if index <= 3 {
            index += 16;
        }
        index -= 3;
    }

    state.swap_with_slice(shifted.as_mut_slice());
}

#[cfg(test)]
#[test]
fn test_shift_rows() {
    let mut state = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

    shift_rows(&mut state);

    assert_eq!(
        [1, 6, 11, 16, 5, 10, 15, 4, 9, 14, 3, 8, 13, 2, 7, 12],
        state
    );
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

// Encode a cipher using AES
pub fn encode(cipher: &[u8], key: &[u8]) -> Option<Vec<u8>> {
    let mut state = cipher.clone();

    for (i, rk) in round_keys(key).take(11).enumerate() {
        if i != 0 {
            // sub_bytes(state);
            shift_rows(state);

            if i != 10 {
                // mix_columns(state);
            }
        }

        // add_round_key(state, &rk);
    }

    Some(state)
}

// To decode, just follow the steps in the opposite order
pub fn decode(cipher: &[u8], key: &str, arg: i32) -> () {
    todo!()
}
