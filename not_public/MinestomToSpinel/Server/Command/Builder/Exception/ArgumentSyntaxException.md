# ArgumentSyntaxException

## Scope

Documents Minestom `ArgumentSyntaxException` source ownership and its approved Spinel representation.

## Implemented State

| Minestom API | Spinel implementation | Status |
| --- | --- | --- |
| constructor message/input/error code | `ArgumentError::new` | complete. |
| input and code getters | `get_input`, `get_error_code` | complete. |
| parse failure transport | `Result<T, ArgumentError>` | complete for typed arguments, map/default resolution, and numeric parsing. |

`ArgumentError` lives at `spinel-server/src/command/exception/argument_error.rs`, implements Rust `Error`, and retains source-required message, input, and error-code data.

## Verification

- [x] Numeric errors 1/2/3.
- [x] Mapper error 555.
- [x] Filter predicate error 556.

## Remaining Scope

Minestom stack-trace suppression has no Rust observable counterpart.
