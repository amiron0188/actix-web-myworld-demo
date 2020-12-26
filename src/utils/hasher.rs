use libreauth::pass::{Algorithm, HashBuilder, Hasher, };

pub const PWD_ALGORITHM: Algorithm = Algorithm::Argon2;
pub const PWD_SCHEME_VERSION: usize = 1;

lazy_static! {
    pub static ref HASHER: Hasher = {
        HashBuilder::new()
            .algorithm(PWD_ALGORITHM)
            .version(PWD_SCHEME_VERSION)
            .min_len(4)
            .finalize()
            .unwrap()
    };
}