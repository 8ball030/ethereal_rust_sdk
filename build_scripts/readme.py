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
    code_block_pattern = re.compile(r'```rust\n(.*?)\n```', re.DOTALL)
    snippets = code_block_pattern.findall(readme_content)
    return snippets[1:]  # Skip first snippet


def extract_example_code(example_path):
    """Extract the main code from an example Rust file."""
    with open(example_path, 'r') as f:
        example_content = f.read()
    return example_content


def test_readme_snippets():
    """Test that README code snippets match the example files."""
    project_root = Path(__file__).parent.parent
    readme_path = project_root / 'readme.md'
    
    if not readme_path.exists():
        raise FileNotFoundError(f"README file not found at {readme_path}")
    
    snippets = extract_code_snippets(readme_path)
    
    if not snippets:
        raise ValueError("No Rust code snippets found in README")

    results = []
    errors = []
    
    for i, snippet in enumerate(snippets, 1):
        lines = snippet.split('\n')
        
        if not lines:
            errors.append(f"Snippet {i}: Empty snippet found")
            continue
            
        # Extract the first line to determine the example file name
        first_line = lines[0]
        
        # Check if first line is a comment with file path
        if not first_line.strip().startswith('//'):
            errors.append(f"Snippet {i}: First line is not a comment with file path: {first_line[:50]}")
            continue
        
        example_file_name = first_line.split('//')[-1].strip()
        
        if not example_file_name:
            errors.append(f"Snippet {i}: Could not extract filename from comment: {first_line}")
            continue
        
        # Build absolute path from project root
        example_path = project_root / example_file_name
        
        if not example_path.exists():
            errors.append(f"Snippet {i}: Example file not found: {example_path}")
            continue

        try:
            example_code = extract_example_code(example_path)
        except Exception as e:
            errors.append(f"Snippet {i}: Failed to read {example_file_name}: {e}")
            continue

        # Remove the first line (comment) from the snippet for comparison
        snippet_without_comment = '\n'.join(lines[1:])
        
        if not snippet_without_comment.strip():
            errors.append(f"Snippet {i}: Snippet is empty after removing comment line")
            continue

        # Exact comparison - snippet must match example code exactly
        is_exact_match = snippet_without_comment == example_code
        
        results.append((example_file_name, is_exact_match))
        
        # If not matching, show detailed diff
        if not is_exact_match:
            snippet_lines = snippet_without_comment.split('\n')
            example_lines = example_code.split('\n')
            
            # Find first differing line
            first_diff_line = None
            for line_num, (s_line, e_line) in enumerate(zip(snippet_lines, example_lines), 1):
                if s_line != e_line:
                    first_diff_line = line_num
                    errors.append(
                        f"Snippet {i} ({example_file_name}): Mismatch at line {line_num}\n"
                        f"    README: '{s_line}'\n"
                        f"    Example: '{e_line}'"
                    )
                    break
            
            if first_diff_line is None:
                # Lines match but different number of lines
                errors.append(
                    f"Snippet {i} ({example_file_name}): Different number of lines\n"
                    f"    README: {len(snippet_lines)} lines\n"
                    f"    Example: {len(example_lines)} lines"
                )
    
    # Print all results
    print(f"\nProcessed {len(snippets)} code snippets from README\n")
    
    for example_file_name, result in results:
        status = '✅ PASSED' if result else '❌ FAILED'
        print(f"  {status}: {example_file_name}")
    
    # Print all errors
    if errors:
        print(f"\n{'='*80}")
        print(f"ERRORS FOUND ({len(errors)}):")
        print('='*80)
        for error in errors:
            print(f"\n{error}")
        print('='*80)
    
    # Final assertion
    if errors:
        raise AssertionError(f"\nFound {len(errors)} error(s) in README code snippets. Fix the README to match example files exactly.")
    
    print(f"\n✅ SUCCESS! All {len(results)} README code snippets match their example files exactly!")


if __name__ == "__main__":
    test_readme_snippets()