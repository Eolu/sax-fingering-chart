use super::Key;
use crate::note::Note;
use image::{DynamicImage, Rgba};

// Various color constants used in chart generation
pub const BLACK: Rgba::<u8> = Rgba([0,0,0,255]);
pub const OCTAVE_2_COLOR: Rgba::<u8> = Rgba([164,120,59,255]);
pub const OCTAVE_3_COLOR: Rgba::<u8> = Rgba([97,90,199,255]);
pub const OCTAVE_4_COLOR: Rgba::<u8> = Rgba([184,94,191,255]);
pub const OCTAVE_5_COLOR: Rgba::<u8> = Rgba([76,158,91,255]);

/// The size of charts, in pixels. The actual charts output may multiply this by a whole-number factor
pub const CHART_SIZE: (u8,u8) = (63, 118);
/// The pixel location of the note-name on the chart
pub const NAME_LOCATION: (u8,u8) = (41,104);
/// The pixel location of the flat symbol that goes next to the note-name
pub const FLAT_LOCATION: (u8,u8) = (51,104);
/// The pixel location of horizontal separator between front keys
pub const SEP_LOCATION: (u8,u8) = (20,65);

/// This module is a container for literal raw png image data
mod raw_image_data
{
    include!(concat!(env!("OUT_DIR"), "/raw_image_data.rs"));
}

// Generated images.
lazy_static!
{
    pub static ref SEPARATOR: DynamicImage = load_image(raw_image_data::SEPARATOR);
    pub static ref A: DynamicImage = load_image(raw_image_data::A);
    pub static ref B: DynamicImage = load_image(raw_image_data::B);
    pub static ref C: DynamicImage = load_image(raw_image_data::C);
    pub static ref D: DynamicImage = load_image(raw_image_data::D);
    pub static ref E: DynamicImage = load_image(raw_image_data::E);
    pub static ref F: DynamicImage = load_image(raw_image_data::F);
    pub static ref G: DynamicImage = load_image(raw_image_data::G);
    pub static ref FLAT: DynamicImage = load_image(raw_image_data::FLAT);
    pub static ref FRONT_KEY: DynamicImage = load_image(raw_image_data::FRONT_KEY);
    pub static ref FRONT_KEY_OFF: DynamicImage = load_image(raw_image_data::FRONT_KEY_OFF);
    pub static ref FRONT_F_KEY: DynamicImage = load_image(raw_image_data::FRONT_F_KEY);
    pub static ref FRONT_F_KEY_OFF: DynamicImage = load_image(raw_image_data::FRONT_F_KEY_OFF);
    pub static ref PALM_KEY: DynamicImage = load_image(raw_image_data::PALM_KEY);
    pub static ref PALM_KEY_OFF: DynamicImage = load_image(raw_image_data::PALM_KEY_OFF);
    pub static ref OCTAVE_KEY: DynamicImage = load_image(raw_image_data::OCTAVE_KEY);
    pub static ref OCTAVE_KEY_OFF: DynamicImage = load_image(raw_image_data::OCTAVE_KEY_OFF);
    pub static ref BIS_KEY: DynamicImage = load_image(raw_image_data::BIS_KEY);
    pub static ref BIS_KEY_OFF: DynamicImage = load_image(raw_image_data::BIS_KEY_OFF);
    pub static ref LOW_A_KEY: DynamicImage = load_image(raw_image_data::LOW_A_KEY);
    pub static ref LOW_A_KEY_OFF: DynamicImage = load_image(raw_image_data::LOW_A_KEY_OFF);
    pub static ref SIDE_KEY: DynamicImage = load_image(raw_image_data::SIDE_KEY);
    pub static ref SIDE_KEY_OFF: DynamicImage = load_image(raw_image_data::SIDE_KEY_OFF);
    pub static ref HIGH_F_SHARP_KEY: DynamicImage = load_image(raw_image_data::HIGH_F_SHARP_KEY);
    pub static ref HIGH_F_SHARP_KEY_OFF: DynamicImage = load_image(raw_image_data::HIGH_F_SHARP_KEY_OFF);
    pub static ref F_SHARP_KEY: DynamicImage = load_image(raw_image_data::F_SHARP_KEY);
    pub static ref F_SHARP_KEY_OFF: DynamicImage = load_image(raw_image_data::F_SHARP_KEY_OFF);
    pub static ref LOW_E_FLAT_KEY: DynamicImage = load_image(raw_image_data::LOW_E_FLAT_KEY);
    pub static ref LOW_E_FLAT_KEY_OFF: DynamicImage = load_image(raw_image_data::LOW_E_FLAT_KEY_OFF);
    pub static ref LOW_C_KEY: DynamicImage = load_image(raw_image_data::LOW_C_KEY);
    pub static ref LOW_C_KEY_OFF: DynamicImage = load_image(raw_image_data::LOW_C_KEY_OFF);
    pub static ref G_SHARP_KEY: DynamicImage = load_image(raw_image_data::G_SHARP_KEY);
    pub static ref G_SHARP_KEY_OFF: DynamicImage = load_image(raw_image_data::G_SHARP_KEY_OFF);
    pub static ref SMALL_PINKY_KEY: DynamicImage = load_image(raw_image_data::SMALL_PINKY_KEY);
    pub static ref SMALL_PINKY_KEY_OFF: DynamicImage = load_image(raw_image_data::SMALL_PINKY_KEY_OFF);
    pub static ref LOW_B_FLAT_KEY: DynamicImage = load_image(raw_image_data::LOW_B_FLAT_KEY);
    pub static ref LOW_B_FLAT_KEY_OFF: DynamicImage = load_image(raw_image_data::LOW_B_FLAT_KEY_OFF);
}

