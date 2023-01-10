use sp_core::{ed25519, sr25519, Pair};


pub (super) fn get_from_seed_sr(seed: &str) -> sr25519::Pair {
    sr25519::Pair::from_string(&format!("{}", seed), None)
        .expect("static values are valid; qed")
}

pub (super) fn get_from_seed_ed(seed: &str) -> ed25519::Pair {
    ed25519::Pair::from_string(&format!("{}", seed), None)
        .expect("static values are valid; qed")
}

