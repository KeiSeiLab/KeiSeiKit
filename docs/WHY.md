# Why I built KeiSeiKit

*By Denis Parfionovich. Restored from the original README preamble (pre-v0.22 product pivot), available separately for those who want the full philosophical background. The main README keeps the product-oriented pitch.*

---

Hello.

Transformers are statistically wired to lie. It is not a bug, it is the core operation: they pick the next token by sampling a probability distribution conditioned on whatever context happens to fit the window. They cannot reliably hold long context, they will drag a hallucination picked up three thousand tokens ago into a confident final sentence, and they may deliver a brilliant insight right next to a fabricated citation. This is not fixable inside the model — it is mathematically baked in from the moment of tokenization onward.

This kit is my humble attempt to build scaffolding *around* those errors. Not to fix the transformer, but to make it behave a little closer to my own working logic: catch the common failure modes before they reach a commit, give it external memory that survives session boundaries, give it a rhythm resembling human work (day sessions → overnight consolidation → morning report), and let parallel agents coordinate through a shared state instead of stepping on each other.

I work across 4 to 8 parallel Claude terminals most days. The problems I hit are mundane: forgetting between sessions, repeating the same mistake on the third try, parallel agents clobbering each other's files, a hallucinated API name shipped to production. None of these are solvable by a better prompt. They are solvable by structure around the prompt.

Most of what is here is well-established bricks (git-as-state, cron, TF-IDF, constructor-pattern composition). What may be new in the Claude Code context is the Constructor Pattern for agents (composable blocks, deterministic build, rebuild-on-block-edit hooks) and sleep-sync (using a git repo as the transport layer between sessions, with an Anthropic-cloud agent doing nightly REM-style consolidation).

## Why Rust, not Python

Transformers write Python confidently and wrongly. Five minutes to a plausible-looking function, two hours to debug why `dict.get("key", None) or []` silently swallowed an empty list, or why an async context manager leaked a file handle across a retry loop. Rust's type system catches whole categories of those errors at compile-time — the model literally cannot ship a `None`-vs-`[]` confusion, a missing `.await`, or an unhandled `Result`, because the compiler refuses. For an LLM-written codebase this is not aesthetic preference, it is survival. Every Rust primitive here is one category of hallucination the transformer is no longer allowed to commit.

## It is not a product

It helps me personally. If it resonates with you, let me know. If enough feedback comes in, there will be a next version — more primitives, more patterns against the "forgetful" transformer. But that needs input; without it, I just keep using this quietly myself.

Forks and PRs welcome from everyone, not only from those who write code. If you hit a problem with Claude Code and have an idea for solving it, open an issue with the description. A well-formulated problem is already half the solution.

Hope it is a small Kei for someone to make vibecoding better.

And double sorry if I'm repeating someone — I never tried other kits, since this one is just all my rules stacked in one place. I can't always tell what I have seen somewhere and what came from my own head — so treat this as just my sample, not a claim of originality.

Thanks.
