use Key::*;
use enumset::*;
use std::collections::HashMap;
use crate::{define_notes, CONFIG};

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

#[derive(EnumSetType, Debug)]
pub enum Key
{
    Octave,
    Left1,
    Left2,
    Left3,
    FrontF,
    Bis,
    PalmD,
    PalmEflat,
    PalmF,
    Gsharp,
    LowCsharp,
    LowB,
    LowBflat,
    Right1,
    Right2,
    Right3,
    Fsharp,
    SideE,
    SideC,
    SideBis,
    HighF,
    LowEflat,
    LowC,
    // Baritone only, not yet supported
    LowA
}

// TODO: Consider generating key images at runtime,
// as at this point we have all the info we need to
// make it happen! Just store images of the visual
// representation of individual keys.

// Note constants are defined in a separate file
include!("keys.cfg");

/// Macro to define notes and load note images using lazy_static
#[macro_export]
macro_rules! define_notes
{
    (
        $(
            $name:literal $num: literal
            {
                $($key: ident),*
            }
        )*
    ) => 
    {
        lazy_static!
        {
            /// Map of loaded note constants
            pub static ref NOTES: HashMap<u8, Note> = 
            {
                let mut notes: HashMap<u8, Note> = HashMap::new();
                $(
                    let fingering = Fingering::new(enum_set!($($key)|*), format!("{}/{}", CONFIG.as_ref().unwrap().source_charts, $name));
                    match notes.get_mut(&$num)
                    {
                        None => 
                        {
                            notes.insert($num, Note { byte: $num, fingerings: vec!(fingering) });
                        },
                        Some(note) => 
                        {
                            note.fingerings.push(fingering);
                        }
                    };
                )*
                notes
            };
        }
    }
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
    pub fn new(keys: EnumSet<Key>, image_path: String) -> Fingering
    {
        Fingering { keys, image: image::open(&image_path).expect(&format!("Failed to read {}", image_path)) }
    }
    
}