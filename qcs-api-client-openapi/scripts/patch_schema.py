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

def openapi_compat_3_0(obj, ref=[]):
    """
    Replaces OpenAPI features not compatible with v3.0 with a best-effort compatible alternative.
    See https://swagger.io/docs/specification/data-models/keywords/#unsupported
    """

    if not isinstance(obj, dict):
        return

    # A const is the same as a 1-member enum
    if "const" in obj:
        obj["enum"] = [obj["const"]]
        del obj["const"]

    # Our usage of pattern properties can be simulated with `additionalProperties`,
    # when a single pattern is the only option. We cannot simulate enforcement of
    # the property key names, so a note is added to the description.
    # Multiple patterns cannot be simulated - in that case an error will be raised.
    #
    ### Before
    # patternProperties:
    #   \d+:
    #     $ref: "#/components/schemas/LegacyNodeDescriptor"
    #
    ### After
    # additionalProperies:
    #   $ref: "#/components/schemas/LegacyNodeDescriptor"
    if "patternProperties" in obj:
        if "additionalProperties" in obj:
            raise ValueError(
                f"For {'.'.join(ref)}, cannot reconcile both 'patternProperties' and 'additionalProperties'"
            )
        props = list(obj["patternProperties"].items())
        numProps = len(props)
        if numProps == 0:
            raise ValueError(
                f"For {'.'.join(ref)}, empty 'patternProperties'"
            )
        if numProps > 1:
            raise ValueError(
                f"For {'.'.join(ref)}, cannot reconcile more than one 'patternProperties' (found {numProps})"
            )
        k, v = props[0]
        obj["additionalProperties"] = v
        obj["description"] = (
            obj.get("description", "")
            + f"\n\nProperty keys must have the format: {k}"
        ).strip()

        del obj["patternProperties"]

    for k, v in obj.items():
        openapi_compat_3_0(v, [*ref, k])


def use_oneof_not_anyof(obj):
    """
    The distinction between these types of unions lend themselves toward different data structures:
    - `oneOf` matches exactly one subschema, an `enum` suits this well.
    - `anyOf` matches one or more subschemas, a "superset struct" of all fields is necessary.

    Our schema uses `anyOf` because it's derived from FastAPI tooling
    which is correctly interpreting python's `Union` as a superset of all members,
    but in our case these unions are intended to represent mutually exclusive members.

    Regardless, the OpenAPI generator has poor support for `anyOf` and generates incorrect structures
    when any subschemas are primitives or have conflicting types for a given field.

    > Example python source models:
    https://gitlab.com/rigetti/share/domain-model-specification/-/blob/master/Python/rigetti_domain_model/models/schedule_ir/instruction_parameters.py#L34-48

    > FastAPI documentation;
    https://fastapi.tiangolo.com/tutorial/extra-models/?h=anyof#union-or-anyof

    > Swagger documentation:
    https://swagger.io/docs/specification/data-models/oneof-anyof-allof-not/#anyof-vs-oneof

    > OpenAPI generator rust templates:
    https://github.com/OpenAPITools/openapi-generator/tree/master/modules/openapi-generator/src/main/resources/rust
    """

    if not isinstance(obj, dict):
        return

    if "anyOf" in obj:
        obj["oneOf"] = obj["anyOf"]
        del obj["anyOf"]

    for _, v in obj.items():
        use_oneof_not_anyof(v)


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print(f"Usage: {sys.argv[0]} <path-to-input-schema>")
    input_file_path = sys.argv[1]
    document = yaml.load(open(input_file_path, "r"), Loader=yaml.SafeLoader)

    fix_x_qcs_headers(document)
    title_conflicting_type_properties(document)
    use_oneof_not_anyof(document)
    openapi_compat_3_0(document)

    output_path = Path(input_file_path)
    output_path = output_path.with_name(output_path.stem + "-patched" + output_path.suffix)

    yaml.dump(document, open(output_path, "w"))
