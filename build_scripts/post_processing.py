"""
Simple post-processing script for generated code.
"""
from pathlib import Path

SOURCE_DIR = Path(__file__).parent.parent / "src" / "apis"

def gather_generated_files(generated_dir: Path):
    """
    Gather all generated Rust files from the specified directory.
    """
    return list(generated_dir.rglob("*.rs"))



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

def post_process_generated_files(generated_files: list[Path]):
    """
    Post-process the gathered generated files.
    """
    for file in generated_files:
        # Perform any necessary post-processing on each file
        add_default_derive_attribute_to_parameter_structs(file)
        

if __name__ == "__main__":
    generated_files = gather_generated_files(SOURCE_DIR)
    post_process_generated_files(generated_files)
