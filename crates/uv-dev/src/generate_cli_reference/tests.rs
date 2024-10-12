use std::env;

use crate::generate_all::Mode;
use anyhow::Result;
use uv_static::EnvVars;

use super::{main, Args};

#[test]
fn test_generate_cli_reference() -> Result<()> {
    let mode = if env::var(EnvVars::UV_UPDATE_SCHEMA).as_deref() == Ok("1") {
        Mode::Write
    } else {
        Mode::Check
    };
    main(&Args { mode })
}
