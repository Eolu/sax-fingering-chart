use crate::{track::*, note::{Note, NoteConst}};
use std::{fs, collections::HashSet};
use midly::{Smf, EventKind::*, MidiMessage::*};
use image::{error::ImageError, GenericImageView, imageops::FilterType};

/// Entire song, just a vec of tracks with some methods
pub struct Song(Vec<Track>);

impl Song
{
    /// Parse a midi file. Generate a list containing all tracks. Tracks themselves are simply lists of notes.
    pub fn load(midi_path: &str, transposition: u8) -> Song
    {
        // Parse a midi file
        let raw_data = fs::read(midi_path).expect("Failed to load midi data");
        let midi = Smf::parse(&raw_data).expect("Failed to parse midi data");

        // Keep track of out-of-range notes
        let mut out_of_range: HashSet<u8> = HashSet::new();

        // Iterate through the midi file and collect notes
        Song(midi.tracks
            .iter()
            .map(|track| track
                .iter()
                .filter_map(|event| 
                    if let Midi { channel: _, message: NoteOn {key, vel: _} } = event.kind 
                    { 
                        let true_key = key.as_int() + transposition;
                        let candidates = Note::get(true_key);
                        match candidates
                        {
                            None => 
                            {
                                if out_of_range.insert(true_key)
                                {
                                    eprintln!("Note out of range: {}", true_key);
                                }
                                None
                            },
                            note => note
                        }
                    } 
                    else 
                    { 
                        None 
                    })
                .collect::<Vec<NoteConst>>())
            .filter_map(|candidates| 
                if candidates.is_empty() 
                { 
                    None 
                } 
                else 
                {
                    Some(Track(candidates)) 
                })
            .collect())
    }

    /// Output chart with each cell as an individual file
    pub fn output_cells(&self, output_path: &str) -> Result<(), ImageError>
    {
        for (i, track) in self.tracks().enumerate()
        {
            let track_path = format!("{}/track{}", output_path, i);
            fs::create_dir_all(&track_path)?;
            for (cell, image) in track.cell_images().iter().enumerate()
            {
                let image = image.resize(image.width() * 2, image.height() * 2, FilterType::Nearest);
                image.save(format!("{}/{}.png", track_path, cell))?;
            }
        }
        Ok(())
    }

    /// Output chart with each row as an individual file
    pub fn output_rows(&self, output_path: &str, notes_per_row: usize, spacing: usize) -> Result<(), ImageError>
    {
        for (i, track) in self.tracks().enumerate()
        {
            let track_path = format!("{}/track{}", output_path, i);
            fs::create_dir_all(&track_path)?;
            for (row, image) in track.row_images(notes_per_row, spacing).iter().enumerate() 
            {
                let image = image.resize(image.width() * 2, image.height() * 2, FilterType::Nearest);
                image.save(format!("{}/row{}.png", track_path, row))?;
            }
        }
        Ok(())
    }

    /// Generate a chart and output to the given directory.
    pub fn output_entire(&self, output_path: &str, notes_per_row: usize, spacing: usize) -> Result<(), ImageError>
    {
        fs::create_dir_all(output_path)?;
        for (i, track) in self.tracks().enumerate()
        {
            let image = track.track_image(notes_per_row, spacing);
            let image = image.resize(image.width() * 2, image.height() * 2, FilterType::Nearest);
            image.save(format!("{}/track{}.png", output_path, i))?;
        }
        Ok(())
    }

    /// Access notes vector
    fn tracks(&self) -> impl Iterator<Item = &Track>
    {
        self.0.iter()
    }
}

