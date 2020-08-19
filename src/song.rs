use crate::{note::Note, Transposition};
use std::{fs, collections::HashSet};
use midly::{Smf, EventKind::*, MidiMessage::*};
use image::{DynamicImage, GenericImageView, GenericImage, error::ImageError};

/// Entire song, just a vec of tracks with some methods
pub struct Song(Vec<Track>);
/// Struct used for tracks
pub struct Track(Vec<&'static Note>);

impl Song
{
    /// Parse a midi file. Generate a list containing all tracks. Tracks themselves are simply lists of notes.
    pub fn load(midi_path: &str, transposition: Transposition) -> Song
    {
        // Parse a midi file
        let raw_data = fs::read(midi_path).expect("Failed to load midi data");
        let midi = Smf::parse(&raw_data).expect("Failed to parse midi data");

        // Keep track of notes
        let mut notes: HashSet<u8> = HashSet::new();

        // Iterate through the midi file and collect notes
        Song(midi.tracks
            .iter()
            .map(|track| track
                .iter()
                .filter_map(|event| 
                    if let Midi { channel: _, message: NoteOn {key, vel: _} } = event.kind 
                    { 
                        let true_key = key.as_int() + transposition as u8;
                        let candidates = Note::get(true_key);
                        if candidates.is_empty()
                        {
                            if notes.insert(true_key)
                            {
                                eprintln!("Note out of range: {}", true_key);
                            }
                            None
                        }
                        else
                        {
                            Some(candidates)
                        }
                    } 
                    else 
                    { 
                        None 
                    })
                .collect::<Vec<Vec<&'static Note>>>())
            .filter_map(|candidates| 
                if candidates.is_empty() 
                { 
                    None 
                } 
                else 
                {
                    Some(Track(Self::select_fingerings(candidates))) 
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
            for (cell, note) in track.notes().iter().enumerate() 
            {
                note.image.save(format!("{}/{}.png", track_path, cell))?;
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
            track
                .track_image(notes_per_row, spacing)
                .save(format!("{}/track{}.png", output_path, i))?;
        }
        Ok(())
    }

    /// Access notes vector
    fn tracks(&self) -> impl Iterator<Item = &Track>
    {
        self.0.iter()
    }

    /// Go through candidates and determine which fingerings to use
    fn select_fingerings(candidates: Vec<Vec<&'static Note>>) -> Vec<&'static Note>
    {
        let mut previous_choice: Option<&'static Note> = None;
        candidates
            .iter()
            .enumerate()
            .filter_map(|(i, choices)| 
            {
                // Determine how alike a note is from a list of candidates
                fn intersection<'a>(note: &'static Note, candidates: impl Iterator<Item = &'a&'static Note>) -> i32
                {
                    candidates
                        .map(|note|*note)
                        .map(|candidate|candidate.keys)
                        .map(|keys|note.keys.intersection(keys).len())
                        .max()
                        .unwrap() as i32
                }

                // Determine how different a note is from a list of candidates
                fn difference<'a>(note: &'static Note, candidates: impl Iterator<Item = &'a&'static Note>) -> i32
                {
                    candidates
                        .map(|note|*note)
                        .map(|candidate|candidate.keys)
                        .map(|keys|note.keys.symmetrical_difference(keys).len())
                        .max()
                        .unwrap() as i32
                }

                if choices.len() == 1
                {
                    previous_choice = Some(choices[0]);
                }
                else if i == 0
                {
                    let next = &candidates[i + 1];
                    previous_choice = Some(choices.iter().fold(choices[0], |note_1, note_2| 
                    {
                        let note_1_intersection = intersection(note_1, next.iter());
                        let note_1_diff = difference(note_1, next.iter());
                        let note_2_intersection = intersection(note_2, next.iter());
                        let note_2_diff = difference(note_2, next.iter());
                        if note_2_intersection - note_2_diff > note_1_intersection - note_1_diff
                        {
                            note_2
                        }
                        else
                        {
                            note_1
                        }
                    }));
                }
                else if i == candidates.len() - 1
                {
                    previous_choice = Some(choices.iter().fold(choices[0], |note_1, note_2| 
                    {
                        let note_1_intersection = intersection(note_1, previous_choice.iter());
                        let note_1_diff = difference(note_1, previous_choice.iter());
                        let note_2_intersection = intersection(note_2, previous_choice.iter());
                        let note_2_diff = difference(note_2, previous_choice.iter());
                        if note_2_intersection - note_2_diff > note_1_intersection - note_1_diff
                        {
                            note_2
                        }
                        else
                        {
                            note_1
                        }
                    }));
                }
                else
                {
                    let next = &candidates[i + 1];
                    previous_choice = Some(choices.iter().fold(choices[0], |note_1, note_2| 
                    {
                        let note_1_intersection = 
                        intersection(note_1, next.iter()) + intersection(note_1, previous_choice.iter());
                        let note_1_diff = 
                        difference(note_1, next.iter()) + difference(note_1, previous_choice.iter());
                        let note_2_intersection = 
                        intersection(note_2, next.iter()) + intersection(note_2, previous_choice.iter());
                        let note_2_diff = 
                        difference(note_2, next.iter()) + difference(note_2, previous_choice.iter());
                        if note_2_intersection - note_2_diff > note_1_intersection - note_1_diff
                        {
                            note_2
                        }
                        else
                        {
                            note_1
                        }
                    }));
                }
                previous_choice
            })
            .collect()
    }
}

impl Track
{
    /// Generates an image for the entire track
    pub fn track_image(&self, notes_per_row: usize, spacing: usize) -> DynamicImage
    {
        let mut track_image: DynamicImage = DynamicImage::new_rgb8(0, spacing as u32);
        for row_image in self.row_images(notes_per_row, spacing)
        {
            let previous = track_image;
            let new_width = std::cmp::max(previous.width(), row_image.width());
            let new_height = previous.height() + row_image.height() + spacing as u32;
            track_image = DynamicImage::new_rgb8(new_width, new_height);
            track_image.copy_from(&previous, 0, 0).expect("Failed to regenerate track image");
            track_image.copy_from(&row_image, 0, previous.height()).expect("Failed to generate track image");
        }
        track_image
    }

    /// Generates images in rows
    pub fn row_images(&self, notes_per_row: usize, spacing: usize) -> Vec<DynamicImage>
    {
        (0..self.notes().len())
            .take_while(|row| row * notes_per_row < self.notes().len())
            .map(|row| 
            {
                let mut row_image: DynamicImage = DynamicImage::new_rgb8(spacing as u32, 0);
                for note in (0..notes_per_row)
                    .take_while(|col| col + (row * notes_per_row) < self.notes().len())
                    .map(|col| self.notes()[(row * notes_per_row) + col])
                {
                    let previous = row_image;
                    let new_width = previous.width() + note.image.width() + spacing as u32;
                    let new_height = std::cmp::max(previous.height(), note.image.height());
                    row_image = DynamicImage::new_rgb8(new_width, new_height);
                    row_image.copy_from(&previous, 0, 0).expect("Failed to regenerate row image");
                    row_image.copy_from(&note.image, previous.width(), 0).expect("Failed to generate row image");
                }
                row_image
            }).collect()
    }

    /// Returns an iterator into cell images. The images themselves are 
    /// generated at load-time, so this method is low-cost.
    #[allow(dead_code)]
    pub fn cell_images(&self) -> impl Iterator<Item = &DynamicImage>
    {
        self.notes().iter().map(|note| &note.image)
    }

    /// Access notes vector
    fn notes(&self) -> &Vec<&'static Note>
    {
        &self.0
    }
}
