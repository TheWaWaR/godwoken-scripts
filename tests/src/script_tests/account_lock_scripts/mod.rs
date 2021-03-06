use ckb_types::bytes::Bytes;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SECP256K1_DATA_BIN: Bytes = Bytes::from(
        &include_bytes!(
            "../../../../../godwoken-scripts/c/deps/ckb-production-scripts/build/secp256k1_data"
        )[..]
    );
}
