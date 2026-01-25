"""
Simple post-processing script for generated code.
"""
import json
from pathlib import Path

from templates import (
    METHOD_TEMPLATE,
    SIGNABLE_MESSAGE_HEADER,
    SIGNABLE_MESSAGE_TEMPLATE,
    TEST_API_TEMPLATE,
    TEST_METHOD_TEMPLATE_WITH_PARAMS,
    TEST_METHOD_TEMPLATE_WITHOUT_PARAMS,
    SUB_CLIENT_TEMPLATE,
    CONFIG_TEMPLATE,
    CONFIG_VALUES_TEMPLATE
)

CRATE_ROOT = Path(__file__).parent.parent / "src"
API_SOURCE_DIR = CRATE_ROOT/ "apis"
ASYNC_CLIENT_PATH = CRATE_ROOT / "async_client"
DATA_DIR = Path(__file__).parent.parent / "data"



def gather_generated_files(generated_dir: Path):
    """
    Gather all generated Rust files from the specified directory.
    """
    return list(generated_dir.rglob("*.rs"))

def to_upper_camel_case(snake_str: str) -> str:
    """
    Convert a snake_case string to UpperCamelCase.
    """
    components = snake_str.split('_')
    return ''.join(x.title() for x in components)

def to_snake_case(camel_str: str) -> str:
    """
    Convert an UpperCamelCase string to snake_case.
    """
    snake_str = ''
    for i, char in enumerate(camel_str):
        if char.isupper() and i != 0:
            snake_str += '_'
        snake_str += char.lower()
    return snake_str

def add_default_derive_attribute_to_parameter_structs(file_path: Path):
    """
    Add #[derive(Default)] to parameter structs in the given Rust file.
    """
    content = file_path.read_text()
    lines = content.splitlines()
    modified_lines = []
    skip_next = False

    for i, line in enumerate(lines):
        if skip_next:
            skip_next = False
            continue

        if line.strip().startswith("pub struct") and "Params" in line:
            # Check if the previous line is already a derive attribute
            if i > 0 and lines[i - 1].strip().startswith("#[derive("):
                if "Default" not in lines[i - 1]:
                    # Modify the existing derive attribute
                    modified_lines[-1] = modified_lines[-1].rstrip(")]") + ", Default)]"
            else:
                # Insert a new derive attribute
                modified_lines.append("#[derive(Default)]")
            modified_lines.append(line)
        else:
            modified_lines.append(line)

    modified_content = "\n".join(modified_lines)
    file_path.write_text(modified_content)

def check_sync_client_has_sub_client(file_path: Path):
    """
    Check that the sync client has sub-client methods.
    """
    sync_client_content = (ASYNC_CLIENT_PATH / "client.rs").read_text()
    api_name = file_path.stem.split("_api")[0]
    expected_client_name = f"{api_name.capitalize()}Client"
    if expected_client_name not in sync_client_content:
        print(f"    Warning: {ASYNC_CLIENT_PATH} does not have client for {expected_client_name}")

def check_sub_client_methods(file_path: Path):
    """
    Check that the sync client has methods for sub-clients.
    """
    api_name = file_path.stem.split("_api")[0]
    # Extract all expected method names for sub-clients
    source_content = file_path.read_text()
    lines = source_content.splitlines()
    to_remove = f"{api_name}_controller_"
    func_started = False
    model_imports = set()
    client_imports = set()
    functions, tests = [], []
    for ix, line in enumerate(lines):
        if line.startswith("pub async fn "):
            func_started = True
            long_method_name = line.split("pub async fn ")[1].split("(")[0]
            short_method_name = long_method_name.replace(to_remove, "")
            print(f"    found: {short_method_name} in {api_name} sub-client at line {ix+1}")
        if func_started and "Result<" in line:
            func_started = False
            # if short_method_name not in sync_client_content:
            error_name = line.split("Error<")[1].split(">")[0]
            return_type = line.split("Result<models::")[1].split(",")[0]

            params_line = ""
            params_struct_name = ""
            if "params: " in line:
                params_struct_name = line.split("params: ")[1].split(")")[0]
                params_line = f"params: {params_struct_name},"
                client_imports.add(params_struct_name)

            templated_function = METHOD_TEMPLATE.substitute(
                short_function_name=short_method_name,
                params_line=params_line,
                params="params" if params_line else "",
                function_name=long_method_name,
                error_name=error_name,
                return_type=return_type
            )
            if params_line == "":
                templated_test_function = TEST_METHOD_TEMPLATE_WITHOUT_PARAMS.substitute(
                    short_function_name=short_method_name,
                    api_name=api_name
                )
            else:
                templated_test_function = TEST_METHOD_TEMPLATE_WITH_PARAMS.substitute(
                    short_function_name=short_method_name,
                    api_name=api_name,
                    params_struct_name=params_struct_name

                )
            tests.append(templated_test_function)
            functions.append(templated_function)
            model_imports.add(return_type)
            client_imports.add(error_name)
            client_imports.add(long_method_name)
    return functions, tests, model_imports, client_imports


def write_sub_client_file(api_name: str, functions: list[str], model_imports: set[str], client_imports: set[str]):
    sub_client_file = ASYNC_CLIENT_PATH / f"{api_name[:-4]}.rs"
    client_name = f"{to_upper_camel_case(api_name[:-4])}Client"
    functions_str = "\n".join(functions)
    model_imports_str = ", ".join(sorted(model_imports))
    client_imports_str = ", ".join(sorted(client_imports))
    sub_client_content = SUB_CLIENT_TEMPLATE.substitute(
        api_name=api_name,
        client_name=client_name,
        functions=functions_str,
        model_imports=model_imports_str,
        client_imports=client_imports_str
    )
    sub_client_file.write_text(sub_client_content)
    print(f"    Wrote sub-client file: {sub_client_file.stem}")


