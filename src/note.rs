use Key::*;
use crate::{define_notes, CONFIG};

pub use enumset::*;

/// Struct used for individual notes
pub struct Note
{
    pub byte: u8,
    pub keys: EnumSet<Key>,
    pub image: image::DynamicImage
}

#[derive(EnumSetType)]
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
include!("keys.in");

/// Macro to define notes and load note images using lazy_static
#[macro_export]
macro_rules! define_notes
{
    ($($name: ident, $num: literal, 
    {
        $($key: ident),*
    })*) => 
    {
        lazy_static!
        {
            $( pub static ref $name: Note = Note::new
                ( 
                    $num, 
                    enum_set!($($key)|*),
                    format!("{}/{}.png", CONFIG.as_ref().unwrap().source_charts, stringify!($name))
                ) 
            );*;

            /// Vec containing all loaded notes
            pub static ref NOTES: Vec<&'static Note> = vec!($(&$name),*);
        }
    }
}

impl Note
{
    /// Note constructor
    pub fn new(byte: u8, keys: EnumSet<Key>, image_path: String) -> Note
    {
        Note { byte, keys, image: image::open(&image_path).expect(&format!("Failed to read {}", image_path)) }
    }

    /// Access a note via it's midi byte index.
    pub fn get(key: u8) -> Vec<&'static Note>
    {
        NOTES.iter().map(|note|*note).filter(|note| note.byte == key).collect()
    }
}