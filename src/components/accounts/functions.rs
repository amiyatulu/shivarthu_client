use subxt_signer::{ SecretUri, sr25519::Keypair };
use std::str::FromStr;

use subxt_signer::sr25519::dev;


pub (super) fn get_from_seed(seed: &str) ->  Keypair {
    let uri = SecretUri::from_str(seed).unwrap();
    let keypair = Keypair::from_uri(&uri).unwrap();
    keypair
}