def include_sub_client_in_mod_file(api_name: str):
    """
    Include the sub-client module in the sync_client mod.rs file.
    """
    mod_file = ASYNC_CLIENT_PATH / "mod.rs"
    mod_content = mod_file.read_text()
    sub_client_mod_line = f"mod {api_name[:-4]};"
    if sub_client_mod_line not in mod_content:
        with mod_file.open("a") as mf:
            mf.write(f"\n{sub_client_mod_line}")
        print(f"    Updated mod.rs to include {api_name[:-4]} sub-client.")

def write_tests_file(api_name: str, tests: list[str], client_imports: set[str]):
    """
    Write the tests file for the sub-client.
    """
    tests_file = ASYNC_CLIENT_PATH.parent.parent / "tests" / f"test_{api_name[:-4]}.rs"
    if tests_file.exists():
        return
    tests_file.touch()
    tests_content = TEST_API_TEMPLATE.substitute(
        api_name=api_name,
        client_imports=", ".join(sorted(client_imports))
    )
    tests_content += "\n\n" + "\n".join(tests)
    tests_file.write_text(tests_content)
    print(f"    Wrote tests file: {tests_file.stem}")

def post_process_generated_files(generated_files: list[Path]):
    """
    Post-process the gathered generated files.
    """
    for file in generated_files:
        # Perform any necessary post-processing on each file
        add_default_derive_attribute_to_parameter_structs(file)
        if "_api" in file.stem:
            print("Processing API file:", file.stem)
            check_sync_client_has_sub_client(file)
            functions, tests, model_imports, client_imports = check_sub_client_methods(file)
            write_sub_client_file(
                api_name=file.stem,
                functions=functions,
                model_imports=model_imports,
                client_imports=client_imports
            )
            include_sub_client_in_mod_file(file.stem)
            write_tests_file(
                api_name=file.stem,
                tests=tests,
                client_imports=client_imports
            )

def generate_domain_config_files():
    """
    Generate domain configuration files for different environments.
    """
    environments = {
        "testnet": DATA_DIR / "testnet" / "rpc_config.json",
        "mainnet": DATA_DIR / "mainnet" / "rpc_config.json",
    }
    domain_config_values = []
    for env_name, config_path in environments.items():
        if not config_path.exists():
            print(f"    Warning: Config file {config_path} does not exist.")
            continue
        config_data = json.loads(config_path.read_text())
        domain_info = config_data.get("domain", {})
        config_values = CONFIG_VALUES_TEMPLATE.substitute(
            environment=env_name,
            name=domain_info.get("name", "Ethereal"),
            version=domain_info.get("version", "1"),
            chain_id=domain_info.get("chainId", 13374202),
            verifying_contract=domain_info.get("verifyingContract", "0x1F0327A80e43FEF1Cd872DC5d38dCe4A165c0643"),
        )
        domain_config_values.append(config_values)

    config_file_path = CRATE_ROOT / "domain_config.rs"
    if not config_file_path.exists():
        config_file_path.touch()
    config_content = CONFIG_TEMPLATE.substitute(
        config_values="\n".join(domain_config_values),
    )
    config_file_path.write_text(config_content)


def gather_signable_messages():
    """
    Gather signable message structs from the models directory.
    Field: sender, Type: address
    Field: subaccount, Type: bytes32
    Field: quantity, Type: uint128
    Field: price, Type: uint128
    Field: reduceOnly, Type: bool
    Field: side, Type: uint8
    Field: engineType, Type: uint8
    Field: productId, Type: uint32
    Field: nonce, Type: uint64
    Field: signedAt, Type: uint64

    """

    sol_to_rust_type_map = {
        "address": "Address",
        "bytes32": "[u8; 32]",
        "uint128": "u128",
        "uint256": "u128",
        "uint64": "u64",
        "uint32": "u32",
        "uint8": "u8",
        "bool": "bool",
    }
    sol_to_ethers_type_map = {
        "address": "Address(self.{name})",
        "bytes32": "FixedBytes(self.{name}.to_vec())",
        "uint128": "Uint(U256::from(self.{name}))",
        "uint256": "Uint(self.{name}.clone().into())",
        "bool": "Bool(self.{name})",
        "uint8": "Uint(U256::from(self.{name}))",
        "uint32": "Uint(U256::from(self.{name}))",
        "uint64": "Uint(U256::from(self.{name}))",
    }
    spec_path = DATA_DIR/  "mainnet" / "rpc_config.json"
    sig_types = json.loads(spec_path.read_text())["signatureTypes"]
    generated_types = []

    for message_name, struct in sig_types.items():
        fields, fields_encoding = [], []
        for value in struct.split(","):
            t, type_name = value.split(" ")
            field = f"    pub {to_snake_case(type_name)}: {sol_to_rust_type_map[t]},"
            fields.append(field)
            ethers_type = sol_to_ethers_type_map[t].format(name=to_snake_case(type_name))
            fields_encoding.append(f"                ethers::abi::Token::{ethers_type},")
        generated_file = SIGNABLE_MESSAGE_TEMPLATE.substitute(
            message_name=message_name,
            fields="\n".join(fields),
            struct=struct,
            fields_encoding="\n".join(fields_encoding)
        )
        generated_types.append(generated_file)
    signable_messages_file = CRATE_ROOT / "signable_messages.rs"
    signable_messages_file.write_text(SIGNABLE_MESSAGE_HEADER + "\n".join(generated_types))

    return fields

if __name__ == "__main__":
    generate_domain_config_files()
    generated_files = gather_generated_files(API_SOURCE_DIR)
    post_process_generated_files(generated_files)
    gather_signable_messages()
    print("Post-processing completed.")
