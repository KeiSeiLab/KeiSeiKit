//! Markdown rendering — frontmatter + sections + backlinks.

use crate::extractor::{Section, SectionKind};

/// Build the full markdown document for one source file.
pub fn render_markdown(
    rel: &str,
    dna: &str,
    lang: &str,
    loc: usize,
    sections: &[Section],
    deps: &[String],
) -> String {
    let mut s = String::new();
    push_frontmatter(&mut s, rel, dna, lang, loc);
    s.push_str(&format!("# {}\n\n", rel));
    push_modules(&mut s, sections);
    push_items(&mut s, sections);
    push_related(&mut s, rel, deps);
    push_giscus(&mut s);
    s
}

/// Append Giscus comments widget — GitHub Discussions backend.
/// User must populate the four placeholders below from giscus.app
/// (Repo Settings → Discussions enable → giscus.app wizard → copy IDs).
fn push_giscus(s: &mut String) {
    s.push_str("\n## Discussion\n\n");
    s.push_str("<script src=\"https://giscus.app/client.js\"\n");
    s.push_str("        data-repo=\"KeiSei84/KeiSeiKit-1.0\"\n");
    s.push_str("        data-repo-id=\"PLACEHOLDER_REPO_ID\"\n");
    s.push_str("        data-category=\"wiki-comments\"\n");
    s.push_str("        data-category-id=\"PLACEHOLDER_CATEGORY_ID\"\n");
    s.push_str("        data-mapping=\"pathname\"\n");
    s.push_str("        data-strict=\"0\"\n");
    s.push_str("        data-reactions-enabled=\"1\"\n");
    s.push_str("        data-emit-metadata=\"0\"\n");
    s.push_str("        data-input-position=\"bottom\"\n");
    s.push_str("        data-theme=\"preferred_color_scheme\"\n");
    s.push_str("        data-lang=\"en\"\n");
    s.push_str("        data-loading=\"lazy\"\n");
    s.push_str("        crossorigin=\"anonymous\"\n");
    s.push_str("        async></script>\n");
}

fn push_frontmatter(s: &mut String, rel: &str, dna: &str, lang: &str, loc: usize) {
    // Title for Astro Starlight schema — derive from path tail.
    let title = rel
        .rsplit('/')
        .next()
        .unwrap_or(rel)
        .trim_end_matches(".md")
        .replace("__", "/");
    s.push_str("---\n");
    s.push_str(&format!("title: {}\n", title));
    s.push_str(&format!("path: {}\n", rel));
    s.push_str(&format!("dna_hash: {}\n", dna));
    s.push_str(&format!("language: {}\n", lang));
    s.push_str(&format!("size_loc: {}\n", loc));
    s.push_str("generated: by-keidocs\n");
    s.push_str("---\n\n");
}

fn push_modules(s: &mut String, sections: &[Section]) {
    for m in sections.iter().filter(|x| x.kind == SectionKind::Module) {
        s.push_str(&m.body);
        s.push_str("\n\n");
    }
}

fn push_items(s: &mut String, sections: &[Section]) {
    let items: Vec<&Section> = sections.iter().filter(|x| x.kind == SectionKind::Item).collect();
    if items.is_empty() {
        return;
    }
    s.push_str("## Public API\n\n");
    for it in items {
        let head = oneline(&it.body);
        match &it.target {
            Some(t) => s.push_str(&format!("- `{}` — {}\n", t, head)),
            None => s.push_str(&format!("- {}\n", head)),
        }
    }
    s.push('\n');
}

fn push_related(s: &mut String, rel: &str, deps: &[String]) {
    s.push_str("## Related\n\n");
    s.push_str(&format!("- parent: `{}`\n", parent_hint(rel)));
    if !deps.is_empty() {
        s.push_str(&format!("- imports: {}\n", deps.join(", ")));
    }
}

fn oneline(body: &str) -> String {
    body.lines().next().unwrap_or("").trim().to_string()
}

fn parent_hint(rel: &str) -> String {
    if let Some(idx) = rel.find("/src/") {
        return format!("{}/Cargo.toml", &rel[..idx]);
    }
    let parts: Vec<&str> = rel.rsplitn(2, '/').collect();
    if parts.len() == 2 {
        parts[1].to_string()
    } else {
        rel.to_string()
    }
}
