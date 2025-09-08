use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::info_address_port_pair::InfoAddressPortPair;

#[derive(Clone)]
pub struct ReportEntry {
    pub key: AddressPortPair,
    pub val: InfoAddressPortPair,
    pub is_blacklisted: bool,
}
