"""
patch script to replace certain ids in openapi.json
"""

import json
from pathlib import Path
import sys

CUSTOM_ENUM_OVERRIDE = {
    "SubmitOrderMarketDtoDataOrderType": "OrderType",
    "SubmitOrderLimitDtoDataOrderType": "OrderType",
}

ENUM_NAME_OVERRIDES = {
    "TypeEnum": "OrderType",
    "StatusEnum": "OrderStatus",
    "SideEnum": "OrderSide",
    "TimeInForceEnum": "TimeInForce",
}

def read_json(file_path: Path) -> dict:
    return json.loads(file_path.read_text())

def write_json(file_path: Path, data: dict):
    file_path.write_text(json.dumps(data, indent=2))

def extract_all_enums(spec: dict):
    old_models = spec.get("components", {}).get("schemas", {})

    new_enums = dict()
    name_to_x_enum_varnames = dict()
    name_to_types = dict()
    for model_name, model_schema in old_models.items():
        for prop_name, prop_schema in model_schema.get("properties", {}).items():
            if (prop_schema.get("type") == "string" and "enum" in prop_schema) or (prop_schema.get("type") == "number" and "x-enum-varnames" in prop_schema):
                enum_name = f"{prop_name[0].upper() + prop_name[1:]}Enum"
                if enum_name in ENUM_NAME_OVERRIDES:
                    enum_name = ENUM_NAME_OVERRIDES[enum_name]

                if enum_name in CUSTOM_ENUM_OVERRIDE:
                    enum_name = CUSTOM_ENUM_OVERRIDE[enum_name]

                elif enum_name not in new_enums:
                    new_enums[enum_name] = prop_schema["enum"]
                else:
                    if new_enums[enum_name] != prop_schema["enum"]:
                        enum_name = f"{model_name}{enum_name}"
                        assert enum_name not in new_enums, f"Enum name collision for {enum_name}"
                    if enum_name in CUSTOM_ENUM_OVERRIDE:
                        enum_name = CUSTOM_ENUM_OVERRIDE[enum_name]
                    new_enums[enum_name] = prop_schema["enum"]
                # Update the property to reference the new enum
                print(f"Extracting enum for {model_name}.{prop_name} as {enum_name}")
                if "x-enum-varnames" in prop_schema:
                    print(f"Also extracting x-enum-varnames for {model_name}.{prop_name} as {enum_name}")
                    name_to_x_enum_varnames[enum_name] = prop_schema["x-enum-varnames"]
                name_to_types[enum_name] = prop_schema.get("type")
                prop_schema.clear()
                prop_schema["$ref"] = f"#/components/schemas/{enum_name}"

    print(f"Extracted {len(new_enums)} enums.")
    for enum_name, enum_values in new_enums.items():
        if enum_name == "OrderType":
            enum_values.append("MARKET")
        old_models[enum_name] = {
            "type": name_to_types.get(enum_name, "string"),
            "enum": enum_values,
            "description": f"Extracted enum for {enum_name}"
        }
        if enum_name in name_to_x_enum_varnames:
            old_models[enum_name]["x-enum-varnames"] = name_to_x_enum_varnames[enum_name]
            old_models[enum_name]["type"] = "integer"
    spec["components"]["schemas"] = old_models
    return spec


def patch_integer_properties(spec: dict):
    schemas = spec.get("components", {}).get("schemas", {})
    processed = 0
    for schema_name, schema in schemas.items():
        properties = schema.get("properties", {})
        for prop_name, prop in properties.items():
            if prop.get("type") == "integer":
                prop["format"] = "int64"
                processed += 1

    for path, path_item in spec.get("paths", {}).items():
        print(f"Processing path: {path}")
        for method, operation in path_item.items():
            if not isinstance(operation, dict):
                continue
            for param in operation.get("parameters", []):
                schema = param.get("schema", {})
                if schema.get("type") == "integer":
                    schema["format"] = "int64"
                    processed += 1
    print(f"Processed {processed} integer properties.") 
    return spec


def patch_schema_has_next_to_optional(spec: dict):
    schemas = [
        "PageOfPositionDtos", 
        "PageOfOrderFillDtos", 
        ]
    for schema_name in schemas:
        schema = spec.get("components", {}).get("schemas", {}).get(schema_name, {})
        schema["required"].remove("hasNext")
    print("Patched path parameters to be nullable where applicable.")
    return spec

def remove_deprecated_endpoints(spec: dict):
    paths = spec.get("paths", {})
    removed = 0
    empty_paths = []

    for path, path_item in paths.items():
        deprecated_methods = [
            method for method, operation in path_item.items()
            if isinstance(operation, dict) and operation.get("deprecated") is True
        ]
        for method in deprecated_methods:
            del path_item[method]
            print(f"Removed deprecated endpoint: {method.upper()} {path}")
            removed += 1
        if not path_item:
            empty_paths.append(path)

    for path in empty_paths:
        del paths[path]

    print(f"Removed {removed} deprecated endpoints.")
    return spec


def remove_default_ts_values(spec: dict):
    description = "End time of the query range (clamped to resolution, ms since Unix epoch, defaults to now)"
    for path, path_item in spec.get("paths", {}).items():
        for method, operation in path_item.items():
            if not isinstance(operation, dict):
                continue
            for param in operation.get("parameters", []):
                if param.get("description") == description:
                    print(f"Removing default value from parameter: {param['name']} in {method.upper()} {path}")
                    param.pop("default", None)
    print("Removed default timestamp values from parameters.")
    return spec

def main():
    file_path = Path("openapi.json")

    if not file_path.exists():
        print(f"File not found: {file_path}")
        sys.exit(1)
    data = read_json(file_path)
    data = remove_deprecated_endpoints(data)
    data = extract_all_enums(data)
    data = patch_schema_has_next_to_optional(data)
    data = patch_integer_properties(data)
    
    write_json(file_path, data)
    print(f"Patched: {file_path}")

    archive_file_path = Path("archive_openapi.json")
    data = read_json(archive_file_path)
    data = extract_all_enums(data)
    data = patch_integer_properties(data)
    data = remove_default_ts_values(data)
    write_json(archive_file_path, data)
    print(f"Patched: {archive_file_path}")  

if __name__ == "__main__":
    main()