impl Note
{
    /// Gets the pixel location of a keyname on the fingering chart and the image that goes there
    pub fn get_image_data<'a>(byte: u8) -> (Rgba::<u8>, &'a DynamicImage, Option<&'a DynamicImage>)
    {
        let (name, flat) = match byte % 12
        {
            0 => (&*C, None),
            1 => (&*C, Some(&*FLAT)),
            2 => (&*D, None),
            3 => (&*D, Some(&*FLAT)),
            4 => (&*E, None),
            5 => (&*F, None),
            6 => (&*F, Some(&*FLAT)),
            7 => (&*G, None),
            8 => (&*G, Some(&*FLAT)),
            9 => (&*A, None),
            10 => (&*A, Some(&*FLAT)),
            11 => (&*B, None),
            n => panic!("Modulus returned impossible number: {}", n)
        };
        let color = match byte
        {
            n if n < 48 => OCTAVE_2_COLOR,
            n if n < 60 => OCTAVE_3_COLOR,
            n if n < 72 => OCTAVE_4_COLOR,
            _ => OCTAVE_5_COLOR
        };
        return (color, name, flat);
    }
}

impl Key
{
    /// Gets the pixel location of a key on the fingering chart and the image that goes there
    pub fn get_image_data(&self) -> (u8, u8, &'static DynamicImage, &'static DynamicImage)
    {
        match *self
        {
            Self::Octave => (1,12,&OCTAVE_KEY,&OCTAVE_KEY_OFF),
            Self::Left1 => (22,23,&FRONT_KEY,&FRONT_KEY_OFF),
            Self::Left2 => (22,38,&FRONT_KEY,&FRONT_KEY_OFF),
            Self::Left3 => (22,53,&FRONT_KEY,&FRONT_KEY_OFF),
            Self::FrontF => (22,15,&FRONT_F_KEY,&FRONT_F_KEY_OFF),
            Self::Bis => (29,31,&BIS_KEY,&BIS_KEY_OFF),
            Self::PalmD => (53,10,&PALM_KEY,&PALM_KEY_OFF),
            Self::PalmEflat => (44,1,&PALM_KEY,&PALM_KEY_OFF),
            Self::PalmF => (37,14,&PALM_KEY,&PALM_KEY_OFF),
            Self::Gsharp => (43,61,&G_SHARP_KEY,&G_SHARP_KEY_OFF),
            Self::LowCsharp => (51,69,&SMALL_PINKY_KEY,&SMALL_PINKY_KEY_OFF),
            Self::LowB => (43,69,&SMALL_PINKY_KEY,&SMALL_PINKY_KEY_OFF),
            Self::LowBflat => (43,75,&LOW_B_FLAT_KEY,&LOW_B_FLAT_KEY_OFF),
            Self::Right1 => (22,68,&FRONT_KEY,&FRONT_KEY_OFF),
            Self::Right2 => (22,83,&FRONT_KEY,&FRONT_KEY_OFF),
            Self::Right3 => (22,98,&FRONT_KEY,&FRONT_KEY_OFF),
            Self::Fsharp => (9,93,&F_SHARP_KEY,&F_SHARP_KEY_OFF),
            Self::SideE => (7,62,&SIDE_KEY,&SIDE_KEY_OFF),
            Self::SideC => (7,71,&SIDE_KEY,&SIDE_KEY_OFF),
            Self::SideBis => (7,80,&SIDE_KEY,&SIDE_KEY_OFF),
            Self::HighFsharp => (12,84,&HIGH_F_SHARP_KEY,&HIGH_F_SHARP_KEY_OFF),
            Self::LowEflat => (4,105,&LOW_E_FLAT_KEY,&LOW_E_FLAT_KEY_OFF),
            Self::LowC => (4,110,&LOW_C_KEY,&LOW_C_KEY_OFF),
            Self::LowA => (3,34,&LOW_A_KEY,&LOW_A_KEY_OFF)
        }
    }
}

/// Load image, if this fails we might as well panic
fn load_image(raw_image: &[u8]) -> DynamicImage
{
    image::load_from_memory_with_format(raw_image, image::ImageFormat::Png)
        .expect("Failed to generate image data")
}
