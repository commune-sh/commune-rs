use rand::seq::IteratorRandom;

pub fn generate_comforming_localpart() -> String {
    let allowed = ('0'..='9')
        .chain('a'..='z')
        .chain(['-', '.', '=', '_', '/', '+']);
    allowed
        .choose_multiple(&mut rand::thread_rng(), 8)
        .into_iter()
        .collect()
}
