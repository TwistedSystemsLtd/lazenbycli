extern crate clap;
extern crate sodiumoxide;

#[cfg(test)]
mod tests {
    use sodiumoxide::crypto::box_;

    #[test]
    fn can_use_crypto_box() {
        let (ourpk, oursk) = box_::gen_keypair();
// normally theirpk is sent by the other party
        let (theirpk, theirsk) = box_::gen_keypair();
        let nonce = box_::gen_nonce();
        let plaintext = b"some data";
        let ciphertext = box_::seal(plaintext, &nonce, &theirpk, &oursk);
        let their_plaintext = box_::open(&ciphertext, &nonce, &ourpk, &theirsk).unwrap();
        assert_eq!(plaintext, &their_plaintext[..]);
    }
}

use clap::App;

fn main() {
    App::new("fake").version("v1.0-beta").get_matches();
}