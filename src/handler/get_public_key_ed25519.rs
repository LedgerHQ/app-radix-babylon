use nanos_sdk::io::Comm;

use crate::app_error::AppError;
use crate::crypto::bip32::Bip32Path;
use crate::crypto::ed25519::KeyPair25519;
use crate::utilities::conversion::{to_hex_str, to_str};
use crate::utilities::{debug, debug_prepared_message};

pub fn handle(comm: &mut Comm) -> Result<(), AppError> {
    Bip32Path::read(comm)
        .and_then(|path| path.validate())
        .and_then(|path| KeyPair25519::derive(&path))
        .map(|key| {
            comm.append(key.public());
            ()
        })
}