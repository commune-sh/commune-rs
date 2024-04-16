use rand::{distributions::Alphanumeric, Rng};

pub fn generate_comforming_localpart() -> String {
    // Synapse does not allow usernames to start with '_' despite
    // the specification doing so.

    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from) // From link above, this is needed in later versions
        .collect();

    s.to_lowercase()
}
