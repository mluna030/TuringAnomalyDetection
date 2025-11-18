use crate::domain::rule::Action;

pub trait FirewallBackend: Send + Sync {
    fn apply_rule(&self, rule_id: u64, action: &Action) -> Result<(), String>;
    fn remove_rule(&self, rule_id: u64) -> Result<(), String>;
    fn flush_rules(&self) -> Result<(), String>;
}

pub struct IpTablesBackend {

}
pub struct NfTablesBackend {
    
}