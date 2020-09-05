use super::Key;
use crate::note::Note;
use image::{DynamicImage, GenericImage, Rgba};
use std::cmp::max;

pub const BLACK: Rgba::<u8> = Rgba([0,0,0,255]);
pub const TRANSPARENT: Rgba::<u8> = Rgba([255,255,255,0]);
pub const GREY: Rgba::<u8> = Rgba([128,128,128,255]);
pub const OCTAVE_2_COLOR: Rgba::<u8> = Rgba([164,120,59,255]);
pub const OCTAVE_3_COLOR: Rgba::<u8> = Rgba([97,90,199,255]);
pub const OCTAVE_4_COLOR: Rgba::<u8> = Rgba([184,94,191,255]);
pub const OCTAVE_5_COLOR: Rgba::<u8> = Rgba([76,158,91,255]);

pub const CHART_SIZE: (u8,u8) = (63, 118);
pub const NAME_LOCATION: (u8,u8) = (41,104);
pub const FLAT_LOCATION: (u8,u8) = (51,104);
pub const SEP_LOCATION: (u8,u8) = (20,65);

lazy_static!
{
    pub static ref SEPARATOR: DynamicImage = gen_name_image(&SEPARATOR_COORDS, GREY);
    static ref A: DynamicImage = gen_name_image(&A_COORDS, BLACK);
    static ref B: DynamicImage = gen_name_image(&B_COORDS, BLACK);
    static ref C: DynamicImage = gen_name_image(&C_COORDS, BLACK);
    static ref D: DynamicImage = gen_name_image(&D_COORDS, BLACK);
    static ref E: DynamicImage = gen_name_image(&E_COORDS, BLACK);
    static ref F: DynamicImage = gen_name_image(&F_COORDS, BLACK);
    static ref G: DynamicImage = gen_name_image(&G_COORDS, BLACK);
    static ref FLAT: DynamicImage = gen_name_image(&FLAT_COORDS, BLACK);
    static ref FRONT_KEY: DynamicImage = gen_key_image(&FRONT_KEY_OUTSIDE_COORDS, &FRONT_KEY_BORDER_COORDS, true);
    static ref FRONT_KEY_OFF: DynamicImage = gen_key_image(&FRONT_KEY_OUTSIDE_COORDS, &FRONT_KEY_BORDER_COORDS, false);
    static ref FRONT_F_KEY: DynamicImage = gen_key_image(&FRONT_F_KEY_OUTSIDE_COORDS, &FRONT_F_KEY_BORDER_COORDS, true);
    static ref FRONT_F_KEY_OFF: DynamicImage = gen_key_image(&FRONT_F_KEY_OUTSIDE_COORDS, &FRONT_F_KEY_BORDER_COORDS, false);
    static ref PALM_KEY: DynamicImage = gen_key_image(&PALM_KEY_OUTSIDE_COORDS, &PALM_KEY_BORDER_COORDS, true);
    static ref PALM_KEY_OFF: DynamicImage = gen_key_image(&PALM_KEY_OUTSIDE_COORDS, &PALM_KEY_BORDER_COORDS, false);
    static ref OCTAVE_KEY: DynamicImage = gen_key_image(&OCTAVE_KEY_OUTSIDE_COORDS, &OCTAVE_KEY_BORDER_COORDS, true);
    static ref OCTAVE_KEY_OFF: DynamicImage = gen_key_image(&OCTAVE_KEY_OUTSIDE_COORDS, &OCTAVE_KEY_BORDER_COORDS, false);
    static ref BIS_KEY: DynamicImage = gen_key_image(&BIS_OUTSIDE_COORDS, &BIS_BORDER_COORDS, true);
    static ref BIS_KEY_OFF: DynamicImage = gen_key_image(&BIS_OUTSIDE_COORDS, &BIS_BORDER_COORDS, false);
    static ref LOW_A_KEY: DynamicImage = gen_key_image(&LOW_A_OUTSIDE_COORDS, &LOW_A_BORDER_COORDS, true);
    static ref LOW_A_KEY_OFF: DynamicImage = gen_key_image(&LOW_A_OUTSIDE_COORDS, &LOW_A_BORDER_COORDS, false);
    static ref SIDE_KEY: DynamicImage = gen_key_image(&SIDE_OUTSIDE_COORDS, &SIDE_BORDER_COORDS, true);
    static ref SIDE_KEY_OFF: DynamicImage = gen_key_image(&SIDE_OUTSIDE_COORDS, &SIDE_BORDER_COORDS, false);
    static ref HIGH_F_SHARP_KEY: DynamicImage = gen_key_image(&HIGH_F_SHARP_OUTSIDE_COORDS, &HIGH_F_SHARP_BORDER_COORDS, true);
    static ref HIGH_F_SHARP_KEY_OFF: DynamicImage = gen_key_image(&HIGH_F_SHARP_OUTSIDE_COORDS, &HIGH_F_SHARP_BORDER_COORDS, false);
    static ref F_SHARP_KEY: DynamicImage = gen_key_image(&F_SHARP_OUTSIDE_COORDS, &F_SHARP_BORDER_COORDS, true);
    static ref F_SHARP_KEY_OFF: DynamicImage = gen_key_image(&F_SHARP_OUTSIDE_COORDS, &F_SHARP_BORDER_COORDS, false);
    static ref LOW_E_FLAT_KEY: DynamicImage = gen_key_image(&LOW_E_FLAT_OUTSIDE_COORDS, &LOW_E_FLAT_BORDER_COORDS, true);
    static ref LOW_E_FLAT_KEY_OFF: DynamicImage = gen_key_image(&LOW_E_FLAT_OUTSIDE_COORDS, &LOW_E_FLAT_BORDER_COORDS, false);
    static ref LOW_C_KEY: DynamicImage = gen_key_image(&LOW_C_OUTSIDE_COORDS, &LOW_C_BORDER_COORDS, true);
    static ref LOW_C_KEY_OFF: DynamicImage = gen_key_image(&LOW_C_OUTSIDE_COORDS, &LOW_C_BORDER_COORDS, false);
    static ref G_SHARP_KEY: DynamicImage = gen_key_image(&G_SHARP_OUTSIDE_COORDS, &G_SHARP_BORDER_COORDS, true);
    static ref G_SHARP_KEY_OFF: DynamicImage = gen_key_image(&G_SHARP_OUTSIDE_COORDS, &G_SHARP_BORDER_COORDS, false);
    static ref SMALL_PINKY_KEY: DynamicImage = gen_key_image(&SMALL_PINKY_KEY_OUTSIDE_COORDS, &SMALL_PINKY_KEY_BORDER_COORDS, true);
    static ref SMALL_PINKY_KEY_OFF: DynamicImage = gen_key_image(&SMALL_PINKY_KEY_OUTSIDE_COORDS, &SMALL_PINKY_KEY_BORDER_COORDS, false);
    static ref LOW_B_FLAT_KEY: DynamicImage = gen_key_image(&LOW_B_FLAT_OUTSIDE_COORDS, &LOW_B_FLAT_BORDER_COORDS, true);
    static ref LOW_B_FLAT_KEY_OFF: DynamicImage = gen_key_image(&LOW_B_FLAT_OUTSIDE_COORDS, &LOW_B_FLAT_BORDER_COORDS, false);
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

/// Generates a key image
fn gen_key_image(outside_coords: &[(u32, u32)], border_coords: &[(u32, u32)], filled: bool) -> DynamicImage
{
    let (width, height) = border_coords.iter().fold((0,0), |(x,y), (cur_x, cur_y)| (max(x, 1 + *cur_x), max(y, 1 + *cur_y)));
    let mut image: DynamicImage = DynamicImage::new_rgba8(width, height);
    for x in 0..width
    {
        for y in 0..height
        {
            if border_coords.contains(&(x,y))
            {
                image.put_pixel(x, y, if filled {BLACK} else {GREY})
            }
            else if filled && !outside_coords.contains(&(x,y))
            {
                image.put_pixel(x, y, BLACK)
            }
            else
            {
                image.put_pixel(x, y, TRANSPARENT)
            }
        }
    }
    image
}

/// Generates a key name image
fn gen_name_image(coords: &[(u32, u32)], color: Rgba<u8>) -> DynamicImage
{
    let (width, height) = coords.iter().fold((0,0), |(x,y), (cur_x, cur_y)| (max(x, 1 + *cur_x), max(y, 1 + *cur_y)));
    let mut image: DynamicImage = DynamicImage::new_rgba8(width, height);
    for x in 0..width
    {
        for y in 0..height
        {
            if coords.contains(&(x,y))
            {
                image.put_pixel(x, y, color)
            }
            else
            {
                image.put_pixel(x, y, TRANSPARENT)
            }
        }
    }
    image
}

// These constants are used to generate actual images of each element of the fingering chart

const FRONT_KEY_OUTSIDE_COORDS: [(u32, u32); 24] =
[
    (0,0), (1,0), (2,0),                             (7,0), (8,0), (9,0),
    (0,1), (1,1),                                           (8,1), (9,1),
    (0,2),                                                         (9,2),
    (0,7),                                                         (9,7),
    (0,8), (1,8),                                           (8,8), (9,8),
    (0,9), (1,9), (2,9),                             (7,9), (8,9), (9,9),
];
const FRONT_KEY_BORDER_COORDS: [(u32, u32); 24] =
[
                            (3,0), (4,0), (5,0), (6,0),
                    (2,1),                             (7,1),
            (1,2),                                           (8,2),
    (0,3),                                                         (9,3),
    (0,4),                                                         (9,4),
    (0,5),                                                         (9,5),
    (0,6),                                                         (9,6),
            (1,7),                                           (8,7),
                    (2,8),                             (7,8), 
                        (3,9), (4,9), (5,9), (6,9),
];
const FRONT_F_KEY_OUTSIDE_COORDS: [(u32, u32); 12] =
[
    (0,0), (1,0),                                           (8,0), (9,0),
    (0,1),                                                         (9,1),
    (0,3),                                                         (9,3),
    (0,4), (1,4),                                           (8,4), (9,4),
];
const FRONT_F_KEY_BORDER_COORDS: [(u32, u32); 18] =
[
                   (2,0), (3,0), (4,0), (5,0), (6,0), (7,0),
            (1,1),                                           (8,1),
    (0,2),                                                          (9,2),
            (1,3),                                           (8,3),
                   (2,4), (3,4), (4,4), (5,4), (6,4), (7,4),
];
const PALM_KEY_OUTSIDE_COORDS: [(u32, u32); 28] =
[
    (0,0),  (1,0),  (2,0),                           (5,0),  (6,0),  (7,0),
    (0,1),  (1,1),                                           (6,1),  (7,1),
    (0,2),                                                           (7,2),
    (0,3),                                                           (7,3),
    (0,11),                                                          (7,11),
    (0,12),                                                          (7,12),
    (0,13), (1,13),                                          (6,13), (7,13),
    (0,14), (1,14), (2,14),                          (5,14), (6,14), (7,14),
];
const PALM_KEY_BORDER_COORDS: [(u32, u32); 30] =
[
                           (3,0),  (4,0),
                   (2,1),                   (5,1),
            (1,2),                                 (6,2),
            (1,3),                                 (6,3),
    (0,4),                                                 (7,4),
    (0,5),                                                 (7,5),
    (0,6),                                                 (7,6),
    (0,7),                                                 (7,7),
    (0,8),                                                 (7,8),
    (0,9),                                                 (7,9),
    (0,10),                                                (7,10),
            (1,11),                                 (6,11),
            (1,12),                                 (6,12),
                    (2,13),                 (5,13), 
                            (3,14), (4,14),
];

// From here on out I got "lazy" and machine-generated the rest

const OCTAVE_KEY_OUTSIDE_COORDS: [(u32, u32); 230] =
[
    (1,0), (2,0), (3,0), (4,0), (5,0), (6,0), (7,0), (8,0), (9,0), (10,0), (11,0), (12,0), (13,0), 
    (1,1), (2,1), (3,1), (4,1), (5,1), (6,1), (7,1), (8,1), (9,1), (10,1), (11,1), (12,1), (13,1), 
    (1,2), (2,2), (3,2), (4,2), (5,2), (6,2), (7,2), (8,2), (9,2), (10,2), (11,2), (12,2), (13,2), 
    (1,3), (2,3), (3,3), (4,3), (5,3), (6,3), (7,3), (8,3), (9,3), (10,3), (11,3), (12,3), (13,3), 
    (1,4), (2,4), (3,4), (4,4), (5,4), (6,4), (7,4), (8,4), (9,4), (10,4), (11,4), (12,4), (13,4), 
    (1,5), (2,5), (3,5), (4,5), (5,5), (6,5), (8,5), (9,5), (10,5), (11,5), (12,5), (13,5), (1,6), 
    (2,6), (3,6), (4,6), (5,6), (6,6), (8,6), (9,6), (10,6), (11,6), (12,6), (13,6), (1,7), (2,7), 
    (3,7), (4,7), (5,7), (9,7), (10,7), (11,7), (12,7), (13,7), (1,8), (2,8), (3,8), (4,8), (5,8), 
    (9,8), (10,8), (11,8), (12,8), (13,8), (1,9), (2,9), (3,9), (4,9), (10,9), (11,9), (12,9), (13,9), 
    (1,10), (2,10), (3,10), (4,10), (10,10), (11,10), (12,10), (13,10), (1,11), (2,11), (3,11), (4,11), 
    (10,11), (11,11), (12,11), (13,11), (1,12), (2,12), (3,12), (11,12), (12,12), (13,12), (1,13), 
    (2,13), (3,13), (11,13), (12,13), (13,13), (1,14), (2,14), (3,14), (11,14), (12,14), (13,14), 
    (1,15), (2,15), (12,15), (13,15), (1,16), (2,16), (12,16), (13,16), (1,17), (2,17), (12,17), 
    (13,17), (1,18), (2,18), (12,18), (13,18), (1,19), (2,19), (3,19), (11,19), (12,19), (13,19), 
    (1,20), (2,20), (3,20), (4,20), (10,20), (11,20), (12,20), (13,20), (1,21), (2,21), (3,21), (4,21), 
    (5,21), (9,21), (10,21), (11,21), (12,21), (13,21), (1,22), (2,22), (3,22), (4,22), (5,22), (6,22), 
    (7,22), (8,22), (9,22), (10,22), (11,22), (12,22), (13,22), (1,23), (2,23), (3,23), (4,23), (5,23), 
    (6,23), (7,23), (8,23), (9,23), (10,23), (11,23), (12,23), (13,23), (1,24), (2,24), (3,24), (4,24), 
    (5,24), (6,24), (7,24), (8,24), (9,24), (10,24), (11,24), (12,24), (13,24)
];
const OCTAVE_KEY_BORDER_COORDS: [(u32, u32); 83] =
[
    (0,0), (14,0), (0,1), (14,1), (0,2), (14,2), (0,3), (14,3), (0,4), (14,4), (0,5), (7,5), (14,5), 
    (0,6), (7,6), (14,6), (0,7), (6,7), (8,7), (14,7), (0,8), (6,8), (8,8), (14,8), (0,9), (5,9), (9,9), 
    (14,9), (0,10), (5,10), (9,10), (14,10), (0,11), (5,11), (9,11), (14,11), (0,12), (4,12), (10,12), 
    (14,12), (0,13), (4,13), (10,13), (14,13), (0,14), (4,14), (10,14), (14,14), (0,15), (3,15), (11,15), 
    (14,15), (0,16), (3,16), (11,16), (14,16), (0,17), (3,17), (11,17), (14,17), (0,18), (3,18), (11,18), 
    (14,18), (0,19), (4,19), (10,19), (14,19), (0,20), (5,20), (9,20), (14,20), (0,21), (6,21), (7,21), 
    (8,21), (14,21), (0,22), (14,22), (0,23), (14,23), (0,24), (14,24)
];
const BIS_OUTSIDE_COORDS: [(u32, u32); 12] =
[

    (0,0), (1,0), (4,0), (5,0), (0,1), (5,1), (0,4), (5,4), (0,5), (1,5), (4,5), (5,5)
];
const BIS_BORDER_COORDS: [(u32, u32); 12] =
[

    (2,0), (3,0), (1,1), (4,1), (0,2), (5,2), (0,3), (5,3), (1,4), (4,4), (2,5), (3,5)
];
const LOW_A_OUTSIDE_COORDS: [(u32, u32); 9] =
[

    (2,0), (3,0), (4,0), (5,0), (6,0), (7,0), (8,0), (0,4), (10,4)
];
const LOW_A_BORDER_COORDS: [(u32, u32); 26] =
[

    (0,0), (1,0), (9,0), (10,0), (0,1), (2,1), (3,1), (4,1), (5,1), (6,1), (7,1), (8,1), (10,1), 
    (0,2), (10,2), (0,3), (10,3), (1,4), (2,4), (3,4), (4,4), (5,4), (6,4), (7,4), (8,4), (9,4)
];
const SIDE_OUTSIDE_COORDS: [(u32, u32); 12] =
[

    (0,0), (1,0), (3,0), (4,0), (0,1), (4,1), (0,8), (4,8), (0,9), (1,9), (3,9), (4,9)
];
const SIDE_BORDER_COORDS: [(u32, u32); 18] =
[

    (2,0), (1,1), (3,1), (0,2), (4,2), (0,3), (4,3), (0,4), (4,4), (0,5), (4,5), (0,6), (4,6), 
    (0,7), (4,7), (1,8), (3,8), (2,9)
];
const HIGH_F_SHARP_OUTSIDE_COORDS: [(u32, u32); 20] =
[
    (0,0), (1,0), (2,0), (5,0), (0,1), (1,1), (0,2), (1,2), (0,3), (1,3), (0,4), 
    (1,4), (0,5), (5,6), (4,7), (5,7), (0,8), (3,8), (4,8), (5,8)
];
const HIGH_F_SHARP_BORDER_COORDS: [(u32, u32); 18] =
[
    (3,0), (4,0), (2,1), (5,1), (2,2), (5,2), (2,3), (5,3), (2,4), (5,4), (1,5), 
    (5,5), (0,6), (4,6), (0,7), (3,7), (1,8), (2,8)
];
const F_SHARP_OUTSIDE_COORDS: [(u32, u32); 38] =
[
    (0,0), (4,0), (5,0), (6,0), (7,0), (8,0), (9,0), (10,0), (6,1), (7,1), (8,1), 
    (9,1), (10,1), (8,2), (9,2), (10,2), (0,3), (9,3), (10,3), (0,4), (1,4), (10,4), 
    (0,5), (1,5), (2,5), (0,6), (1,6), (2,6), (3,6), (4,6), (0,7), (1,7), (2,7), 
    (3,7), (4,7), (5,7), (6,7), (10,7)
];
const F_SHARP_BORDER_COORDS: [(u32, u32); 22] =
[
    (1,0), (2,0), (3,0), (0,1), (4,1), (5,1), (0,2), (6,2), (7,2), (1,3), (8,3), 
    (2,4), (9,4), (3,5), (4,5), (10,5), (5,6), (6,6), (10,6), (7,7), (8,7), (9,7)
];
const LOW_E_FLAT_OUTSIDE_COORDS: [(u32, u32); 16] =
[
    (0,0), (1,0), (2,0), (3,0), (4,0), (11,0), (12,0), (13,0), (14,0), (15,0), 
    (0,1), (1,1), (14,1), (15,1), (0,2), (15,2)
];
const LOW_E_FLAT_BORDER_COORDS: [(u32, u32); 34] =
[
    (5,0), (6,0), (7,0), (8,0), (9,0), (10,0), (2,1), (3,1), (4,1), (11,1), (12,1), 
    (13,1), (1,2), (14,2), (0,3), (15,3), (0,4), (15,4), (0,5), (1,5), (2,5), (3,5), 
    (4,5), (5,5), (6,5), (7,5), (8,5), (9,5), (10,5), (11,5), (12,5), (13,5), (14,5), 
    (15,5)
];
const LOW_C_OUTSIDE_COORDS: [(u32, u32); 16] =
[
    (0,3), (15,3), (0,4), (1,4), (14,4), (15,4), (0,5), (1,5), (2,5), (3,5), (4,5),
    (11,5), (12,5), (13,5), (14,5), (15,5)
];
const LOW_C_BORDER_COORDS: [(u32, u32); 34] =
[
    (0,0), (1,0), (2,0), (3,0), (4,0), (5,0), (6,0), (7,0), (8,0), (9,0), (10,0), 
    (11,0), (12,0), (13,0), (14,0), (15,0), (0,1), (15,1), (0,2), (15,2), (1,3), 
    (14,3), (2,4), (3,4), (4,4), (11,4), (12,4), (13,4), (5,5), (6,5), (7,5), (8,5), 
    (9,5), (10,5)
];
const G_SHARP_OUTSIDE_COORDS: [(u32, u32); 28] =
[
    (0,0), (1,0), (2,0), (3,0), (12,0), (13,0), (14,0), (15,0), (0,1), (1,1), 
    (14,1), (15,1), (0,2), (15,2), (0,5), (15,5), (0,6), (1,6), (14,6), (15,6), 
    (0,7), (1,7), (2,7), (3,7), (12,7), (13,7), (14,7), (15,7)
];
const G_SHARP_BORDER_COORDS: [(u32, u32); 32] =
[
    (4,0), (5,0), (6,0), (7,0), (8,0), (9,0), (10,0), (11,0), (2,1), (3,1), (12,1), 
    (13,1), (1,2), (14,2), (0,3), (15,3), (0,4), (15,4), (1,5), (14,5), (2,6), (3,6), 
    (12,6), (13,6), (4,7), (5,7), (6,7), (7,7), (8,7), (9,7), (10,7), (11,7)
];
const SMALL_PINKY_KEY_OUTSIDE_COORDS: [(u32, u32); 12] =
[
    (0,0), (1,0), (6,0), (7,0), (0,1), (7,1), (0,4), (7,4), (0,5), (1,5), (6,5), (7,5)
];
const SMALL_PINKY_KEY_BORDER_COORDS: [(u32, u32); 16] =
[
    (2,0), (3,0), (4,0), (5,0), (1,1), (6,1), (0,2), (7,2), (0,3), (7,3), (1,4), 
    (6,4), (2,5), (3,5), (4,5), (5,5)
];
const LOW_B_FLAT_OUTSIDE_COORDS: [(u32, u32); 12] =
[
    (0,0), (1,0), (14,0), (15,0), (0,1), (15,1), (0,5), (15,5), (0,6), (1,6), 
    (14,6), (15,6)
];
const LOW_B_FLAT_BORDER_COORDS: [(u32, u32); 34] =
[
    (2,0), (3,0), (4,0), (5,0), (6,0), (7,0), (8,0), (9,0), (10,0), (11,0), (12,0), 
    (13,0), (1,1), (14,1), (0,2), (15,2), (0,3), (15,3), (0,4), (15,4), (1,5), (14,5), 
    (2,6), (3,6), (4,6), (5,6), (6,6), (7,6), (8,6), (9,6), (10,6), (11,6), (12,6), (13,6)
];

const A_COORDS: [(u32, u32); 56] =
[
    (2,0), (3,0), (4,0), (5,0), (2,1), (3,1), (4,1), (5,1), (0,2), (1,2), (6,2), (7,2), 
    (0,3), (1,3), (6,3), (7,3), (0,4), (1,4), (6,4), (7,4), (0,5), (1,5), (6,5), (7,5), 
    (0,6), (1,6), (2,6), (3,6), (4,6), (5,6), (6,6), (7,6), (0,7), (1,7), (2,7), (3,7), 
    (4,7), (5,7), (6,7), (7,7), (0,8), (1,8), (6,8), (7,8), (0,9), (1,9), (6,9), (7,9), 
    (0,10), (1,10), (6,10), (7,10), (0,11), (1,11), (6,11), (7,11)
];
const B_COORDS: [(u32, u32); 60] =
[
    (0,0), (1,0), (2,0), (3,0), (4,0), (5,0), (0,1), (1,1), (2,1), (3,1), (4,1), (5,1), 
    (0,2), (1,2), (6,2), (7,2), (0,3), (1,3), (6,3), (7,3), (0,4), (1,4), (2,4), (3,4), 
    (4,4), (5,4), (0,5), (1,5), (2,5), (3,5), (4,5), (5,5), (0,6), (1,6), (6,6), (7,6), 
    (0,7), (1,7), (6,7), (7,7), (0,8), (1,8), (6,8), (7,8), (0,9), (1,9), (6,9), (7,9), 
    (0,10), (1,10), (2,10), (3,10), (4,10), (5,10), (0,11), (1,11), (2,11), (3,11), (4,11), 
    (5,11)
];
const C_COORDS: [(u32, u32); 40] =
[
    (2,0), (3,0), (4,0), (5,0), (6,0), (7,0), (2,1), (3,1), (4,1), (5,1), (6,1), (7,1), 
    (0,2), (1,2), (0,3), (1,3), (0,4), (1,4), (0,5), (1,5), (0,6), (1,6), (0,7), (1,7), 
    (0,8), (1,8), (0,9), (1,9), (2,10), (3,10), (4,10), (5,10), (6,10), (7,10), (2,11), 
    (3,11), (4,11), (5,11), (6,11), (7,11)
];
const D_COORDS: [(u32, u32); 56] =
[
    (0,0), (1,0), (2,0), (3,0), (4,0), (5,0), (0,1), (1,1), (2,1), (3,1), (4,1), (5,1), 
    (0,2), (1,2), (6,2), (7,2), (0,3), (1,3), (6,3), (7,3), (0,4), (1,4), (6,4), (7,4), 
    (0,5), (1,5), (6,5), (7,5), (0,6), (1,6), (6,6), (7,6), (0,7), (1,7), (6,7), (7,7), 
    (0,8), (1,8), (6,8), (7,8), (0,9), (1,9), (6,9), (7,9), (0,10), (1,10), (2,10), (3,10), 
    (4,10), (5,10), (0,11), (1,11), (2,11), (3,11), (4,11), (5,11)
];
const E_COORDS: [(u32, u32); 56] =
[
    (0,0), (1,0), (2,0), (3,0), (4,0), (5,0), (6,0), (7,0), (0,1), (1,1), (2,1), (3,1), 
    (4,1), (5,1), (6,1), (7,1), (0,2), (1,2), (0,3), (1,3), (0,4), (1,4), (2,4), (3,4), 
    (4,4), (5,4), (0,5), (1,5), (2,5), (3,5), (4,5), (5,5), (0,6), (1,6), (0,7), (1,7), 
    (0,8), (1,8), (0,9), (1,9), (0,10), (1,10), (2,10), (3,10), (4,10), (5,10), (6,10), 
    (7,10), (0,11), (1,11), (2,11), (3,11), (4,11), (5,11), (6,11), (7,11)
];
const F_COORDS: [(u32, u32); 44] =
[
    (0,0), (1,0), (2,0), (3,0), (4,0), (5,0), (6,0), (7,0), (0,1), (1,1), (2,1), (3,1), 
    (4,1), (5,1), (6,1), (7,1), (0,2), (1,2), (0,3), (1,3), (0,4), (1,4), (2,4), (3,4), 
    (4,4), (5,4), (0,5), (1,5), (2,5), (3,5), (4,5), (5,5), (0,6), (1,6), (0,7), (1,7), 
    (0,8), (1,8), (0,9), (1,9), (0,10), (1,10), (0,11), (1,11)
];
const G_COORDS: [(u32, u32); 44] =
[
    (0,0), (1,0), (2,0), (3,0), (4,0), (5,0), (6,0), (7,0), (0,1), (1,1), (2,1), (3,1), 
    (4,1), (5,1), (6,1), (7,1), (0,2), (1,2), (0,3), (1,3), (0,4), (1,4), (2,4), (3,4), 
    (4,4), (5,4), (0,5), (1,5), (2,5), (3,5), (4,5), (5,5), (0,6), (1,6), (0,7), (1,7), 
    (0,8), (1,8), (0,9), (1,9), (0,10), (1,10), (0,11), (1,11)
];
const FLAT_COORDS: [(u32, u32); 44] =
[
    (0,0), (1,0), (0,1), (1,1), (0,2), (1,2), (0,3), (1,3), (0,4), (1,4), (0,5), (1,5), 
    (0,6), (1,6), (2,6), (3,6), (4,6), (5,6), (0,7), (1,7), (2,7), (3,7), (4,7), (5,7), 
    (0,8), (1,8), (6,8), (7,8), (0,9), (1,9), (6,9), (7,9), (0,10), (1,10), (2,10), (3,10), 
    (4,10), (5,10), (0,11), (1,11), (2,11), (3,11), (4,11), (5,11)
];
const SEPARATOR_COORDS: [(u32, u32); 14] =
[
    (0,0), (1,0), (2,0), (3,0), (4,0), (5,0), (6,0), (7,0), (8,0), (9,0), (10,0), (11,0),
    (12,0), (13,0)
];
