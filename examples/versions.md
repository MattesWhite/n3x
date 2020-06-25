# Examples

This folder contains various examples from the different stages of development of N3X.
Each example starts with a comment about the version of N3/N3X used.

| Version-tag | Note |
| ----------- | ---- |
| `N3`        | Standard N3 |
| `N3X-0.0.1` | Expressions as special literals |
| `N3X-0.1.0` | No delimiters around expressions |
| `N3X-0.1.1` | With `$` as delimiter for expressions |
| `N3X-0.2.0` | Limit magic predicates to `:=` and `+=` [1] |

[1] `:=` is a shorthand for `http://ti.rw.fau.de/n3x/0/2/0/ns#bind` and `+=` is a shorthand for `http://ti.rw.fau.de/n3x/0/2/0/ns#addCondition`.