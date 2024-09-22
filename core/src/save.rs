use crate::{config::TakeSnapshotParams, path::parse_save_path, snapshot::take_snapshot};
use anyhow::{bail, Result};

pub fn save_snapshot(config: TakeSnapshotParams) -> Result<()> {
    match &config.save_path {
        Some(path) => {
            if !path.ends_with(".png") {
                bail!("The save_path must ends with .png");
            }

            let pixmap = take_snapshot(config.clone())?;
            let path = parse_save_path(path.to_string())?;

            pixmap.save_png(path)?;
        }
        None => bail!("Cannot find 'save_path' in config"),
    }

    Ok(())
}
