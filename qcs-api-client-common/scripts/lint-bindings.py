"""This script is a lint helper for our PyO3 wrappers.

Given a starting directory, it recursively searches it for ``*.rs`` files,
and attempts to extract PyO3 annotations and exports from the source files.
Afterward, it may print some messages about potential mistakes.
Run the script with ``--help`` to see its options.
"""

import logging
import sys

from pyo3_linter import (
    Item,
    Kind,
    MacroContext,
    PackageConfig,
    default_macro_handlers,
    find_possible_mistakes,
    iter_delim,
    join_lines,
    macro_handler,
    parser,
    print_package_info,
    process_dir,
)
from pyo3_linter.package import StubAttr, StubKind

logging.basicConfig(level=logging.WARNING, handlers=[], force=True)
logger = logging.getLogger()

def main():
    """Process Rust source files to show PyO3 layout and/or issues."""
    args = parser.get_parser().parse_args()
    configure_logger(args.log_level)

    package_config = PackageConfig(root_module="qcs_api_client_common", internal_module="_qcs_api_client_common")
    annotated, exported = process_dir(args.base, package_config, default_macro_handlers() + [_make_secret_string])

    issues = find_possible_mistakes(package_config, annotated, exported)
    if args.show_mistakes:
        for issue in issues:
            print(issue.message)

    if args.show_package:
        print_package_info(annotated)

    if issues:
        print(f"\n {len(issues)} potential issue(s) discovered.", file=sys.stderr)
        if not args.show_mistakes:
            print("  (use --show-mistakes to see)", file=sys.stderr)
        sys.exit(1)


@macro_handler(r"make_secret_string!")
def _make_secret_string(ctx: MacroContext, module: str | None = None) -> None:
    """Process the input to the ``make_secret_string!`` macro."""
    line = join_lines(iter_delim(ctx.lines, "()"))
    rust_name = line.text.replace(" ", "").removeprefix("make_secret_string!(").removesuffix(");").strip()
    item = Item(
        kind=Kind.Class,
        python_name=rust_name,
        rust_name=rust_name,
        path=ctx.path,
        line=line,
        stub_attr=StubAttr(kind=StubKind.Class, module="qcs_api_client_common.configuration"),
    )

    ctx.annotated["qcs_api_client_common.configuration"].add(item)
    ctx.exported["qcs_api_client_common.configuration"].add(item)


@macro_handler(r"impl_instruction!")
def _impl_instruction(ctx: MacroContext, module: str | None = None) -> None:
    """Process the input to the ``impl_instruction!`` macro."""
    line = join_lines(iter_delim(ctx.lines, "[]"))
    ctx.exported["quil.instructions"].update(
        Item(
            kind=Kind.Class,
            python_name=rust_name,
            rust_name=rust_name,
            path=ctx.path,
            line=line,
        )
        for name in line.text.replace(" ", "").removeprefix("impl_instruction!([").removesuffix("]);").split(",")
        if (rust_name := name.partition("[")[0].strip()) != ""
    )


def configure_logger(log_level: str | None = None):
    """Configure the logger with the given log level and a sensible format."""
    if log_level is not None:
        logger.setLevel(log_level)

    formatter = logging.Formatter(fmt="[{levelname:5} {name}:{filename}:{lineno}]: {message}", style="{")
    handler = logging.StreamHandler()
    handler.setFormatter(formatter)
    logger.addHandler(handler)


if __name__ == "__main__":
    main()
