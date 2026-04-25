//! Integration tests for the preset persona templates.
//!
//! These tests guarantee that every bundled template:
//!   - parses as valid TOML against the current schema,
//!   - passes the full R1-R19 validator,
//!   - stays exposed in a stable, published order for `/pet-setup`.

use kei_pet::{load_template, list_templates, PetTemplate};

#[test]
fn load_friend_template_parses_valid() {
    let m = load_template(PetTemplate::Friend).expect("friend template must parse + validate");
    assert_eq!(m.schema, 1);
    assert_eq!(m.identity.pet_name, "Kei");
}

#[test]
fn all_five_templates_pass_validation() {
    let all = [
        PetTemplate::Friend,
        PetTemplate::Tutor,
        PetTemplate::Coach,
        PetTemplate::TherapistCompanion,
        PetTemplate::ProductivityPartner,
    ];
    for t in all {
        let r = load_template(t);
        assert!(r.is_ok(), "template {:?} failed to parse/validate: {:?}", t, r.err());
    }
}

#[test]
fn list_templates_returns_five_in_stable_order() {
    let list = list_templates();
    assert_eq!(list.len(), 5, "preset list must have exactly 5 entries");
    assert_eq!(list[0].0, PetTemplate::Friend);
    assert_eq!(list[1].0, PetTemplate::Tutor);
    assert_eq!(list[2].0, PetTemplate::Coach);
    assert_eq!(list[3].0, PetTemplate::TherapistCompanion);
    assert_eq!(list[4].0, PetTemplate::ProductivityPartner);
    for (_, desc) in &list {
        assert!(!desc.is_empty(), "every template needs a non-empty description");
    }
}
