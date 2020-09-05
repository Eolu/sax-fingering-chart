use enumset::EnumSet;
use std::collections::HashMap;
use crate::keys::Key;
use crate::CONFIG;

/// Type alias for note constants
pub type NoteConst = &'static Note;

/// Struct used for individual notes
pub struct Note
{
    pub byte: u8,
    pub fingerings: Vec<Fingering>
}

/// Struct used for individual fingering charts, may be multiple per note
pub struct Fingering
{
    pub keys: EnumSet<Key>,
    pub image: image::DynamicImage
}

// Define notes and load note images using lazy_static
lazy_static!
{
    /// Map of loaded note constants
    pub static ref NOTES: HashMap<u8, Note> = 
    {
        match &*CONFIG
        {
            Ok(config) => 
            {
                let mut notes: HashMap<u8, Note> = HashMap::new();
                for (byte, fingerings) in &config.notes
                {
                    let fingerings = fingerings
                        .into_iter()
                        .map(|&enumset| Fingering::new(enumset, *byte))
                        .collect();
                    match notes.get_mut(&byte)
                    {
                        None => 
                        {
                            notes.insert(*byte, Note { byte: *byte, fingerings });
                        },
                        Some(note) => 
                        {
                            note.fingerings.extend(fingerings);
                        }
                    };
                }
                notes
            },
            Err(e) => 
            {
                eprintln!("Failed to load config: {}", e);
                HashMap::new()
            }
        }
    };
}

impl Note
{
    /// Access a note via it's midi byte index.
    pub fn get(byte: u8) -> Option<NoteConst>
    {
        NOTES.get(&byte)
    }
}

impl Fingering
{
    /// Fingering contructor
    pub fn new(keys: EnumSet<Key>, byte: u8) -> Fingering
    {
        let mut image = Fingering::gen_chart(keys);
        Note::include_note_name(byte, &mut image);
        Fingering { keys, image }
    }
}