# MEMORY PROTOCOL

**At start:**
1. Read `~/.claude/memory/MEMORY.md` (or your index file) → find relevant project file
2. Read `memory/{project}.md` → constraints, stack, status, learnings
3. If ML / research work: also check your `wrong-paths.md` notes (dead ends worth avoiding)

**At end (if stage completed — feature/phase/milestone/audit/bug+fix/deploy/decision/blocker):**
1. Append to `memory/{project}.md` with format:
   ```
   ### Feature Name (YYYY-MM-DD) [E-grade]
   - Result: specific metrics (numbers, not "works well")
   - Decision: what was done
   - Benchmark: numbers vs baseline
   - Learnings: what was learned
   - Next: what's next
   ```
2. If dead end / wrong path → append to your `wrong-paths.md`
3. If architectural decision → project's `DECISIONS.md`
4. Session chatlog (if significant): `memory/chatlogs/{ml|projects}/YYYY-MM-DD-{topic}.md`

**Forbidden:** transitioning without saving; writing "works" without metrics; leaving credentials only in conversation context.
