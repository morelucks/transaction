// use std::io::Chain;

#[derive(Debug)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub value: u8,
    pub gas_limit: u8,
    pub max_priority_fee_per_gas: u8,
    pub data: String,
    pub max_fee_per_gas: u8,
    pub chain: Network,
}

#[derive(Debug)]
pub enum Network {
    Base,
    Ethereum,
    Lisk,
}

impl Transaction {
    //..... Creates a new transaction "contructor".....//
    pub fn new(
        from: String,
        to: String,
        value: u8,
        gas_limit: u8,
        max_priority_fee_per_gas: u8,
        data: String,
        max_fee_per_gas: u8,
        chain: Network,
    ) -> Self {
        Self {
            from,
            to,
            value,
            gas_limit,
            max_priority_fee_per_gas,
            data,
            max_fee_per_gas,
            chain,
        }
    }
}
