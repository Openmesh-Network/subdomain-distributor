use chrono::{Datelike, Timelike, Utc};

use crate::utils::env::{
    domain, soa_expire, soa_mailbox, soa_minimum_ttl, soa_nameserver, soa_refresh, soa_retry, ttl,
};

pub fn main_file() -> String {
    format!(
        "{origin}
{soa}
{records}",
        origin = origin(domain().as_str()),
        soa = soa(),
        records = record(
            "@",
            "NS",
            format!("{nameserver}.", nameserver = soa_nameserver()).as_str()
        )
    )
}

pub fn subdomain_file(
    subdomain: &str,
    user: &str,
    ipv4: Option<&str>,
    ipv6: Option<&str>,
) -> String {
    format!(
        "{origin}
{soa}
{records}",
        origin = origin(format!("{subdomain}.{domain}", domain = domain()).as_str()),
        soa = soa(),
        records = vec![record("user", "TXT", user)]
            .into_iter()
            .chain(
                ipv4.map(|ipv4| vec![record("@", "A", ipv4), record("*", "A", ipv4)])
                    .unwrap_or_default()
            )
            .chain(
                ipv6.map(|ipv6| vec![record("@", "AAAA", ipv6), record("*", "AAAA", ipv6)])
                    .unwrap_or_default()
            )
            .collect::<Vec<String>>()
            .join("\n")
    )
}

pub fn origin(domain: &str) -> String {
    format!("$ORIGIN {domain}.")
}

pub fn soa() -> String {
    let time = Utc::now();
    format!(
        "@ {ttl} IN SOA {nameserver}. {mailbox}. {year:02}{month:02}{day:02}{hour:02}{minutes:02} {refresh} {retry} {expire} {minimum_ttl}",
        ttl = ttl(),
        nameserver = soa_nameserver(),
        mailbox = soa_mailbox(),
        refresh = soa_refresh(),
        retry = soa_retry(),
        expire = soa_expire(),
        minimum_ttl = soa_minimum_ttl(),
        year = time.year() % 100,
        month = time.month(),
        day = time.day(),
        hour = time.hour(),
        minutes = time.minute()
    )
}

pub fn record(zone: &str, record_type: &str, record_value: &str) -> String {
    format!("{zone} {ttl} IN {record_type} {record_value}", ttl = ttl())
}
