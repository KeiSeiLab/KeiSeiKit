//! Table-driven marker scorer.
//!
//! `MARKERS` is the single source of truth for project signals. To
//! teach the classifier a new build system, add a row here — no other
//! file changes.

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MarkerKind {
    BuildManifest,
    SourceDir,
    Doc,
    AlreadyRepo,
}

#[derive(Debug, Clone, Copy)]
pub struct Marker {
    pub file: &'static str,
    pub weight: i32,
    pub kind: MarkerKind,
    pub primary_lang: Option<&'static str>,
}

pub const MARKERS: &[Marker] = &[
    // 8 build manifests — weight 10, primary-lang hint set.
    Marker { file: "Cargo.toml",      weight: 10, kind: MarkerKind::BuildManifest, primary_lang: Some("rust")    },
    Marker { file: "package.json",    weight: 10, kind: MarkerKind::BuildManifest, primary_lang: Some("node")    },
    Marker { file: "pyproject.toml",  weight: 10, kind: MarkerKind::BuildManifest, primary_lang: Some("python")  },
    Marker { file: "go.mod",          weight: 10, kind: MarkerKind::BuildManifest, primary_lang: Some("go")      },
    Marker { file: "pom.xml",         weight: 10, kind: MarkerKind::BuildManifest, primary_lang: Some("java")    },
    Marker { file: "build.gradle",    weight: 10, kind: MarkerKind::BuildManifest, primary_lang: Some("gradle")  },
    Marker { file: "Gemfile",         weight: 10, kind: MarkerKind::BuildManifest, primary_lang: Some("ruby")    },
    Marker { file: "composer.json",   weight: 10, kind: MarkerKind::BuildManifest, primary_lang: Some("php")     },
    // Secondary signals.
    Marker { file: "README.md",       weight:  3, kind: MarkerKind::Doc,           primary_lang: None            },
    Marker { file: "src",             weight:  5, kind: MarkerKind::SourceDir,     primary_lang: None            },
    // Negative-marker (handled specially in classify): presence forces ALREADY-REPO.
    Marker { file: ".git",            weight: 15, kind: MarkerKind::AlreadyRepo,   primary_lang: None            },
];

pub fn marker_for(name: &str) -> Option<&'static Marker> {
    MARKERS.iter().find(|m| m.file == name)
}
