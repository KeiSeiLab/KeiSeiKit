//! 20-sample invalid-corpus test. Each fixture documents WHICH
//! `IssueKind` it is meant to elicit; the test asserts that exactly that
//! kind appears in the issue list (additional issues OK — multiple
//! findings stack, see `validator::validate`).

use kei_skills::validator::{validate, IssueKind};
use std::path::Path;

fn fixtures() -> Vec<(IssueKind, &'static str)> {
    vec![
        (IssueKind::MissingOpenFence, "no fence at all"),
        (IssueKind::UnclosedFrontmatter, "---\nname: x\ndescription: y\n"),
        (IssueKind::YamlParse, "---\nname: x\n  bad: : :\n---\nBody.\n"),
        (IssueKind::MissingName, "---\ndescription: only desc\n---\nBody.\n"),
        (IssueKind::MissingDescription, "---\nname: solo\n---\nBody.\n"),
        (
            IssueKind::NameInvalid,
            "---\nname: BadCaps\ndescription: ok\n---\nBody.\n",
        ),
        (
            IssueKind::NameInvalid,
            "---\nname: \"-leading-dash\"\ndescription: ok\n---\nBody.\n",
        ),
        (
            IssueKind::NameInvalid,
            "---\nname: \"slash/in/name\"\ndescription: ok\n---\nBody.\n",
        ),
        (
            IssueKind::NameTooLong,
            &*Box::leak(format!("---\nname: {}\ndescription: ok\n---\nBody.\n", "a".repeat(70)).into_boxed_str()),
        ),
        (
            IssueKind::DescriptionTooLong,
            &*Box::leak(format!("---\nname: ok\ndescription: \"{}\"\n---\nBody.\n", "x".repeat(1100)).into_boxed_str()),
        ),
        (
            IssueKind::BodyEmpty,
            "---\nname: ok\ndescription: ok\n---\n",
        ),
        (
            IssueKind::BodyEmpty,
            "---\nname: ok\ndescription: ok\n---\n   \n\n\t\n",
        ),
        (
            IssueKind::ContentTooLarge,
            &*Box::leak(format!("---\nname: ok\ndescription: ok\n---\n{}", "a".repeat(100_005)).into_boxed_str()),
        ),
        (IssueKind::MissingOpenFence, ""),
        (
            IssueKind::NameInvalid,
            "---\nname: \" leadingspace\"\ndescription: ok\n---\nBody.\n",
        ),
        (
            IssueKind::YamlParse,
            "---\nname: [list, not, scalar]\ndescription: ok\n---\nBody.\n",
        ),
        (
            IssueKind::MissingName,
            "---\nname: \"\"\ndescription: ok\n---\nBody.\n",
        ),
        (
            IssueKind::MissingDescription,
            "---\nname: ok\ndescription: \"\"\n---\nBody.\n",
        ),
        (
            IssueKind::NameInvalid,
            "---\nname: \"with space\"\ndescription: ok\n---\nBody.\n",
        ),
        (
            IssueKind::UnclosedFrontmatter,
            "---\njust an opening fence and no end",
        ),
    ]
}

#[test]
fn corpus_each_sample_fails_with_expected_kind() {
    let path = Path::new("<corpus>");
    for (i, (expected, src)) in fixtures().into_iter().enumerate() {
        let res = validate(src, path);
        match res {
            Ok(_) => panic!("fixture {i} (expected {expected:?}) unexpectedly validated"),
            Err(issues) => {
                let kinds: Vec<_> = issues.iter().map(|i| i.kind).collect();
                assert!(
                    kinds.contains(&expected),
                    "fixture {i} (expected {expected:?}) produced {kinds:?}"
                );
            }
        }
    }
}

#[test]
fn valid_minimal_passes() {
    let src = "---\nname: ok\ndescription: ok\n---\nbody\n";
    validate(src, Path::new("<inline>")).expect("must validate");
}
