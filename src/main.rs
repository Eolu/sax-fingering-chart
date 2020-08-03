#![allow(non_upper_case_globals)]
#[macro_use]
extern crate lazy_static;

use std::{fs, fmt::Display};
use midly::{Smf, EventKind::*, MidiMessage::*, number::u7};
use image::{DynamicImage, GenericImageView, GenericImage};

const SPACING: usize = 10;
const CHART_DIR: &str = "C:/Project/Workspace/Rust/fingering_chart/res/fingerings";
const OUTPUT_PATH: &str = "C:/Project/Workspace/Rust/fingering_chart/out";
const MIDI_PATH: &str = "C:/Project/Workspace/Rust/fingering_chart/res/kass_notes.mid";
const NOTES_PER_ROW: usize = 7;

// TODO: Add altissimo fingerings
// TODO: Add trill fingerings
// TODO: Transpose midi file so most amount of notes can fit. Try octaves first.
// Then try semitones (with warning). If impossible, leave out notes (with
// warning).
// TODO: Make this more general, right not it's hard-coded for the tenor sax (Bb transposed)

/// Macro to load notes using lazy_static
#[macro_export]
macro_rules! load_note
{
    ($name: ident, $num: literal) => 
    {
        lazy_static!
        {
            static ref $name: Note = Note::new($num, format!("{}/{}.png", CHART_DIR, stringify!($name)));
        }
    }
}

// Note constants
load_note!(Ab2, 44);
load_note!(A2, 45);
load_note!(Bb2, 46);
load_note!(B2, 47);
load_note!(C3, 48); 
load_note!(Db3, 49); 
load_note!(D3, 50); 
load_note!(Eb3, 51); 
load_note!(E3, 52); 
load_note!(F3, 53); 
load_note!(Gb3, 54); 
load_note!(G3, 55); 
load_note!(Ab3, 56); 
load_note!(A3, 57); 
load_note!(Bb3, 58); 
load_note!(B3, 59);
load_note!(C4, 60); 
load_note!(Db4, 61); 
load_note!(D4, 62); 
load_note!(Eb4, 63); 
load_note!(E4, 64); 
load_note!(F4, 65); 
load_note!(Gb4, 66); 
load_note!(G4, 67); 
load_note!(Ab4, 68); 
load_note!(A4, 69); 
load_note!(Bb4, 70); 
load_note!(B4, 71);
load_note!(C5, 72); 
load_note!(Db5, 73); 
load_note!(D5, 74); 
load_note!(Eb5, 75);
// Altissimo from here beyond: 
// load_note!(E5, 76);

/// Entry-point
fn main() -> Result<(), image::error::ImageError>
{
    let fingering_chart = Song::load(MIDI_PATH);
    fingering_chart.output_entire(OUTPUT_PATH, NOTES_PER_ROW, SPACING)?;
    println!("{}", fingering_chart);
    Ok(())
}

/// Entire song, just a vec of tracks with some methods
pub struct Song
{
    tracks: Vec<Track>
}

/// Struct used for tracks
pub struct Track
{
    notes: Vec<&'static Note>
}

/// Struct used for individual notes
pub struct Note
{
    name: String,
    byte: u8,
    image: image::DynamicImage
}

impl Song
{
    /// Parse a midi file. Generate a list containing all tracks. Tracks themselves are simply lists of notes.
    pub fn load(midi_path: &str) -> Song
    {
        // Parse a midi file
        let raw_data = fs::read(midi_path).expect("Failed to load midi data");
        let midi = Smf::parse(&raw_data).expect("Failed to parse midi data");

        // Iterate through the midi file and collect notes
        Song {tracks: midi.tracks
            .iter()
            .map(|track| track
                .iter()
                .filter_map(|event| if let Midi { channel: _, message: NoteOn {key, vel: _} } = event.kind { Note::get(key) } else { None })
                .collect::<Vec<&Note>>())
            .filter_map(|notes| if notes.is_empty() { None } else { Some(Track{notes}) })
            .collect::<Vec<Track>>()}
    }

    /// Output chart with each cell as an individual file
    pub fn output_cells(&self, output_path: &str) -> Result<(), image::error::ImageError>
    {
        for track_index in 0..self.tracks.len()
        {
            let track_path = format!("{}/track{}", output_path, track_index);
            fs::create_dir_all(&track_path)?;
            for (cell, note) in self.tracks[track_index].notes.iter().enumerate() 
            {
                note.image.save(format!("{}/cell{}.png", track_path, cell))?;
            }
        }
        Ok(())
    }

    /// Output chart with each row as an individual file
    pub fn output_rows(&self, output_path: &str, notes_per_row: usize, spacing: usize) -> Result<(), image::error::ImageError>
    {
        for track_index in 0..self.tracks.len()
        {
            let track_path = format!("{}/track{}", output_path, track_index);
            fs::create_dir_all(&track_path)?;
            for (row, image) in self.tracks[track_index].row_images(notes_per_row, spacing).iter().enumerate() 
            {
                image.save(format!("{}/row{}.png", track_path, row))?;
            }
        }
        Ok(())
    }

