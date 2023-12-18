use crate::{load_image_files, set_wallpaper_on_config, Configuration};
use anyhow::{anyhow, Context};
use rand::seq::SliceRandom;
use tokio::runtime::Runtime;

pub fn pick_random_image(settings: Configuration) -> anyhow::Result<()> {
    let path = settings
        .wallpapers_path
        .ok_or(anyhow!("No `wallpaper_path` on config."))?;
    let rt = Runtime::new().context("Failed to create runtime.")?;
    rt.block_on(async {
        let mut rng = rand::thread_rng();
        let image_path = load_image_files(path)
            .await?
            .choose(&mut rng)
            .ok_or(anyhow!("No files loaded."))?
            .clone();
        set_wallpaper_on_config(image_path).await?;
        Ok(())
    })
}
