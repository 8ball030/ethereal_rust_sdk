use crate::enums::Environment;
#[derive(Clone)]
pub struct DomainConfig {
    pub name: &'static str,
    pub version: &'static str,
    pub chain_id: u64,
    pub verifying_contract: &'static str,
}

pub struct EnvDomains {
    pub testnet: DomainConfig,
    pub mainnet: DomainConfig,
}
impl EnvDomains {
    pub fn get(&self, env: Environment) -> &DomainConfig {
        match env {
            Environment::Testnet => &self.testnet,
            Environment::Mainnet => &self.mainnet,
        }
    }
}
pub static DOMAINS: EnvDomains = EnvDomains {
    testnet: DomainConfig {
        name: "Meridian",
        version: "1",
        chain_id: 46630,
        verifying_contract: "0x09c9DE924BEDB93A01FA9e7beCa3a078b99E1B3b",
    },

    mainnet: DomainConfig {
        name: "Meridian",
        version: "1",
        chain_id: 4663,
        verifying_contract: "0xD540F47F214dC7D6D244E62A6aE7e06B586Ef44A",
    },
};
