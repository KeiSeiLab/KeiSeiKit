//! LLM provider router — multi-provider abstraction.
//!
//! Wave 32 v0.40: holds a registry `name → Box<dyn Provider>` and selects
//! either by explicit name or by cost given a token estimate.
//!
//! Stateless per request: the router holds provider configs (api keys + model
//! + endpoint), but no conversation state.

use std::collections::HashMap;

use crate::provider::{Error, Provider};
use crate::providers::{AnthropicProvider, KimiProvider, OpenAiProvider};

pub struct LlmRouter {
    providers: HashMap<&'static str, Box<dyn Provider>>,
}

impl Default for LlmRouter {
    fn default() -> Self { Self::new() }
}

impl LlmRouter {
    /// Empty router — register providers manually.
    pub fn new() -> Self { Self { providers: HashMap::new() } }

    /// Register all providers whose API key is present in env.
    /// Order is informational: Anthropic, OpenAI, Kimi.
    pub fn from_env() -> Self {
        let mut r = Self::new();
        if let Some(p) = AnthropicProvider::from_env() { r.register(Box::new(p)); }
        if let Some(p) = OpenAiProvider::from_env() { r.register(Box::new(p)); }
        if let Some(p) = KimiProvider::from_env() { r.register(Box::new(p)); }
        r
    }

    /// Register one provider; returns the name registered.
    pub fn register(&mut self, p: Box<dyn Provider>) -> &'static str {
        let name = p.name();
        self.providers.insert(name, p);
        name
    }

    /// Lookup by stable name. Errors with `UnknownProvider` if unregistered.
    pub fn pick(&self, name: &str) -> Result<&dyn Provider, Error> {
        self.providers
            .get(name)
            .map(|b| b.as_ref())
            .ok_or_else(|| Error::UnknownProvider(name.to_string()))
    }

    /// Names of registered providers (sorted for stable iteration).
    pub fn names(&self) -> Vec<&'static str> {
        let mut v: Vec<&'static str> = self.providers.keys().copied().collect();
        v.sort_unstable();
        v
    }

    /// Cheapest provider for an estimated workload. Cost is computed per-MTok
    /// from `cost_per_m_tok_input_cents * in_tok / 1_000_000 + (output side)`.
    /// Errors if no providers are registered.
    pub fn cheapest_for_estimated_tokens(
        &self,
        in_tok: u64,
        out_tok: u64,
    ) -> Result<&dyn Provider, Error> {
        self.providers
            .values()
            .min_by_key(|p| estimate_cents(p.as_ref(), in_tok, out_tok))
            .map(|b| b.as_ref())
            .ok_or_else(|| Error::UnknownProvider("(no providers registered)".into()))
    }
}

/// Estimate request cost in cents (rounded up to whole cents — we never
/// undercount).
fn estimate_cents(p: &dyn Provider, in_tok: u64, out_tok: u64) -> u64 {
    let in_cents = (p.cost_per_m_tok_input_cents() as u64) * in_tok;
    let out_cents = (p.cost_per_m_tok_output_cents() as u64) * out_tok;
    let total = in_cents + out_cents;
    // ceil-div by 1M
    (total + 999_999) / 1_000_000
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use futures::stream::BoxStream;
    use crate::provider::{Message, Provider, StreamEvent, Tool};

    struct Fake { name: &'static str, in_c: u32, out_c: u32 }

    #[async_trait]
    impl Provider for Fake {
        fn name(&self) -> &'static str { self.name }
        fn cost_per_m_tok_input_cents(&self) -> u32 { self.in_c }
        fn cost_per_m_tok_output_cents(&self) -> u32 { self.out_c }
        async fn stream_message(
            &self,
            _: &str,
            _: &[Message],
            _: Option<&[Tool]>,
        ) -> Result<BoxStream<'static, Result<StreamEvent, Error>>, Error> {
            unimplemented!()
        }
    }

    #[test]
    fn estimate_cents_simple() {
        let p = Fake { name: "x", in_c: 100, out_c: 500 };
        // 1M in @ 100c + 1M out @ 500c = 600c
        assert_eq!(estimate_cents(&p, 1_000_000, 1_000_000), 600);
    }

    #[test]
    fn cheapest_picks_lowest_total() {
        let mut r = LlmRouter::new();
        r.register(Box::new(Fake { name: "expensive", in_c: 300, out_c: 1500 }));
        r.register(Box::new(Fake { name: "cheap", in_c: 15, out_c: 60 }));
        let p = r.cheapest_for_estimated_tokens(10_000, 2_000).unwrap();
        assert_eq!(p.name(), "cheap");
    }

    #[test]
    fn pick_by_name_works() {
        let mut r = LlmRouter::new();
        r.register(Box::new(Fake { name: "x", in_c: 1, out_c: 1 }));
        assert_eq!(r.pick("x").unwrap().name(), "x");
        assert!(matches!(r.pick("y"), Err(Error::UnknownProvider(_))));
    }
}
