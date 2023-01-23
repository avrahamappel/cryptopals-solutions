use cryptopals::base64;
use cryptopals::hamming;
use cryptopals::xor;

fn main() {
    // There's a file here. It's been base64'd after being encrypted with repeating-key XOR.
    let input = base64::decode(include_str!("../data/6.txt"));

    // Decrypt it.

    // Here's how:

    // Let KEYSIZE be the guessed length of the key; try values from 2 to (say) 40.
    let mut key_sizes: Vec<_> = (2..=40)
        .map(|keysize| {
            // Write a function to compute the edit distance/Hamming distance between two strings. The Hamming distance is just the number of differing bits. The distance between:
            // this is a test
            // and
            // wokka wokka!!!
            // is 37. Make sure your code agrees before you proceed.
            // For each KEYSIZE, take the first KEYSIZE worth of bytes, and the second KEYSIZE worth of bytes, and find the edit distance between them. Normalize this result by dividing by KEYSIZE.

            let chunk1 = &input[..keysize];
            let chunk2 = &input[keysize..(keysize + 1)];

            hamming::distance(chunk1, chunk2) / keysize
        })
        .collect();

    key_sizes.sort();

    dbg!(&key_sizes);

    // The KEYSIZE with the smallest normalized edit distance is probably the key. You could proceed perhaps with the smallest 2-3 KEYSIZE values. Or take 4 KEYSIZE blocks instead of 2 and average the distances.
    for keysize in key_sizes.into_iter().take(3) {
        // Now that you probably know the KEYSIZE: break the ciphertext into blocks of KEYSIZE length.
        let key: Vec<_> = input
            .chunks_exact(keysize)
            // Now transpose the blocks: make a block that is the first byte of every block, and a block that is the second byte of every block, and so on.
            .fold(
                (0..keysize)
                    .map(|_| Vec::with_capacity(input.len() / keysize))
                    .collect(),
                |mut blocks: Vec<_>, chunk| {
                    for i in 0..keysize {
                        blocks[i].push(chunk[i]);
                    }

                    blocks
                },
            )
            // Solve each block as if it was single-character XOR. You already have code to do this.
            .into_iter()
            .map(|block| xor::single(&block)[0].byte)
            // For each block, the single-byte XOR key that produces the best looking histogram is the repeating-key XOR key byte for that block. Put them together and you have the key.
            .collect();

        println!("Key: {}", String::from_utf8_lossy(&key));

        let decoded = xor::repeating(&input, &key);

        println!("Decoded: {}", String::from_utf8_lossy(&decoded));
    }
    // This code is going to turn out to be surprisingly useful later on. Breaking repeating-key XOR ("Vigenere") statistically is obviously an academic exercise, a "Crypto 101" thing. But more people "know how" to break it than can actually break it, and a similar technique breaks something much more important.
}
