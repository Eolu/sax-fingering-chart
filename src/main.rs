#![allow(non_upper_case_globals)]

#[macro_use]
extern crate lazy_static;

use clap::{Arg, App, ArgMatches};
use std::{fs, fmt::Display, collections::HashSet};
use midly::{Smf, EventKind::*, MidiMessage::*, number::u7};
use image::{DynamicImage, GenericImageView, GenericImage, error::ImageError};

#[derive(Copy, Clone)]
pub enum Transposition
{
    C = 0,
    Bb = 2,
    Eb = -3
}

// Note constants
define_notes!
{
    Bb2, 46, 
    B2, 47, 
    C3, 48,  
    Db3, 49,  
    D3, 50,  
    Eb3, 51,  
    E3, 52,  
    F3, 53,  
    Gb3, 54,  
    G3, 55,  
    Ab3, 56,  
    A3, 57,  
    Bb3, 58,  
    B3, 59, 
    C4, 60,  
    Db4, 61,  
    D4, 62,  
    Eb4, 63,  
    E4, 64,  
    F4, 65,  
    Gb4, 66,  
    G4, 67,  
    Ab4, 68,  
    A4, 69,  
    Bb4, 70,  
    B4, 71, 
    C5, 72,  
    Db5, 73,  
    D5, 74,  
    Eb5, 75,
    E5, 76,
    F5, 77
    // Altissimo from here beyond: 
    // Gb5, 78
}

// Load args TODO: Consider using a config file instead of command line args
lazy_static!
{
    static ref ARGS: ArgMatches<'static> = App::new("Saxophone Fingering Chart Generator")
        .version("0.1")
        .author("Griffin O'Neill <gsoneill1003@gmail.com>")
        .arg(Arg::with_name("midi_file")
            .help("Sets the source midi file to use in chart generation")
            .index(1)
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("transposition")
            .help("Sets the transposition to use.")
            .short("t")
            .long("trans")
            .possible_values(&["C", "Bb", "Eb"])
            .default_value("Bb")
            .takes_value(true))
        .arg(Arg::with_name("spacing")
            .help("Sets the visual spacing between fingering cha=rts")
            .short("s")
            .long("spacing")
            .default_value("10")
            .takes_value(true))
        .arg(Arg::with_name("source_charts")
            .help("Sets the directory containing the chart source files.")
            .short("c")
            .long("charts")
            .default_value("./fingerings")
            .takes_value(true))
        .arg(Arg::with_name("notes_per_row")
            .help("Sets the number of fingering charts per row")
            .short("n")
            .long("cols")
            .default_value("14")
            .takes_value(true))
        .arg(Arg::with_name("output_format")
            .help("Sets the output format. May output each fingering chart as an individual file, or as rows of a set length, or each entire tracks as a single file.")
            .short("f")
            .long("format")
            .possible_values(&["separate", "rows", "tracks"])
            .default_value("tracks")
            .takes_value(true))
        .arg(Arg::with_name("output_path")
            .help("Sets the output path of generated chart files")
            .short("o")
            .long("output")
            .default_value("./out")
            .takes_value(true))
        .get_matches();
}

