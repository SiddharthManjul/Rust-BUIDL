use ethers::types::Address;
use std::str::FromStr;

#[allow(dead_code)]
trait EthereumAddress {
    fn convert_address(&self) -> Result<Address, &'static str>;
}

impl EthereumAddress for &str {
    fn convert_address(&self) -> Result<Address, &'static str> {
        match Address::from_str(self) {
            Ok(address) => Ok(address),
            Err(_) => Err("Invalid Ethereum Address String")
        }
    }
}

impl EthereumAddress for Address {
    fn convert_address(&self) -> Result<Address, &'static str> {
        Ok(*self)
    }
}

#[allow(dead_code)]
fn get_ethereum_address<T: EthereumAddress>(address: T) -> Address {
    let converted_address: Address = address.convert_address().unwrap();
    converted_address
}

#[cfg(test)]
mod test {

    use super::*;

    #[test] 
    fn tests_poly() {
        let addr: Address = Address::from_str("0x93FF93157290652bc3d20bE34907D47e9c27B06f").unwrap();

        let new_addr: Address = get_ethereum_address(addr);
        assert_eq!(new_addr, Address::from_str("0x93FF93157290652bc3d20bE34907D47e9c27B06f").unwrap());
    }
}