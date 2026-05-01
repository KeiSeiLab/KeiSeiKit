//! kei-task integration tests.

use kei_task::deps::{add_dependency, dependency_chain};
use kei_task::graph::list_edges;
use kei_task::milestones::{create_milestone, link_task_to_milestone, tasks_in_milestone};
use kei_task::search::search;
use kei_task::{Milestone, Store, Task};

fn mk() -> Store { Store::open_memory().unwrap() }

fn mktask(title: &str) -> Task {
    Task { title: title.into(), priority: "high".into(), ..Default::default() }
}

#[test]
fn create_and_get() {
    let s = mk();
    let id = s.create_task(&mktask("a")).unwrap();
    let t = s.get_task(id).unwrap().unwrap();
    assert_eq!(t.title, "a");
    assert_eq!(t.status, "pending");
}

#[test]
fn update_persists() {
    let s = mk();
    let id = s.create_task(&mktask("a")).unwrap();
    let mut t = s.get_task(id).unwrap().unwrap();
    t.status = "in_progress".into();
    s.update_task(&t).unwrap();
    let u = s.get_task(id).unwrap().unwrap();
    assert_eq!(u.status, "in_progress");
}

#[test]
fn cycle_detected() {
    let s = mk();
    let a = s.create_task(&mktask("a")).unwrap();
    let b = s.create_task(&mktask("b")).unwrap();
    let c = s.create_task(&mktask("c")).unwrap();
    add_dependency(&s, a, b, "blocks").unwrap();
    add_dependency(&s, b, c, "blocks").unwrap();
    // a -> b -> c; now c -> a would be a cycle
    let err = add_dependency(&s, c, a, "blocks");
    assert!(err.is_err(), "cycle detection must reject");
}

#[test]
fn milestone_linking() {
    let s = mk();
    let t = s.create_task(&mktask("design")).unwrap();
    let ms_id = create_milestone(&s, &Milestone {
        name: "v1".into(), ..Default::default() }).unwrap();
    link_task_to_milestone(&s, t, ms_id).unwrap();
    let tasks = tasks_in_milestone(&s, ms_id).unwrap();
    assert_eq!(tasks, vec![t]);
}

#[test]
fn dependency_chain_traversal() {
    let s = mk();
    let a = s.create_task(&mktask("a")).unwrap();
    let b = s.create_task(&mktask("b")).unwrap();
    let c = s.create_task(&mktask("c")).unwrap();
    add_dependency(&s, a, b, "blocks").unwrap();
    add_dependency(&s, b, c, "blocks").unwrap();
    let chain = dependency_chain(&s, a).unwrap();
    assert!(chain.contains(&b));
    assert!(chain.contains(&c));
    assert_eq!(chain.len(), 2);
}

#[test]
fn task_graph_edges() {
    let s = mk();
    let a = s.create_task(&mktask("a")).unwrap();
    let b = s.create_task(&mktask("b")).unwrap();
    add_dependency(&s, a, b, "blocks").unwrap();
    let edges = list_edges(&s).unwrap();
    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0].task_id, a);
    assert_eq!(edges[0].depends_on, b);
}

#[test]
fn search_finds_task() {
    let s = mk();
    s.create_task(&Task {
        title: "refactor router".into(),
        description: "split monolith".into(),
        ..Default::default()
    }).unwrap();
    let hits = search(&s, "refactor", 10).unwrap();
    assert_eq!(hits.len(), 1);
}
