# Argument

## Scope

Owns the selected inherited `Argument<T>` surface for `ArgumentInteger`.

## Implemented State

Implemented by the accepted Spinel implementation commit under `spinel-server/src/command/builder/arguments/argument.rs`, `mapped_argument.rs`, and `default_value.rs`.

| Minestom API | Spinel implementation | Status and evidence |
| --- | --- | --- |
| constructors and input policy | `Argument::custom`, `set_allows_space`, `set_uses_remaining_input` | complete; custom begins false/false and setters remain independent. |
| static `Argument.parse` | `Argument::parse(sender, &argument)` | complete; source-shaped static call retained. |
| receiver `parse` | `parse_input(&self, sender, input)` | complete under D-039; Rust cannot overload the static `parse` name. |
| parser/properties/class | `ArgumentBehavior<T>` | complete through `get_parser`, `get_node_properties`, and `get_concrete_class`. |
| callback/default/suggestion state | typed owner getters/setters | complete; default resolution is `Result<Option<T>, ArgumentError>`. |
| three default setters | value, supplier, sender-provider setters | complete. |
| `map` overloads | `map` and `map_for_sender` | complete; mapped parsing and mapped defaults propagate mapper `Err` code 555. |
| `filter` | `filter` | complete; parsed values failing the predicate return 556; source defaults are copied unchanged, matching Minestom `ArgumentFilter`. |
| exact-class/id equality | `PartialEq`, `Eq`, `Hash` | complete. |

## Verification

- [x] Typed/static parsing, metadata, default, map/filter, and equality foundation.
- [x] Map code 555 and filter code 556.
- [x] Filtered `-1` default remains `Ok(Some(-1))`; parsed `-1` is code 556.
- [x] Focused server command suite: 37 passed.

## Remaining Scope

Future `ArgumentGroup` and `ArgumentLoop` consumers may use the completed `AnyArgument` and typed-handle seams; neither argument family is implemented by this slice.


