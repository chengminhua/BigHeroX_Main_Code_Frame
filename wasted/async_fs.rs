use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::error::BigHeroXError;

#[allow(unused)]
async fn open_file() -> Result<(PathBuf, Arc<Vec<u8>>), BigHeroXError> {
    let picked_file = rfd::AsyncFileDialog::new()
        .set_title("Open a image...")
        .pick_file()
        .await
        .ok_or(BigHeroXError::DialogClosed)?;

    load_file(picked_file.path().to_owned()).await
}

#[allow(unused)]
async fn load_file(path: PathBuf) -> Result<(PathBuf, Arc<Vec<u8>>), BigHeroXError> {
    let contents = async_std::fs::read(&path)
        .await
        .map(Arc::new)
        .map_err(BigHeroXError::from)?;

    Ok((path, contents))
}

#[allow(unused)]
async fn pick_save_text_to_file(
    path: Option<PathBuf>,
    contents: String,
) -> Result<PathBuf, BigHeroXError> {
    let path = if let Some(path) = path {
        path
    } else {
        rfd::AsyncFileDialog::new()
            .save_file()
            .await
            .as_ref()
            .map(rfd::FileHandle::path)
            .map(Path::to_owned)
            .ok_or(BigHeroXError::DialogClosed)?
    };

    async_std::fs::write(&path, contents)
        .await
        .map_err(|error| BigHeroXError::IoError(error.kind()))?;

    Ok(path)
}
