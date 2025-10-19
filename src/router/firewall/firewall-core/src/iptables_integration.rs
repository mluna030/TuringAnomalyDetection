use iptables::IPTables;

pub struct Firewall {
    ipt: IPTables,
}
impl Firewall {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let ipt = iptables::new(false)?;
        Ok(Firewall {ipt})
    }
    pub fn block_ip(&self, ip: &str) -> Result<(), Box<dyn std::error::Error>> {
        let rule = format!("-s {} -j DROP", ip);
        self.ipt.append("filter", "INPUT", &rule)?;
        println!("Blocked IP: {}", ip);
        Ok(())
    }
    pub fn allow_port(&self, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        let rule = format!("-p tcp --dport {} -j ACCEPT", port);
        self.ipt.append("filter", "INPUT", &rule)?;
        println!("Allowed traffic on port: {}", port);
        Ok(())
    }
    pub fn list_rules(&self) -> Result<(), Box<dyn std::error::Error>> {
        let rules = self.ipt.list("filter", "INPUT")?;
        for rule in rules {
            println!("Rule: {}", rule);
        }
        Ok(())
    }
}
