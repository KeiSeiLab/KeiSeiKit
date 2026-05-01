use kei_content_store::assets::{get_asset, register_asset, Asset};
use kei_content_store::campaigns::{attach_asset, campaign_assets, create_campaign};
use kei_content_store::prompts::{register_prompt, Prompt};
use kei_content_store::Store;

fn mk() -> Store { Store::open_memory().unwrap() }

#[test]
fn asset_roundtrip() {
    let s = mk();
    let id = register_asset(&s, &Asset {
        title: "logo.png".into(), media_type: "image/png".into(),
        ..Default::default()
    }).unwrap();
    let a = get_asset(&s, id).unwrap().unwrap();
    assert_eq!(a.title, "logo.png");
}

#[test]
fn prompt_dedup_by_hash() {
    let s = mk();
    let a = register_prompt(&s, &Prompt {
        prompt_text: "describe a cat".into(), model: "dall-e-3".into(),
        ..Default::default()
    }).unwrap();
    let b = register_prompt(&s, &Prompt {
        prompt_text: "describe a cat".into(), model: "dall-e-3".into(),
        ..Default::default()
    }).unwrap();
    assert_eq!(a, b, "same text+model must collapse");
}

#[test]
fn campaign_creation() {
    let s = mk();
    let c = create_campaign(&s, "spring", "spring launch").unwrap();
    assert!(c > 0);
}

#[test]
fn campaign_asset_attach() {
    let s = mk();
    let c = create_campaign(&s, "launch", "").unwrap();
    let a = register_asset(&s, &Asset {
        title: "hero.mp4".into(), ..Default::default() }).unwrap();
    attach_asset(&s, c, a).unwrap();
    assert_eq!(campaign_assets(&s, c).unwrap(), vec![a]);
}
