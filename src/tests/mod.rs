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
