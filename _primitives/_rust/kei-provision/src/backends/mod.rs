//! Backend registry. Add a new cloud: implement `Backend`, register here.

pub mod hetzner;
pub mod vultr;

use crate::backend::Backend;
use anyhow::{anyhow, Result};

pub fn resolve(name: &str) -> Result<Box<dyn Backend>> {
    match name {
        "hetzner" => Ok(Box::new(hetzner::HetznerBackend::new())),
        "vultr" => Ok(Box::new(vultr::VultrBackend::new())),
        other => Err(anyhow!(
            "unknown backend `{other}`. Known: hetzner, vultr. (aws, do, linode: planned)"
        )),
    }
}
