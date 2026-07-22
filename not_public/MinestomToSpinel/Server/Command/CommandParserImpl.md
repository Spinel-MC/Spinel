# CommandParserImpl

## Scope

Owns command-graph integration for typed argument parsing.

## Implemented State

`spinel-server/src/command/parser.rs` recognizes the typed argument adapter, delegates conversion to the argument owner through `AnyArgument`, and stores the resulting integer in `CommandContext` through the erased bridge.

| Minestom behavior | Spinel implementation | Status |
| --- | --- | --- |
| argument parse during syntax selection | typed adapter invokes argument-owned parse | complete. |
| parse failures reject syntax before execution | parser returns invalid candidate | complete. |
| typed context value | `CommandContext::integer` | complete for `ArgumentInteger`. |

## Verification

- [x] Minimum-zero `-1` rejects syntax.
- [x] Valid integer reaches typed context.
- [x] Declaration properties carry the configured lower bound.
- [x] Focused server command suite: 37 passed.

## Remaining Scope

Future heterogeneous `ArgumentGroup` and homogeneous `ArgumentLoop` parsing remain consumers of the completed `AnyArgument`/`Argument<T>` capability, not part of this slice.
