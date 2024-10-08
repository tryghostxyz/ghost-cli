use crate::types::{GraphConfig, GraphFile};
use eyre::{eyre, EyreHandler};
use std::error::Error;
use std::fs::{self};
use std::path::{Path, PathBuf};
use tracing::debug;
use yansi::Paint;

pub fn check_and_get_conf(required_files: &[&str]) -> eyre::Result<GraphConfig> {
    if !Path::new("config.json").exists() {
        return Err(eyre!(
            "config.json not found. This command can only be run in a ghost directory"
        ));
    }

    for &file in required_files {
        if !Path::new(file).exists() {
            return Err(eyre!("{} not found", file));
        }
    }

    GraphConfig::read(PathBuf::from("config.json")).map_err(|_| eyre!("cannot read config.json"))
}

pub fn check_and_create_dir(dir: &PathBuf) -> eyre::Result<()> {
    if !dir.exists() {
        fs::create_dir_all(dir)?;
    }
    let dir = dunce::canonicalize(dir)?;
    if dir.read_dir().map_or(false, |mut i| i.next().is_some()) {
        eyre::bail!("Cannot run `create` on a non-empty directory");
    }
    Ok(())
}

pub fn write_files(dir: &Path, sources: Vec<GraphFile>) -> eyre::Result<()> {
    let src = dir.join("src");
    fs::create_dir_all(&src)?;
    for source in sources {
        fs::write(src.join(source.path), source.code)?;
    }
    Ok(())
}

pub fn write_sources_and_conf(
    dir: &Path,
    id: String,
    version_id: String,
    sources: Vec<GraphFile>,
) -> eyre::Result<()> {
    write_files(dir, sources)?;
    GraphConfig { id, version_id }.write(dir.join("config.json"))?;
    Ok(())
}

/**
Source: foundry (crates/cli/src/handler.rs) subject to the same Apache/MIT license
 */

#[derive(Debug)]
pub struct Handler;

impl EyreHandler for Handler {
    fn debug(
        &self,
        error: &(dyn Error + 'static),
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        if f.alternate() {
            return core::fmt::Debug::fmt(error, f);
        }
        writeln!(f)?;
        write!(f, "{}", error.red())?;

        if let Some(cause) = error.source() {
            write!(f, "\n\nContext:")?;

            let multiple = cause.source().is_some();
            let errors = std::iter::successors(Some(cause), |e| (*e).source());

            for (n, error) in errors.enumerate() {
                writeln!(f)?;
                if multiple {
                    write!(f, "- Error #{n}: {error}")?;
                } else {
                    write!(f, "- {error}")?;
                }
            }
        }

        Ok(())
    }
}

pub unsafe fn install_handler() {
    if std::env::var_os("RUST_BACKTRACE").is_none() {
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    let (panic_hook, _) = color_eyre::config::HookBuilder::default()
        .panic_section("Please report to https://t.me/ghostlogsxyz")
        .into_hooks();
    panic_hook.install();
    if let Err(e) = eyre::set_hook(Box::new(move |_| Box::new(Handler))) {
        debug!("failed to install eyre error hook: {e}");
    }
}
