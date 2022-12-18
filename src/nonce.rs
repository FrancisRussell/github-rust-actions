use crate::system::rng::MathRandomRng;
use rust_toolchain_manifest::HashValue;

pub fn build_nonce(num_bytes: usize) -> HashValue {
    let mut bytes = vec![0u8; num_bytes];
    let mut rng = MathRandomRng::default();
    rng.fill_bytes(&mut bytes);
    HashValue::from_bytes(&bytes)
}