/// Entry-point
fn main() -> Result<(), ImageError>
{
    match ARGS.value_of("midi_file")
    {
        Some(midi_path) => 
        {
            let transposition = Transposition::from(ARGS.value_of("transposition").unwrap());
            let output_format = ARGS.value_of("output_format").unwrap();
            let output_path: &str = ARGS.value_of("output_path").unwrap();
            let fingering_chart = Song::load(midi_path, &transposition);
            if output_format == "separate"
            {
                fingering_chart.output_cells(output_path)?
            }
            else
            {
                let notes_per_row: usize = ARGS.value_of("notes_per_row").unwrap().parse().unwrap();
                let spacing: usize = ARGS.value_of("spacing").unwrap().parse().unwrap();
                if output_format == "rows"
                {
                    fingering_chart.output_rows(output_path, notes_per_row, spacing)?
                }
                else
                {
                    fingering_chart.output_entire(output_path, notes_per_row, spacing)?
                }
            }
            // println!("{}", fingering_chart)
        },
        None => 
        {
            eprintln!("Must specify a midi file path.")
        }
    }
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
    pub fn load(midi_path: &str, transposition: &Transposition) -> Song
    {
        // Parse a midi file
        let raw_data = fs::read(midi_path).expect("Failed to load midi data");
        let midi = Smf::parse(&raw_data).expect("Failed to parse midi data");

        // Keep track of notes
        let mut notes: HashSet<u8> = HashSet::new();

        // Iterate through the midi file and collect notes
        Song {tracks: midi.tracks
            .iter()
            .map(|track| track
                .iter()
                .filter_map(|event| if let Midi { channel: _, message: NoteOn {key, vel: _} } = event.kind { Note::get(key, transposition, &mut notes) } else { None })
                .collect::<Vec<&Note>>())
            .filter_map(|notes| if notes.is_empty() { None } else { Some(Track{notes}) })
            .collect::<Vec<Track>>()}
    }

    /// Output chart with each cell as an individual file
    pub fn output_cells(&self, output_path: &str) -> Result<(), ImageError>
    {
        for track_index in 0..self.tracks.len()
        {
            let track_path = format!("{}/track{}", output_path, track_index);
            fs::create_dir_all(&track_path)?;
            for (cell, note) in self.tracks[track_index].notes.iter().enumerate() 
            {
                note.image.save(format!("{}/{}.png", track_path, cell))?;
            }
        }
        Ok(())
    }

    /// Output chart with each row as an individual file
    pub fn output_rows(&self, output_path: &str, notes_per_row: usize, spacing: usize) -> Result<(), ImageError>
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
    pub fn output_entire(&self, output_path: &str, notes_per_row: usize, spacing: usize) -> Result<(), ImageError>
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
        (0..self.notes.len())
            .take_while(|row| row * notes_per_row < self.notes.len())
            .map(|row| 
            {
                let mut row_image: DynamicImage = DynamicImage::new_rgb8(spacing as u32, 0);
                for note in (0..notes_per_row)
                    .take_while(|col| col + (row * notes_per_row) < self.notes.len())
                    .map(|col| self.notes[(row * notes_per_row) + col])
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
    pub fn cell_images(&self) -> impl Iterator<Item = &DynamicImage>
    {
        self.notes.iter().map(|note| &note.image)
    }
}

/// Macro to load notes using lazy_static
#[macro_export]
macro_rules! define_notes
{
    ($($name: ident, $num: literal),*) => 
    {
        lazy_static!
        {
            $( static ref $name: Note = Note::new($num, format!("{}/{}.png", ARGS.value_of("source_charts").unwrap(), stringify!($name))) );*;
        }

        impl Note
        {
            fn new(byte: u8, image_path: String) -> Note
            {
                let image = image::open(&image_path).expect(&format!("Failed to read {}", image_path));
                let name = image_path[image_path.rfind('/').expect("Bad path") + 1 .. image_path.len() - 4].to_string();
                Note { name, byte, image }
            }

            /// Access a note via it's midi byte index.
            fn get(index: u7, transposition: &Transposition, notes: &mut HashSet<u8>) -> Option<&'static Note>
            {
                match (index.as_int() + *transposition as u8)
                {
                    $(i if i == $name.byte => Some(&$name)),*,
                    i => 
                    {
                        if notes.insert(i)
                        {
                            eprintln!("Note out of range: {}", i);
                        }
                        None
                    }
                }
            }
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
            if let Err(e) = write!(f, "Track: {}", track)
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

// Gets transposition from a string
impl From<&str> for Transposition 
{
    fn from(string: &str) -> Self 
    {
        match string
        {
            "Bb" => Transposition::Bb,
            "Eb" => Transposition::Eb,
            _ => Transposition::C
        }
    }
}