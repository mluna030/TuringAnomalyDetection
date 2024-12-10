use std::thread;
use std::time::Duration;

mod iptables_integration;

use iptables_integration::Firewall;
use simplelog::*;
use std::fs::File;

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        WriteLogger::new(LevelFilter::Debug, Config::default(), File::create("firewall.log").unwrap()),
    ])
    .unwrap();

    log::info!("Starting the Router Node...");

    match Firewall::new() {
        Ok(firewall) => {
            log::info!("Firewall initialized.");

            if let Err(e) = firewall.block_ip("192.168.1.100") {
                log::error!("Failed to block IP: {}", e);
            }

            if let Err(e) = firewall.allow_port(8080) {
                log::error!("Failed to allow port: {}", e);
            }

            if let Err(e) = firewall.list_rules() {
                log::error!("Failed to list rules: {}", e);
            }
        }
        Err(e) => {
            log::error!("Failed to initialize the firewall: {}", e);
        }
    }

    // Keep the program running
    log::info!("Router Node is now running. Press Ctrl+C to stop.");
    loop {
        thread::sleep(Duration::from_secs(60));
    }
}