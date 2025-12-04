"""
Simple post-processing script for generated code.
"""
from pathlib import Path
from string import Template

SOURCE_DIR = Path(__file__).parent.parent / "src" / "apis"

SYNC_CLIENT_PATH = Path(__file__).parent.parent / "src" / "sync_client"

SUBCLIENT_TEMPLATE = Template("""
pub struct ProductClient<'a> {
    pub config: &'a Configuration,
}

impl<'a> ProductClient<'a> {
{methods}
""")

METHOD_TEMPLATE = Template("""
    pub fn $short_function_name(
        &self,
        $params_line
    ) -> Result<$return_type, Error<$error_name>> {
        $function_name(self.config, $params)
    }
""")

TEST_METHOD_TEMPLATE_WITH_PARAMS = Template("""
#[test]
fn test_$short_function_name() {
    let client = HttpClient::new(Environment::Testnet);
    let params = $params_struct_name::default();
    let result = client.$api_name().$short_function_name(params);
    assert!(result.is_ok());
}
""")
TEST_METHOD_TEMPLATE_WITHOUT_PARAMS = Template("""
#[test]
fn test_$short_function_name() {
    let client = HttpClient::new(Environment::Testnet);
    let result = client.$api_name().$short_function_name();
    assert!(result.is_ok());
}
""")

TEST_API_TEMPLATE = Template("""
use ethereal_rust_sdk::sync_client::client::HttpClient;
use ethereal_rust_sdk::enums::Environment;
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
    sync_client_content = (SYNC_CLIENT_PATH / "client.rs").read_text()
    api_name = file_path.stem.split("_api")[0]
    expected_client_name = f"{api_name.capitalize()}Client"
    if expected_client_name not in sync_client_content:
        print(f"    Warning: {SYNC_CLIENT_PATH} does not have client for {expected_client_name}")

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
        if line.startswith("pub fn "):
            func_started = True
            long_method_name = line.split("pub fn ")[1].split("(")[0]
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
    sub_client_file = SYNC_CLIENT_PATH / f"{api_name[:-4]}.rs"
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
    mod_file = SYNC_CLIENT_PATH / "mod.rs"
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
    tests_file = SYNC_CLIENT_PATH.parent.parent / "tests" / f"test_{api_name[:-4]}.rs"
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
            # write_tests_file(
            #     api_name=file.stem,
            #     tests=tests,
            #     client_imports=client_imports
            # )


if __name__ == "__main__":
    generated_files = gather_generated_files(SOURCE_DIR)
    post_process_generated_files(generated_files)
