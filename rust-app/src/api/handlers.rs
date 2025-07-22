use std::fs::{exists, write};

use actix_web::{HttpResponse, Responder, get, post, web};
use regex::Regex;

use crate::{
    api::models::Reserve,
    utils::{
        coredns::subdomain_file,
        env::{domain, zonesdir},
        error::ResponseError,
    },
};

#[get("/{subdomain}/available")]
async fn available(path: web::Path<String>) -> impl Responder {
    let subdomain = path.into_inner();
    let free = is_free(&subdomain);
    HttpResponse::Ok().json(free)
}

#[post("/{subdomain}/reserve")]
async fn reserve(path: web::Path<String>, reservation: web::Json<Reserve>) -> impl Responder {
    let subdomain = path.into_inner();

    let invalid = invalid_arg(
        [(
            "Subomdain",
            subdomain.as_str(),
            Regex::new(r"^[A-Za-z0-9](?:[A-Za-z0-9\-]{0,61}[A-Za-z0-9])?$").expect("Invalid Subdomain Regex"),
        ), (
            "User",
            reservation.user.as_str(),
            Regex::new(r"^.*$").expect("Invalid User Regex"),
        )]
        .into_iter()
        .chain(
            reservation
                .ipv4
                .as_ref()
                .map(|ipv4| {
                    vec![(
                        "ipv4",
                        ipv4.as_str(),
                        Regex::new(r"^((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)\.?\b){4}$")
                            .expect("Invalid ipv4 Regex"),
                    )]
                })
                .unwrap_or_default(),
        )
        .chain(
            reservation
                .ipv6
                .as_ref()
                .map(|ipv6| {
                    vec![(
                        "ipv6",
                        ipv6.as_str(),
                        Regex::new(r"(([0-9a-fA-F]{1,4}:){7,7}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,7}:|([0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,5}(:[0-9a-fA-F]{1,4}){1,2}|([0-9a-fA-F]{1,4}:){1,4}(:[0-9a-fA-F]{1,4}){1,3}|([0-9a-fA-F]{1,4}:){1,3}(:[0-9a-fA-F]{1,4}){1,4}|([0-9a-fA-F]{1,4}:){1,2}(:[0-9a-fA-F]{1,4}){1,5}|[0-9a-fA-F]{1,4}:((:[0-9a-fA-F]{1,4}){1,6})|:((:[0-9a-fA-F]{1,4}){1,7}|:)|fe80:(:[0-9a-fA-F]{0,4}){0,4}%[0-9a-zA-Z]{1,}|::(ffff(:0{1,4}){0,1}:){0,1}((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])|([0-9a-fA-F]{1,4}:){1,4}:((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9]))").expect("Invalid ipv6 Regex"),
                    )]
                })
                .unwrap_or_default(),
        )
        .collect(),
    );
    if let Some(arg) = invalid {
        return HttpResponse::BadRequest().json(ResponseError::new(format!(
            "Argument {arg} does not satisfy the expected format."
        )));
    }

    let free = is_free(&subdomain);
    if !free {
        return HttpResponse::BadRequest().json(ResponseError::new(format!(
            "Subdomain {subdomain} already reserved."
        )));
    }

    let path = zonesdir().join(format!("db.{subdomain}.{domain}", domain = domain()));
    if let Err(e) = write(
        path,
        subdomain_file(
            &subdomain,
            &reservation.user,
            reservation.ipv4.as_deref(),
            reservation.ipv6.as_deref(),
        ),
    ) {
        log::error!("Reserving subdomain {subdomain} file write error: {e}");
        return HttpResponse::InternalServerError().json(ResponseError::new(format!(
            "Could not reserve subdomain {subdomain}."
        )));
    }

    log::info!(
        "Subdomain {subdomain} got reserved by {user}",
        user = reservation.user
    );
    HttpResponse::Ok().finish()
}

fn is_free(subdomain: &str) -> bool {
    let path = zonesdir().join(format!("db.{subdomain}.{domain}", domain = domain()));
    exists(path).is_ok_and(|exists| !exists)
}

fn invalid_arg(args: Vec<(&str, &str, Regex)>) -> Option<String> {
    for (key, value, regex) in args {
        if !regex.is_match(value) {
            return Some(key.to_string());
        }
    }

    None
}
