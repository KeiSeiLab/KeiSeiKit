# Wave Test — Malformed Action Table

## Actionable plan

This master has the heading but the next table only has one column.

| Action |
|---|
| something |
| else |

So the parser should still pick up these one-cell rows since "Action" is present, but with no severity / effort / id columns. Severity and effort default to empty.
