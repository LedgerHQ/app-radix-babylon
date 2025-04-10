use crate::io::Comm;
use sbor::bech32::address::Address;
use sbor::bech32::encoder::Bech32;
use sbor::bech32::network::NetworkId;
use sbor::static_vec::StaticVec;

use crate::xui::address;

pub fn verify_address(address: Address, network_id: NetworkId, comm: &mut Comm) {
    let mut vec = StaticVec::<u8, { Bech32::MAX_LEN }>::new(0);
    address.format(&mut vec, network_id);

    address::display(vec.as_slice());
    comm.append(vec.as_slice());
}
