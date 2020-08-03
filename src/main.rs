#![allow(non_upper_case_globals)]

#[macro_use]
extern crate lazy_static;

use std::{fs, fmt};
use midly::{Smf, EventKind::*, MidiMessage::*, number::u7};
use image::{DynamicImage, GenericImageView, GenericImage};

const CHART_DIR: &str = "C:/Project/Workspace/Rust/fingering_chart/res/fingerings";
const OUTPUT_PATH: &str = "C:/Project/Workspace/Rust/fingering_chart/out";
const MIDI_PATH: &str = "C:/Project/Workspace/Rust/fingering_chart/res/kass_notes.mid";
const NOTES_PER_ROW: usize = 7;

// TODO: Add altissimo fingerings
// TODO: Add trill fingerings
// TODO: Transpose midi file so most amount of notes can fit. Try octaves first.
// Then try semitones (with warning). If impossible, leave out notes (with
// warning).

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
    const spacing: usize = 10;
    let fingering_chart = load_midi(MIDI_PATH);
    fingering_chart.generate_chart(OUTPUT_PATH, NOTES_PER_ROW, spacing)?;
    println!("{}", fingering_chart);
    Ok(())
}

/// Parse a midi file. Generate a list containing all tracks. Tracks themselves are simply lists of notes.
fn load_midi(midi_path: &str) -> Song
{
    // Parse a midi file
    let raw_data = fs::read(midi_path).unwrap();
    let midi = Smf::parse(&raw_data).unwrap();

    // Use the information
    println!("midi file has {} tracks", midi.tracks.len());

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

struct Song
{
    tracks: Vec<Track>
}

impl Song
{
    /// Generate a chart and output to the given directory.
    fn generate_chart(&self, output_path: &str, notes_per_row: usize, spacing: usize) -> Result<(), image::error::ImageError>
    {
        fs::create_dir_all(output_path)?;
        for track_index in 0..self.tracks.len()
        {
            let track = &self.tracks[track_index];
            let row_images: Vec<DynamicImage> = (0..track.notes.len())
                .take_while(|row| row * notes_per_row < track.notes.len())
                .map(|row| 
                {
                    let row_notes: Vec<&Note> = (0..notes_per_row)
                        .take_while(|col| col + (row * notes_per_row) < track.notes.len())
                        .map(|col| track.notes[(row * notes_per_row) + col])
                        .collect();

                    // Create an image
                    let width: u32 = row_notes.iter().map(|note| note.image.width()).sum::<u32>() + row_notes.len() as u32 * spacing as u32;
                    let height: u32 = row_notes.iter().map(|note| note.image.height()).max().unwrap_or(0);
                    let mut row_image: DynamicImage = DynamicImage::new_rgb8(width, height);

                    // Draw the image
                    let mut current_x: u32 = 0;
                    for note in row_notes
                    {
                        row_image.copy_from(&note.image, current_x, 0).unwrap();
                        current_x += note.image.width() + spacing as u32;
                    }

                    row_image
                })
                .collect();

            // Create an image with all notes of the entire track
            let width: u32 = row_images.iter().map(|image| image.width()).max().unwrap_or(0);
            let height: u32 = row_images.iter().map(|image| image.height()).sum::<u32>() + (row_images.len() * spacing) as u32;
            let mut track_image: DynamicImage = DynamicImage::new_rgb8(width, height);

            // Draw the image
            let mut current_y: u32 = 0;
            for row_image in row_images
            {
                track_image.copy_from(&row_image, 0, current_y)?;
                current_y += row_image.height() + spacing as u32;
            }

            // Write the image
            track_image.save(format!("{}/track{}.png", output_path, track_index))?;
        }
        Ok(())
    }
}

impl fmt::Display for Song
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
        for track in self.tracks.iter()
        {
            if let Err(e) = write!(f, "Track:{}\n", track)
            {
                return Err(e);
            }
        }
        Ok(())
    }
}

/// Struct used for tracks
struct Track
{
    notes: Vec<&'static Note>
}

impl fmt::Display for Track
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
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

/// Struct used for individual notes
struct Note
{
    name: String,
    byte: u8,
    image: image::DynamicImage
}

impl Note
{
    fn new(byte: u8, image_path: String) -> Note
    {
        let image = image::open(&image_path).expect(&format!("Failed to read {}", image_path));
        let name = image_path[image_path.rfind('/').unwrap() + 1 .. image_path.len() - 4].to_string();
        Note { name, byte, image }
    }

    /// Access a note via it's midi byte index.
    fn get(index: u7) -> Option<&'static Note>
    {
        match index.as_int()
        {
            i if i == Ab2.byte => Some(&Ab2),
            i if i == A2.byte => Some(&A2),
            i if i == Bb2.byte => Some(&Bb2),
            i if i == B2.byte => Some(&B2),
            i if i == C3.byte => Some(&C3),
            i if i == Db3.byte => Some(&Db3),
            i if i == D3.byte => Some(&D3), 
            i if i == Eb3.byte => Some(&Eb3), 
            i if i == E3.byte => Some(&E3),
            i if i == F3.byte => Some(&F3),
            i if i == Gb3.byte => Some(&Gb3),
            i if i == G3.byte => Some(&G3),
            i if i == Ab3.byte => Some(&Ab3),
            i if i == A3.byte => Some(&A3),
            i if i == Bb3.byte => Some(&Bb3),
            i if i == B3.byte => Some(&B3),
            i if i == C4.byte => Some(&C4),
            i if i == Db4.byte => Some(&Db4),
            i if i == D4.byte => Some(&D4),
            i if i == Eb4.byte => Some(&Eb4),
            i if i == E4.byte => Some(&E4),
            i if i == F4.byte => Some(&F4),
            i if i == Gb4.byte => Some(&Gb4),
            i if i == G4.byte => Some(&G4),
            i if i == Ab4.byte => Some(&Ab4),
            i if i == A4.byte => Some(&A4),
            i if i == Bb4.byte => Some(&Bb4),
            i if i == B4.byte => Some(&B4),
            i if i == C5.byte => Some(&C5),
            i if i == Db5.byte => Some(&Db5),
            i if i == D5.byte => Some(&D5),
            i if i == Eb5.byte => Some(&Eb5),
            _ => None
        }
    }
}

impl fmt::Display for Note
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
        write!(f, "{}", self.name)
    }
}