//! Router cost-based selection test.
//!
//! Given (in=10K, out=2K) the cheapest registered provider should be picked.
//! Costs (cents per MTok in/out):
//!   anthropic = 100 / 500
//!   openai    = 15 / 60
//!   kimi      = 60 / 250
//! For (10_000 in, 2_000 out):
//!   anthropic = ceil((100*10_000 + 500*2_000) / 1_000_000) = ceil(2_000_000/1M) = 2c
//!   openai    = ceil((15*10_000 + 60*2_000) / 1_000_000)  = ceil(270_000/1M)   = 1c
//!   kimi      = ceil((60*10_000 + 250*2_000) / 1_000_000) = ceil(1_100_000/1M) = 2c
//! → openai wins.

use kei_router::{
    AnthropicProvider, KimiProvider, LlmRouter, OpenAiProvider, Provider,
};

#[test]
fn picks_openai_for_typical_chat_request() {
    let mut router = LlmRouter::new();
    router.register(Box::new(AnthropicProvider::with_endpoint(
        "a".into(),
        "claude-haiku-4-5".into(),
        "http://stub".into(),
    )));
    router.register(Box::new(OpenAiProvider::with_endpoint(
        "o".into(),
        "gpt-4o-mini".into(),
        "http://stub".into(),
    )));
    router.register(Box::new(KimiProvider::with_endpoint(
        "k".into(),
        "kimi-k2-thinking".into(),
        "http://stub".into(),
    )));

    let cheapest = router.cheapest_for_estimated_tokens(10_000, 2_000).unwrap();
    assert_eq!(cheapest.name(), "openai");
}

#[test]
fn picks_anthropic_when_only_one_registered() {
    let mut router = LlmRouter::new();
    router.register(Box::new(AnthropicProvider::with_endpoint(
        "a".into(),
        "claude-haiku-4-5".into(),
        "http://stub".into(),
    )));
    let p = router.cheapest_for_estimated_tokens(1_000_000, 200_000).unwrap();
    assert_eq!(p.name(), "anthropic");
}

#[test]
fn pick_by_name_returns_correct_provider() {
    let mut router = LlmRouter::new();
    router.register(Box::new(AnthropicProvider::with_endpoint(
        "a".into(),
        "claude-haiku-4-5".into(),
        "http://stub".into(),
    )));
    router.register(Box::new(KimiProvider::with_endpoint(
        "k".into(),
        "kimi-k2-thinking".into(),
        "http://stub".into(),
    )));

    assert_eq!(router.pick("anthropic").unwrap().name(), "anthropic");
    assert_eq!(router.pick("kimi").unwrap().name(), "kimi");
    assert!(router.pick("unknown").is_err());
}

#[test]
fn names_returns_sorted_list() {
    let mut router = LlmRouter::new();
    router.register(Box::new(KimiProvider::with_endpoint(
        "k".into(),
        "kimi-k2-thinking".into(),
        "http://stub".into(),
    )));
    router.register(Box::new(AnthropicProvider::with_endpoint(
        "a".into(),
        "claude-haiku-4-5".into(),
        "http://stub".into(),
    )));
    assert_eq!(router.names(), vec!["anthropic", "kimi"]);
}
