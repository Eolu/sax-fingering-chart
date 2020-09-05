mod image_data;

use crate::note::{Note, Fingering};
use image_data::*;
use enumset::*;
use image::{DynamicImage, GenericImage, GenericImageView};
use serde::{Serialize, Deserialize};

#[derive(EnumSetType, Debug, Deserialize, Serialize)]
#[enumset(serialize_as_list)]
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
    HighFsharp,
    LowEflat,
    LowC,
    // Baritone only
    LowA
}

impl Fingering
{
    /// Fingering charts are generated here. Note names must be added after
    pub fn gen_chart(keys: EnumSet<Key>) -> DynamicImage
    {
        let mut chart = gen_base_chart();

        // Depending on which keys are visible, show more greyed-out keys
        if !keys.is_disjoint(Key::LowEflat | Key::LowC)
        {
            Key::LowEflat.include_key_image(false, &mut chart);
            Key::LowC.include_key_image(false, &mut chart);
        }
        if !keys.is_disjoint(Key::Gsharp | Key::LowCsharp | Key::LowB | Key::LowBflat)
        {
            Key::Gsharp.include_key_image(false, &mut chart);
            Key::LowCsharp.include_key_image(false, &mut chart);
            Key::LowB.include_key_image(false, &mut chart);
            Key::LowBflat.include_key_image(false, &mut chart);
        }
        if !keys.is_disjoint(Key::PalmD | Key::PalmEflat | Key::PalmF)
        {
            Key::PalmD.include_key_image(false, &mut chart);
            Key::PalmEflat.include_key_image(false, &mut chart);
            Key::PalmF.include_key_image(false, &mut chart);
        }
        if !keys.is_disjoint(Key::SideE | Key::SideC | Key::SideBis | Key::Fsharp | Key::HighFsharp)
        {
            Key::SideE.include_key_image(false, &mut chart);
            Key::SideC.include_key_image(false, &mut chart);
            Key::SideBis.include_key_image(false, &mut chart);
        }

        for key in keys
        {
            key.include_key_image(true, &mut chart);
        }
        chart
    }
}

impl Note
{
    pub fn include_note_name(byte: u8, image: &mut DynamicImage)
    {
        let (color, name, flat) = Note::get_image_data(byte);
        for (x, y, inner_color) in name.pixels()
        {
            if inner_color == BLACK
            {
                let x = x + NAME_LOCATION.0 as u32;
                let y = y + NAME_LOCATION.1 as u32;
                image.put_pixel(x, y, color)
            }
        }
        if let Some(flat) = flat
        {
            for (x, y, inner_color) in flat.pixels()
            {
                if inner_color == BLACK
                {
                    let x = x + FLAT_LOCATION.0 as u32;
                    let y = y + FLAT_LOCATION.1 as u32;
                    image.put_pixel(x, y, color)
                }
            }
        }
    }
}

impl Key
{
    /// Include this key image on a chart
    fn include_key_image(&self, filled: bool, image: &mut DynamicImage)
    {
        let (x, y, note_data, note_off_data) = self.get_image_data();
        image.copy_from(if filled {note_data} else {note_off_data}, x as u32, y as u32)
            .expect("Failed to generate key image");
    }
}

/// Generates a blank fingering chart
fn gen_base_chart() -> DynamicImage
{
    let mut image: DynamicImage = DynamicImage::new_rgba8(CHART_SIZE.0 as u32, CHART_SIZE.1 as u32);
    image.invert();

    // All charts start with 6 unfilled notes and a separator
    Key::Octave.include_key_image(false, &mut image);
    Key::Left1.include_key_image(false, &mut image);
    Key::Left2.include_key_image(false, &mut image);
    Key::Left3.include_key_image(false, &mut image);
    Key::Right1.include_key_image(false, &mut image);
    Key::Right2.include_key_image(false, &mut image);
    Key::Right3.include_key_image(false, &mut image);
    image.copy_from(&*SEPARATOR, SEP_LOCATION.0 as u32, SEP_LOCATION.1 as u32)
        .expect("Failed to generate separator image");
    image
}

