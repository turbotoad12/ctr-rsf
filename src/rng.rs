use rand::Rng;

/// Generates a valid random 3DS UniqueId (20‑bit).
/// Range: 0x00000 ..= 0xFFFFF
pub fn generate_unique_id() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0x00000..=0xFFFFF)
}
