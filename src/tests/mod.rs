const OUTPUT_DIR: &str = "test_out";

/// Prints out each note image
#[test]
fn output_notes()
{
    std::fs::create_dir_all(OUTPUT_DIR).expect("Failed to create test output dir");
    for note in (46..78).filter_map(crate::note::Note::get)
    {
        for fingering in &note.fingerings
        {
            let outfile = format!("{}/{}_{}.png", OUTPUT_DIR, note.byte, fingering.keys.as_u32());
            println!("Writing {:?} to {}", fingering.keys, outfile);
            fingering.image.save(outfile).expect("Failed to save note image");
        }
    };
}

/// Prints the bits that make up other images
#[test]
pub fn output_images()
{
    use crate::keys::image_data::*;
    use std::fs::*;

    std::fs::create_dir_all("./test_image_data").expect("Failed to create test output dir");

    macro_rules! write_images
    {
        ($($name:ident),*) => 
        {
            $(
                let mut file = File::create(concat!("test_image_data/", stringify!($name), ".png")).unwrap();
                $name.write_to(&mut file, image::ImageOutputFormat::Png).unwrap();
            )*
        }
    }

    write_images!
    (
        SEPARATOR, A, B, C, D, E, F, G, FLAT, FRONT_KEY, FRONT_KEY_OFF,
        FRONT_F_KEY, FRONT_F_KEY_OFF,PALM_KEY, PALM_KEY_OFF, OCTAVE_KEY, 
        OCTAVE_KEY_OFF, BIS_KEY, BIS_KEY_OFF, LOW_A_KEY, LOW_A_KEY_OFF, 
        SIDE_KEY, SIDE_KEY_OFF, HIGH_F_SHARP_KEY, HIGH_F_SHARP_KEY_OFF, 
        F_SHARP_KEY, F_SHARP_KEY_OFF, LOW_E_FLAT_KEY, LOW_E_FLAT_KEY_OFF, 
        LOW_C_KEY, LOW_C_KEY_OFF, G_SHARP_KEY, G_SHARP_KEY_OFF, SMALL_PINKY_KEY, 
        SMALL_PINKY_KEY_OFF, LOW_B_FLAT_KEY, LOW_B_FLAT_KEY_OFF
    );
}