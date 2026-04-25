use kei_social_store::graph::relationship_graph;
use kei_social_store::interactions::{interactions_for, log_interaction, Interaction};
use kei_social_store::people::{add_org, add_person, get_person, Organization, Person};
use kei_social_store::search::search_people;
use kei_social_store::Store;

fn mk() -> Store { Store::open_memory().unwrap() }

#[test]
fn people_crud() {
    let s = mk();
    let id = add_person(&s, &Person {
        name: "Alice".into(), email: "alice@example.com".into(),
        ..Default::default()
    }).unwrap();
    let p = get_person(&s, id).unwrap().unwrap();
    assert_eq!(p.name, "Alice");
}

#[test]
fn orgs_idempotent() {
    let s = mk();
    let a = add_org(&s, &Organization { name: "Acme".into(), ..Default::default() }).unwrap();
    let b = add_org(&s, &Organization { name: "Acme".into(), ..Default::default() }).unwrap();
    assert_eq!(a, b);
}

#[test]
fn interactions_tracked() {
    let s = mk();
    let p = add_person(&s, &Person { name: "Bob".into(), ..Default::default() }).unwrap();
    log_interaction(&s, &Interaction {
        person_id: p, interaction_type: "email".into(),
        content: "hi".into(), channel: "gmail".into(),
        ..Default::default()
    }).unwrap();
    let hist = interactions_for(&s, p).unwrap();
    assert_eq!(hist.len(), 1);
    assert_eq!(hist[0].interaction_type, "email");
}

#[test]
fn search_finds_person() {
    let s = mk();
    add_person(&s, &Person {
        name: "Carol Chang".into(), bio: "rust async".into(),
        ..Default::default()
    }).unwrap();
    let hits = search_people(&s, "rust", 10).unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].name, "Carol Chang");
}

#[test]
fn relationship_graph_groups() {
    let s = mk();
    let a = add_person(&s, &Person { name: "A".into(), ..Default::default() }).unwrap();
    let b = add_person(&s, &Person { name: "B".into(), ..Default::default() }).unwrap();
    for _ in 0..3 {
        log_interaction(&s, &Interaction {
            person_id: a, target_id: b, interaction_type: "msg".into(),
            channel: "slack".into(), ..Default::default()
        }).unwrap();
    }
    let pairs = relationship_graph(&s).unwrap();
    assert_eq!(pairs.len(), 1);
    assert_eq!(pairs[0].count, 3);
}
