"""
Simple tests to ensure that the README code snippets are directly lifted from the example files.
"""
import re
from pathlib import Path

def extract_code_snippets(readme_path):
    """Extract Rust code snippets from the README file."""
    with open(readme_path, 'r') as f:
        readme_content = f.read()

    # Regex to find Rust code blocks
    code_block_pattern = re.compile(r'```rust(.*?)```', re.DOTALL)
    snippets = code_block_pattern.findall(readme_content)
    return [snippet.strip() for snippet in snippets][1:]

def extract_example_code(example_path):
    """Extract the main code from an example Rust file."""
    with open(example_path, 'r') as f:
        example_content = f.read()
    return example_content.strip()

def test_readme_snippets():
    """Test that README code snippets match the example files."""
    readme_path = Path(__file__).parent.parent / 'readme.md'
    snippets = extract_code_snippets(readme_path)

    results = []
    for snippet in snippets:
        # Extract the first line to determine the example file name
        first_line = snippet.splitlines()[0]
        example_file_name = first_line.split('//')[-1].strip()
        example_path = Path(example_file_name)

        if not example_path.exists():
            raise FileNotFoundError(f"Example file {example_file_name} not found for snippet.")

        example_code = extract_example_code(example_path)

        # remove the first line from the snippet for comparison
        snippet = '\n'.join(snippet.splitlines()[1:]).strip()

        # Normalize whitespace for comparison
        normalized_snippet = re.sub(r'\s+', ' ', snippet)
        normalized_example_code = re.sub(r'\s+', ' ', example_code)
        results.append((example_file_name, normalized_snippet in normalized_example_code))
    for example_file_name, result in results:
        print(f"Testing snippet from {example_file_name}: {'Passed' if result else 'Failed'}")
    
    assert all(result for _, result in results), "Some README code snippets do not match the example files."


if __name__ == "__main__":
    test_readme_snippets()
    print("All README code snippets match the example files.")