    /// Generate a chart and output to the given directory.
    pub fn output_entire(&self, output_path: &str, notes_per_row: usize, spacing: usize) -> Result<(), image::error::ImageError>
    {
        fs::create_dir_all(output_path)?;
        for track_index in 0..self.tracks.len()
        {
            let track_image = self.tracks[track_index].track_image(notes_per_row, spacing);
            track_image.save(format!("{}/track{}.png", output_path, track_index))?;
        }
        Ok(())
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
            track_image = DynamicImage::new_rgb8(std::cmp::max(previous.width(), row_image.width()), previous.height() + row_image.height() + spacing as u32);
            track_image.copy_from(&previous, 0, 0).expect("Failed to regenerate track image");
            track_image.copy_from(&row_image, 0, previous.height()).expect("Failed to generate track image");
        }
        track_image
    }

    /// Generates images in rows
    pub fn row_images(&self, notes_per_row: usize, spacing: usize) -> Vec<DynamicImage>
    {
        (0..self.notes.len())
            .take_while(move |row| row * notes_per_row < self.notes.len())
            .map(move |row| 
            {
                let mut row_image: DynamicImage = DynamicImage::new_rgb8(spacing as u32, 0);
                for note in (0..notes_per_row)
                    .take_while(|col| col + (row * notes_per_row) < self.notes.len())
                    .map(|col| self.notes[(row * notes_per_row) + col])
                {
                    let previous = row_image;
                    row_image = DynamicImage::new_rgb8(previous.width() + note.image.width() + spacing as u32, std::cmp::max(previous.height(), note.image.height()));
                    row_image.copy_from(&previous, 0, 0).expect("Failed to regenerate row image");
                    row_image.copy_from(&note.image, previous.width(), 0).expect("Failed to generate row image");
                }
                row_image
            }).collect()
    }

    /// Returns an iterator into cell images. The images themselves are 
    /// generated at load-time, so this method is low-cost.
    pub fn cell_images(&self) -> impl Iterator<Item = &DynamicImage>
    {
        self.notes.iter().map(|note| &note.image)
    }
}

impl Note
{
    /// Note constructor. Should only be called in a lazy_static way
    fn new(byte: u8, image_path: String) -> Note
    {
        let image = image::open(&image_path).expect(&format!("Failed to read {}", image_path));
        let name = image_path[image_path.rfind('/').expect("Bad path") + 1 .. image_path.len() - 4].to_string();
        Note { name, byte, image }
    }

    /// Access a note via it's midi byte index. To support additional notes, they must be added to this function.
    fn get(index: u7) -> Option<&'static Note>
    {
        match index.as_int()
        {
            i if i == Ab2.byte => Some(&Ab2),
            i if i == A2.byte  => Some(&A2),
            i if i == Bb2.byte => Some(&Bb2),
            i if i == B2.byte  => Some(&B2),
            i if i == C3.byte  => Some(&C3),
            i if i == Db3.byte => Some(&Db3),
            i if i == D3.byte  => Some(&D3), 
            i if i == Eb3.byte => Some(&Eb3), 
            i if i == E3.byte  => Some(&E3),
            i if i == F3.byte  => Some(&F3),
            i if i == Gb3.byte => Some(&Gb3),
            i if i == G3.byte  => Some(&G3),
            i if i == Ab3.byte => Some(&Ab3),
            i if i == A3.byte  => Some(&A3),
            i if i == Bb3.byte => Some(&Bb3),
            i if i == B3.byte  => Some(&B3),
            i if i == C4.byte  => Some(&C4),
            i if i == Db4.byte => Some(&Db4),
            i if i == D4.byte  => Some(&D4),
            i if i == Eb4.byte => Some(&Eb4),
            i if i == E4.byte  => Some(&E4),
            i if i == F4.byte  => Some(&F4),
            i if i == Gb4.byte => Some(&Gb4),
            i if i == G4.byte  => Some(&G4),
            i if i == Ab4.byte => Some(&Ab4),
            i if i == A4.byte  => Some(&A4),
            i if i == Bb4.byte => Some(&Bb4),
            i if i == B4.byte  => Some(&B4),
            i if i == C5.byte  => Some(&C5),
            i if i == Db5.byte => Some(&Db5),
            i if i == D5.byte  => Some(&D5),
            i if i == Eb5.byte => Some(&Eb5),
            _ => None
        }
    }
}

/// Prints every track in a song
impl Display for Song
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result 
    {
        for (i, track) in self.tracks.iter().enumerate()
        {
            if i > 0
            {
                if let Err(e) = write!(f, "\n")
                {
                    return Err(e);
                }
            }
            if let Err(e) = write!(f, "Track:{}", track)
            {
                return Err(e);
            }
        }
        Ok(())
    }
}

/// Prints every note in a track
impl Display for Track
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result 
    {
        for note in self.notes.iter()
        {
            if let Err(e) = write!(f, "{} ", note)
            {
                return Err(e);
            }
        }
        Ok(())
    }
}

/// Prints the name of a note
impl Display for Note
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result 
    {
        write!(f, "{}", self.name)
    }
}