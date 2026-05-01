# TEST-FIRST

- Critical paths: tests BEFORE code (TDD — RED → GREEN → REFACTOR)
- Everything else: tests WITH code in the same change
- NEVER "I'll write tests later"

**Goal-Driven variant:** convert any task to a verify-criterion BEFORE starting.
- "Add validation" → "Write tests for invalid inputs, then make them pass"
- "Fix the bug" → "Write a test that reproduces it, then make it pass"
- "Refactor X" → "Ensure tests pass before and after"

Strong success criteria let you loop independently. Weak criteria ("make it work") require constant clarification.
