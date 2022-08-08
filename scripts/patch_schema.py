"""Edit schema files to fix broken code generation

Requires the dependencies in requirements.txt
"""
import sys
from pathlib import Path

import yaml


def fix_x_qcs_headers(schema):
    # The generator is unhappy with the account type header unless it's within a allOf
    current_value = schema["components"]["parameters"]["accountTypeHeader"]["schema"]
    schema["components"]["parameters"]["accountTypeHeader"]["schema"] = {
        "allOf": [current_value]
    }

    # The names are also wonky
    name = schema["components"]["parameters"]["accountTypeHeader"]["name"]
    schema["components"]["parameters"]["accountTypeHeader"]["name"] = name.lower()

    name = schema["components"]["parameters"]["accountIdHeader"]["name"]
    schema["components"]["parameters"]["accountIdHeader"]["name"] = name.lower()


def title_conflicting_type_properties(obj, parent=None):
    """
    Underscore prefixes are removed when generating code, which causes syntax errors like:

    ```rust
    struct Thing {
        type: First,
        type: Second,
    }
    ```
    """

    if not isinstance(obj, dict):
        return

    if "_type" in obj and "type" in obj:
        del obj["_type"]
        parent["description"] = (
            parent.get("description", "")
            + "\n\n    Caution: the `_type` property was removed from this model but can still be accessed via "
            "`.additional_properties`"
        ).strip()

    for _, v in obj.items():
        title_conflicting_type_properties(v, obj)


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print(f"Usage: {sys.argv[0]} <path-to-input-schema>")
    input_file_path = sys.argv[1]
    document = yaml.load(open(input_file_path, "r"), Loader=yaml.SafeLoader)

    fix_x_qcs_headers(document)
    title_conflicting_type_properties(document)

    output_path = Path(input_file_path)
    output_path = output_path.with_name(output_path.stem + "-patched" + output_path.suffix)

    yaml.dump(document, open(output_path, "w"))
