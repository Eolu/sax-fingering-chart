
#[macro_use]
extern crate lazy_static;

#[macro_use]
mod note;
mod keys;
mod track;
mod song;

#[cfg(test)]
mod tests;

use song::*;
use ron::de::from_str;
use serde::Deserialize;
use std::{env, path::Path, collections::BTreeMap};
use image::{error::ImageError};
use enumset::*;
use keys::Key;

// Load configuration before executing main program
lazy_static!
{
    static ref CONFIG: Result<Config, ron::error::Error> = 
    std::fs::read_to_string("./cfg.ron")
        .map_err(ron::error::Error::from)
        .and_then(|str| from_str(&str));
}

/// The note transposition to use, supports saxes of any kind
#[derive(Copy, Clone, Deserialize)]
pub enum TranspositionType
{
    Sopranissimo = -22,
    Sopranino = -15,
    Soprano = -10,
    Alto = -3,
    CMelody = 0,
    Tenor = 2,
    Baritone = 9,
    Bass = 14,
    Contrabass = 21,
    Subcontrabass = 26
}

/// The output format, determines how the chart images are layed out.
#[derive(Copy, Clone, Deserialize, PartialEq)]
pub enum OutputFormat
{
    Separate,
    Rows,
    Tracks
}

/// Data from the loaded cfg.ron file.
#[derive(Deserialize)]
pub struct Config 
{
    transposition_type: TranspositionType,
    output_path: String,
    output_format: OutputFormat,
    spacing: usize,
    notes_per_row: usize,
    notes: BTreeMap<u8, Vec<EnumSet<Key>>> 
}

/// Entry-point
fn main() -> Result<(), ImageError>
{
    match &*CONFIG
    {
        Ok(config) =>
        {
            for midi_file in env::args().skip(1)
            {
                let fingering_chart = Song::load(&midi_file, config.transposition_type as i16);
                let dir_name = Path::new(&midi_file).file_stem().unwrap().to_string_lossy();
                let output_path = format!("{}/{}", config.output_path, dir_name);
                match config.output_format
                {
                    OutputFormat::Tracks => fingering_chart.output_entire(&output_path, config.notes_per_row, config.spacing)?,
                    OutputFormat::Rows => fingering_chart.output_rows(&output_path, config.notes_per_row, config.spacing)?,
                    OutputFormat::Separate => fingering_chart.output_cells(&output_path)?
                }
            }
        }
        Err(e) => eprintln!("Failed to load config: {}", e)
    }
    Ok(())
}