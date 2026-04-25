use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub prune_threshold: f64,
    pub default_lambda: f64,
    pub decay_lambdas: HashMap<String, f64>,
}

impl Default for Config {
    fn default() -> Self {
        let mut l = HashMap::new();
        // research-backed defaults mirroring LBM internal/curator/types.go
        l.insert("threat".into(), 0.1);
        l.insert("code".into(), 0.01);
        l.insert("protocol".into(), 0.03);
        l.insert("finance".into(), 0.08);
        l.insert("osint".into(), 0.1);
        l.insert("infra".into(), 0.02);
        l.insert("sage".into(), 0.005);
        Self {
            prune_threshold: 0.1,
            default_lambda: 0.05,
            decay_lambdas: l,
        }
    }
}

impl Config {
    pub fn lambda_for(&self, domain: &str) -> f64 {
        self.decay_lambdas.get(domain).copied().unwrap_or(self.default_lambda)
    }
}
