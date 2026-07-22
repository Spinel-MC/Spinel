# ArgumentNumber

## Scope

Owns the selected full numeric surface inherited by `ArgumentInteger`.

## Implemented State

Implemented by the accepted Spinel implementation commit in `spinel-server/src/command/builder/arguments/number/argument_number.rs`.

| Minestom behavior | Spinel implementation | Status |
| --- | --- | --- |
| constants 1/2/3 | `NOT_NUMBER_ERROR`, `TOO_LOW_ERROR`, `TOO_HIGH_ERROR` | complete. |
| decimal, binary, hexadecimal, scientific parsing | numeric `ArgumentBehavior<T>::parse_input` | complete for `i32`. |
| malformed/overflow, lower, upper errors | `ArgumentError` codes 1, 2, 3 | complete. |
| inclusive `min`, `max`, `between` | mutable `ArgumentNumber<T>` methods | complete. |
| properties | flags plus min then max encoded bytes | complete for all four bound states. |
| numeric getters | `get_number_properties`, `has_min`, `get_min`, `has_max`, `get_max` | complete. |

## Verification

- [x] Decimal, binary, hexadecimal, scientific parsing.
- [x] Boundary/error codes 1, 2, and 3.
- [x] No-bound, min-only, max-only, and both-bound property bytes.
- [x] Integer declaration and core packet command tests.

## Remaining Scope

Only the signed-32-bit `ArgumentInteger` specialization is implemented. Other Minestom numeric concrete descendants remain separate future work over this completed shared owner.




