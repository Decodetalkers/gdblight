use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

const BACKLIGHT_PATTERNL: &str = "/sys/class/backlight/*";
const MAX_LIGHT_FILE: &str = "max_brightness";
const BRIGHT_FILE: &str = "brightness";

#[derive(Debug, thiserror::Error)]
pub enum BackLightError {
    #[error("Error during glob")]
    GlobError(#[from] glob::PatternError),
    #[error("Io Error")]
    IoError(#[from] io::Error),
    #[error("Parse Error")]
    ParseError,
    #[error("Not find a backlight file")]
    BackLightNotFound,
}

#[derive(Debug)]
pub struct BackLightInfo {
    max_light: u32,
    current_light: u32,
    file_path: PathBuf,
}

fn read_to_light<P: AsRef<Path>>(path: P) -> Result<u32, BackLightError> {
    let data = fs::read_to_string(path)?;
    data.trim().parse().map_err(|_| BackLightError::ParseError)
}

#[allow(unused)]
impl BackLightInfo {
    pub fn new() -> Result<Self, BackLightError> {
        let paths = glob::glob(BACKLIGHT_PATTERNL)?;
        let pa = paths
            .flatten()
            .next()
            .ok_or(BackLightError::BackLightNotFound)?;

        let maxlightfile = pa.join(MAX_LIGHT_FILE);
        let max_light = read_to_light(maxlightfile)?;
        let lightfile = pa.join(BRIGHT_FILE);
        let current_light = read_to_light(&lightfile)?;
        Ok(Self {
            max_light,
            current_light,
            file_path: lightfile,
        })
    }

    pub fn max_light(&self) -> u32 {
        self.max_light
    }

    pub fn current_light(&self) -> u32 {
        self.current_light
    }

    pub fn get_light_percent(&self) -> u32 {
        self.current_light * 100 / self.max_light
    }

    pub fn set_light_percent(&self, percent: u32) -> io::Result<()> {
        let mut file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .open(&self.file_path)?;
        let final_value = (self.max_light * percent / 100).to_string();
        file.write_all(final_value.as_bytes())
    }
}
