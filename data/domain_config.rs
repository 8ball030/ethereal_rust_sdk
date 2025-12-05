
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
static DOMAINS: EnvDomains = EnvDomains {

    testnet: DomainConfig {
        name: "Ethereal",
        version: "1",
        chain_id: 13374202,
        verifying_contract: "0x1F0327A80e43FEF1Cd872DC5d38dCe4A165c0643",
    },


    mainnet: DomainConfig {
        name: "Ethereal",
        version: "1",
        chain_id: 5064014,
        verifying_contract: "0xB3cDC82035C495c484C9fF11eD5f3Ff6d342e3cc",
    },

};
