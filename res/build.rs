use std::path::Path;
use const_gen::*;

#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() 
{
    let mut res = winres::WindowsResource::new();
    res.set_icon("res/icon.ico");
    res.compile().unwrap();
    build();
}

#[cfg(unix)]
fn main() 
{
    build();
}

macro_rules! load_images
{
    ($($name:ident),*) => 
    {
        vec!
        [
            $(
                const_declaration!(pub $name = load_image(concat!(stringify!($name), ".png")))
            ),*
        ].join("\n")
    }
}

fn build()
{
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("raw_image_data.rs");

    let const_decs = load_images!
    {
        SEPARATOR, A, B, C, D, E, F, G, FLAT, FRONT_KEY, FRONT_KEY_OFF,
        FRONT_F_KEY, FRONT_F_KEY_OFF,PALM_KEY, PALM_KEY_OFF, OCTAVE_KEY, 
        OCTAVE_KEY_OFF, BIS_KEY, BIS_KEY_OFF, LOW_A_KEY, LOW_A_KEY_OFF, 
        SIDE_KEY, SIDE_KEY_OFF, HIGH_F_SHARP_KEY, HIGH_F_SHARP_KEY_OFF, 
        F_SHARP_KEY, F_SHARP_KEY_OFF, LOW_E_FLAT_KEY, LOW_E_FLAT_KEY_OFF, 
        LOW_C_KEY, LOW_C_KEY_OFF, G_SHARP_KEY, G_SHARP_KEY_OFF, SMALL_PINKY_KEY, 
        SMALL_PINKY_KEY_OFF, LOW_B_FLAT_KEY, LOW_B_FLAT_KEY_OFF
    };

    std::fs::write(&dest_path, const_decs).unwrap();
}

fn load_image<'a>(name: &str) -> Vec<u8>
{
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    std::fs::read(Path::new(&root).join("image_data").join(name))
        .expect(&format!("Failed to read image file: {}", name))
}

