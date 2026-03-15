#![allow(warnings)]
#![allow(dependency_on_unit_never_type_fallback)]
pub mod io;
pub mod testing;
pub mod json;
pub mod net;
pub mod regex;
pub mod temporal;
mod support;
pub (crate) use support::*;
pub fn init(config: Option<temper_core::Config>) -> temper_core::Result<temper_core::AsyncRunner> {
    crate::CONFIG.get_or_init(| | config.unwrap_or_else(| | temper_core::Config::default()));
    io::init() ? ;
    testing::init() ? ;
    json::init() ? ;
    net::init() ? ;
    regex::init() ? ;
    temporal::init() ? ;
    Ok(crate::config().runner().clone())
}
