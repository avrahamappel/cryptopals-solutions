use aes::{
    cipher::{generic_array::GenericArray, BlockDecrypt, KeyInit},
    Aes128,
};
use itertools::Itertools;

use crate::{
    crack_utils::{guess_keysizes, transpose, CrackedMessage},
    sorted::Sorted,
};

#[must_use]
pub fn aes_128_ecb(key: &[u8], mut data: Vec<u8>) -> Vec<u8> {
    let key = GenericArray::from_slice(key);
    let cipher = Aes128::new(key);
    for chunk in data.chunks_mut(16) {
        cipher.decrypt_block(GenericArray::from_mut_slice(chunk));
    }
    data
}

fn single(bytes: &[u8]) -> Vec<CrackedMessage<u8>> {
    (0x00..=0xFF)
        .filter_map(|b| {
            let cipher = Aes128::new(GenericArray::from_slice(&[b; 16]));
            let mut data = bytes.to_owned();
            cipher.decrypt_block(GenericArray::from_mut_slice(&mut data));

            CrackedMessage::try_from((b, data)).ok()
        })
        .collect_vec()
        .sorted()
}

#[must_use]
pub fn aes_crack(input: &[u8], min: usize, max: usize) -> Vec<CrackedMessage<Vec<u8>>> {
    guess_keysizes(input, min, max)
        .into_iter()
        .filter_map(|keysize| {
            let key: Vec<_> = transpose(input, keysize)
                .into_iter()
                .filter_map(|block| single(&block).first().map(|res| res.key))
                .collect();

            if key.len() != keysize {
                return None;
            }

            let data = input.to_vec();
            let decoded = aes_128_ecb(&key, data);

            CrackedMessage::try_from((key, decoded)).ok()
        })
        .collect_vec()
        .sorted()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes_128_ecb() {
        let key = "YELLOW SUBMARINE";
        let string = include_str!("../data/7.txt");

        let decrypted = aes_128_ecb(key.as_bytes(), crate::base64::decode(string));
        let decrypted_str = std::str::from_utf8(&decrypted).unwrap();

        assert_eq!("I'm back and I'm ringin' the bell \nA rockin' on the mike while the fly girls yell \nIn ecstasy in the back of me \nWell that's my DJ Deshay cuttin' all them Z's \nHittin' hard and the girlies goin' crazy \nVanilla's on the mike, man I'm not lazy. \n\nI'm lettin' my drug kick in \nIt controls my mouth and I begin \nTo just let it flow, let my concepts go \nMy posse's to the side yellin', Go Vanilla Go! \n\nSmooth 'cause that's the way I will be \nAnd if you don't give a damn, then \nWhy you starin' at me \nSo get off 'cause I control the stage \nThere's no dissin' allowed \nI'm in my own phase \nThe girlies sa y they love me and that is ok \nAnd I can dance better than any kid n' play \n\nStage 2 -- Yea the one ya' wanna listen to \nIt's off my head so let the beat play through \nSo I can funk it up and make it sound good \n1-2-3 Yo -- Knock on some wood \nFor good luck, I like my rhymes atrocious \nSupercalafragilisticexpialidocious \nI'm an effect and that you can bet \nI can take a fly girl and make her wet. \n\nI'm like Samson -- Samson to Delilah \nThere's no denyin', You can try to hang \nBut you'll keep tryin' to get my style \nOver and over, practice makes perfect \nBut not if you're a loafer. \n\nYou'll get nowhere, no place, no time, no girls \nSoon -- Oh my God, homebody, you probably eat \nSpaghetti with a spoon! Come on and say it! \n\nVIP. Vanilla Ice yep, yep, I'm comin' hard like a rhino \nIntoxicating so you stagger like a wino \nSo punks stop trying and girl stop cryin' \nVanilla Ice is sellin' and you people are buyin' \n'Cause why the freaks are jockin' like Crazy Glue \nMovin' and groovin' trying to sing along \nAll through the ghetto groovin' this here song \nNow you're amazed by the VIP posse. \n\nSteppin' so hard like a German Nazi \nStartled by the bases hittin' ground \nThere's no trippin' on mine, I'm just gettin' down \nSparkamatic, I'm hangin' tight like a fanatic \nYou trapped me once and I thought that \nYou might have it \nSo step down and lend me your ear \n'89 in my time! You, '90 is my year. \n\nYou're weakenin' fast, YO! and I can tell it \nYour body's gettin' hot, so, so I can smell it \nSo don't be mad and don't be sad \n'Cause the lyrics belong to ICE, You can call me Dad \nYou're pitchin' a fit, so step back and endure \nLet the witch doctor, Ice, do the dance to cure \nSo come up close and don't be square \nYou wanna battle me -- Anytime, anywhere \n\nYou thought that I was weak, Boy, you're dead wrong \nSo come on, everybody and sing this song \n\nSay -- Play that funky music Say, go white boy, go white boy go \nplay that funky music Go white boy, go white boy, go \nLay down and boogie and play that funky music till you die. \n\nPlay that funky music Come on, Come on, let me hear \nPlay that funky music white boy you say it, say it \nPlay that funky music A little louder now \nPlay that funky music, white boy Come on, Come on, Come on \nPlay that funky music \n\u{4}\u{4}\u{4}\u{4}", decrypted_str);
    }
}
