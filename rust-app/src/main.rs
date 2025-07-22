use std::fs::{create_dir_all, write};

use actix_cors::Cors;
use actix_web::{App, HttpServer};

use crate::utils::{
    coredns::main_file,
    env::{datadir, domain, hostname, port, zonesdir},
};

mod api;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // Create data directories
    {
        let dir = datadir();
        create_dir_all(&dir).inspect_err(|e| {
            log::error!(
                "Could not create data dir at {dir}: {e}",
                dir = dir.display()
            )
        })?;
    }
    {
        let dir = zonesdir();
        create_dir_all(&dir).inspect_err(|e| {
            log::error!(
                "Could not create zones dir at {dir}: {e}",
                dir = dir.display()
            )
        })?;
    }

    // Write main domain zone file
    {
        let path = zonesdir().join(format!("db.{domain}", domain = domain()));
        if let Err(e) = write(path, main_file()) {
            panic!("Could not write main domain zone file: {e}");
        }
    }

    // Start server
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .configure(api::configure)
    })
    .bind(format!(
        "{hostname}:{port}",
        hostname = hostname(),
        port = port()
    ))?
    .run()
    .await
}
