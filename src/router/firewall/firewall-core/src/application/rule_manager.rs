use crate::domain::rule::{Filter, RuleEntry, Action};
use std::sync::{Arc, RwLock};

pub struct RuleManager {
    rules: Arc<RwLock<Vec<RuleEntry>>>,
    next_id: Arc<RwLock<u64>>,
}

impl RuleManager {
    pub fn new() -> Self {
        Self {
            rules: Arc::new(RwLock::new(Vec::new())),
            next_id: Arc::new(RwLock::new(0)),
        }
    }

    pub fn add_rule(&self, filter: Box<dyn Filter>) -> u64 {
        let id = {
            let mut next_id = self.next_id.write().unwrap;
            let id = *next_id;
            *next_id += 1;
            id
        };

        let entry = RuleEntry {
            id,
            filter,
            enabled: true,
            hit_count: 0,
        };
        let mut rules = self.rules.write().unwrap();
        rules.push(entry);

        rules.sort_by(|a, b| {
            b.filter.priority().cmp(&a.filter.priority())
        });
        id
    }

    pub fn remove_rule(&self, id: u64) -> bool {
        let mut rules = self.rules.write().unwrap();
        let len_before = rules.len();

        rules.retain(|entry| entry.id != id);
        rules.len() < len_before
    }

    pub fn list_rules(&self) -> Vec<RuleInfo> {
        let rules = self.rules.read().unwrap();

        rules.iter()
            .map(|entry| RuleInfo {
                id: entry.id,
                name: entry.filter.name().to_string(),
                priority: entry.filter.priority(),
                enabled: entry.enabled,
                hit_count: entry.hit_count,
            })
            .collect()
    }

    pub fn clear_all(&self) {
        let mut rules = self.rules.write().unwrap();
        rules.clear();
    }
    pub(crate) fn rules_ref(&self) -> Arc<RwLock<Vec<RuleEntry>>> {
        Arc::clone(&self.rules)
    }
}

#[derive(Debug, Clone)]
pub struct RuleInfo {
    pub id: u64,
    pub name: String,
    pub priority: i32,
    pub enabled: bool,
    pub hit_count: u64,
}