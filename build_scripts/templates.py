"""
Templates for generating Rust SDK code.
"""
from string import Template

SUBCLIENT_TEMPLATE = Template("""
pub struct ProductClient<'a> {
    pub config: &'a Configuration,
}

impl<'a> ProductClient<'a> {
{methods}
""")

METHOD_TEMPLATE = Template("""
    pub async fn $short_function_name(
        &self,
        $params_line
    ) -> Result<$return_type, Error<$error_name>> {
        $function_name(self.config, $params).await
    }
""")

TEST_METHOD_TEMPLATE_WITH_PARAMS = Template("""
#[test]
fn test_$short_function_name() {
    let client = common::create_test_client().unwrap();
    let params = $params_struct_name::default();
    let result = client.$api_name().$short_function_name(params);
    assert!(result.is_ok());
}
""")
TEST_METHOD_TEMPLATE_WITHOUT_PARAMS = Template("""
#[test]
fn test_$short_function_name() {
    let client = common::create_test_client().unwrap();
    let result = client.$api_name().$short_function_name();
    assert!(result.is_ok());
}
""")

TEST_API_TEMPLATE = Template("""
mod common;
use ethereal_rust_sdk::apis::$api_name::{$client_imports};
""")

SUB_CLIENT_TEMPLATE = Template("""
use crate::{
    apis::{
        Error,
        configuration::Configuration,
        $api_name::{$client_imports},
    },
    models::{$model_imports},
};
pub struct $client_name<'a> {
    pub config: &'a Configuration,
}

impl<'a> $client_name<'a> {
$functions
}
""")


CONFIG_TEMPLATE = Template("""
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
$config_values
};
""")

CONFIG_VALUES_TEMPLATE = Template("""
    $environment: DomainConfig {
        name: "$name",
        version: "$version",
        chain_id: $chain_id,
        verifying_contract: "$verifying_contract",
    },
""")

SIGNABLE_MESSAGE_HEADER = """
use ethers::types::transaction::eip712::Eip712Error;
use ethers::types::{Address};

use ethers::{types::U256};
use serde::{Deserialize, Serialize};

use crate::signing::Eip712;

"""
SIGNABLE_MESSAGE_TEMPLATE = Template("""
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct $message_name{
$fields
}
impl Eip712 for $message_name {
    fn type_hash() -> Result<[u8; 32], Eip712Error> {
        Ok(ethers::utils::keccak256(
            "$message_name($struct)"
        ))
    }

    fn struct_hash(&self) -> Result<[u8; 32], Eip712Error> {
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&Self::type_hash()?);
        encoded.extend_from_slice(&ethers::abi::encode(&[
$fields_encoding
        ]));

        Ok(ethers::utils::keccak256(&encoded))
    }
}
""")