use std::path::{Path, PathBuf};

fn env_var(id: &str) -> Option<String> {
    std::env::var(id)
        .inspect_err(|e| {
            log::warn!("Could not read env var {id}: {e}");
        })
        .ok()
}

pub fn hostname() -> String {
    env_var("HOSTNAME").unwrap_or("0.0.0.0".to_string())
}

pub fn port() -> String {
    env_var("PORT").unwrap_or("42923".to_string())
}

pub fn domain() -> String {
    env_var("DOMAIN").expect("No domain specified.")
}

pub fn ttl() -> String {
    env_var("TTL").expect("No TTL specified.")
}

pub fn soa_nameserver() -> String {
    env_var("SOANAMESERVER").expect("No SOA nameserver specified.")
}

pub fn soa_mailbox() -> String {
    env_var("SOAMAILBOX")
        .expect("No SOA mailbox specified.")
        .replace("@", ".")
}

pub fn soa_refresh() -> String {
    env_var("SOAREFRESH").expect("No SOA refresh specified.")
}

pub fn soa_retry() -> String {
    env_var("SOARETRY").expect("No SOA retry specified.")
}

pub fn soa_expire() -> String {
    env_var("SOAEXPIRE").expect("No SOA expire specified.")
}

pub fn soa_minimum_ttl() -> String {
    env_var("SOAMINIMUMTTL").expect("No SOA minimum ttl specified.")
}

pub fn datadir() -> PathBuf {
    env_var("DATADIR")
        .map(|d| Path::new(&d).to_path_buf())
        .unwrap_or(Path::new("/var/lib/subdomain-distributor").to_path_buf())
}

pub fn zonesdir() -> PathBuf {
    env_var("ZONESDIR")
        .map(|d| Path::new(&d).to_path_buf())
        .unwrap_or(Path::new(&datadir()).join("zones"))
}
