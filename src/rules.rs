// Rule system — CleanerML-inspired app-specific cleanup rules.
// Parses .rule files from the rules/ directory and matches paths against them.
// Stdlib-only: no serde, no toml, just line parsing.

use std::path::Path;

/// A single cleanup rule.
#[derive(Clone, Debug)]
#[allow(dead_code)] // name/description available for future reporting; kind and protected are used
pub struct Rule {
    pub name: String,
    pub paths: Vec<String>,
    pub patterns: Vec<String>,
    pub kind: String,
    pub description: String,
    pub protected: bool,
}

impl Rule {
    /// Returns true if `path` matches this rule (by path prefix or glob pattern).
    pub fn matches(&self, path: &Path) -> bool {
        let spill = path.to_string_lossy();
        // Path prefix match
        for p in &self.paths {
            if spill.starts_with(p.as_str()) {
                return true;
            }
        }
        // Glob pattern match on file name
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            for pat in &self.patterns {
                if glob_match(name, pat) {
                    return true;
                }
            }
        }
        false
    }
}

/// Simple glob match: supports `*` (matches any sequence) and `?` (matches one char).
/// Depth-capped to prevent stack overflow on pathological patterns.
fn glob_match(text: &str, pattern: &str) -> bool {
    glob_match_inner(text.as_bytes(), pattern.as_bytes(), 0)
}

const GLOB_MAX_DEPTH: u32 = 64;

fn glob_match_inner(text: &[u8], pattern: &[u8], depth: u32) -> bool {
    if depth > GLOB_MAX_DEPTH {
        return false;
    }
    match (text.first(), pattern.first()) {
        (None, None) => true,
        (None, Some(b'*')) => glob_match_inner(text, &pattern[1..], depth + 1),
        (None, Some(_)) => false,
        (Some(_), None) => false,
        (Some(t), Some(p)) => {
            if *p == b'*' {
                // * matches zero or more chars
                (0..=text.len()).any(|i| glob_match_inner(&text[i..], &pattern[1..], depth + 1))
            } else if *p == b'?' || *p == *t {
                glob_match_inner(&text[1..], &pattern[1..], depth + 1)
            } else {
                false
            }
        }
    }
}

/// Load all rules from `rules/` directory relative to `base`.
pub fn load_rules(base: &Path) -> Vec<Rule> {
    let rules_dir = base.join("rules");
    let mut rules: Vec<Rule> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&rules_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("rule") {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    if let Some(rule) = parse_rule(&content) {
                        rules.push(rule);
                    }
                }
            }
        }
    }
    rules
}

/// Parse a single rule file's content into a Rule.
fn parse_rule(content: &str) -> Option<Rule> {
    let mut name = String::new();
    let mut paths: Vec<String> = Vec::new();
    let mut patterns: Vec<String> = Vec::new();
    let mut description = String::new();
    let mut protected = false;
    let mut kind = String::new();
    let mut in_section = false;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            name = line[1..line.len() - 1].trim().to_string();
            in_section = true;
            continue;
        }
        if !in_section {
            continue;
        }
        if let Some((key, val)) = split_once(line, '=') {
            match key.trim() {
                "paths" => {
                    paths = val.split(',').map(|s| s.trim().to_string()).collect();
                }
                "patterns" => {
                    patterns = val.split(',').map(|s| s.trim().to_string()).collect();
                }
                "description" => {
                    description = val.trim().to_string();
                }
                "protected" => {
                    protected = val.trim().eq_ignore_ascii_case("true");
                }
                "kind" => {
                    kind = val.trim().to_string();
                }
                _ => {}
            }
        }
    }

    if name.is_empty() {
        return None;
    }

    Some(Rule {
        name,
        paths,
        patterns,
        kind,
        description,
        protected,
    })
}

fn split_once<'a>(s: &'a str, delim: char) -> Option<(&'a str, &'a str)> {
    s.find(delim).map(|i| (&s[..i], &s[i + 1..]))
}

/// Find all rules that match `path`.
pub fn matching_rules<'a>(rules: &'a [Rule], path: &Path) -> Vec<&'a Rule> {
    rules.iter().filter(|r| r.matches(path)).collect()
}

/// Returns true if any matching rule marks the path as protected.
pub fn is_rule_protected(rules: &[Rule], path: &Path) -> bool {
    matching_rules(rules, path).iter().any(|r| r.protected)
}
