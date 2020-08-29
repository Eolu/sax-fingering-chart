use crate::note::{NoteConst, Fingering};
use image::{DynamicImage, GenericImageView, GenericImage};

/// Struct used for tracks
pub struct Track(pub Vec<NoteConst>);

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
            track_image.copy_from(&previous, 0, 0).expect("Failed to copy track image");
            track_image.copy_from(&row_image, 0, previous.height()).expect("Failed to generate track image");
        }
        track_image
    }

    /// Generates images in rows
    pub fn row_images(&self, notes_per_row: usize, spacing: usize) -> Vec<DynamicImage>
    {
        let fingerings = self.select_fingerings();
        (0..fingerings.len())
            .take_while(|row| row * notes_per_row < self.notes().len())
            .map(|row|
            {
                let mut row_image: DynamicImage = DynamicImage::new_rgb8(spacing as u32, 0);
                for image in (0..notes_per_row)
                    .take_while(|col| col + (row * notes_per_row) < self.notes().len())
                    .map(|col| fingerings[(row * notes_per_row) + col])
                {
                    let previous = row_image;
                    let new_width = previous.width() + image.width() + spacing as u32;
                    let new_height = std::cmp::max(previous.height(), image.height());
                    row_image = DynamicImage::new_rgb8(new_width, new_height);
                    row_image.copy_from(&previous, 0, 0).expect("Failed to copy row image");
                    row_image.copy_from(image, previous.width(), 0).expect("Failed to generate row image");
                }
                row_image
            }).collect()
    }

    /// Returns an iterator into cell images. The images themselves are 
    /// generated at load-time, so this method is low-cost.
    pub fn cell_images(&self) -> Vec<&DynamicImage>
    {
        self.select_fingerings()
    }

    /// Access notes vector
    fn notes(&self) -> &Vec<NoteConst>
    {
        &self.0
    }

    /// Go through candidates and determine which fingerings to use
    fn select_fingerings(&self) -> Vec<&DynamicImage>
    {
        let mut previous_choice: Option<&Fingering> = None;
        self.notes()
            .iter()
            .enumerate()
            .map(|(i, notes)| (i, &notes.fingerings))
            .filter_map(|(i, fingerings)| 
            {
                // Determine how alike a note is from a list of candidates
                fn intersection<'a>(note: &Fingering, candidates: impl Iterator<Item = &'a Fingering>) -> i32
                {
                    candidates
                        .map(|candidate|candidate.keys)
                        .map(|keys|note.keys.intersection(keys).len())
                        .max()
                        .unwrap() as i32
                }

                // Determine how different a note is from a list of candidates
                fn difference<'a>(note: &Fingering, candidates: impl Iterator<Item = &'a Fingering>) -> i32
                {
                    candidates
                        .map(|candidate|candidate.keys)
                        .map(|keys|note.keys.symmetrical_difference(keys).len())
                        .min()
                        .unwrap() as i32
                }

                // Take two candidates notes and one or more others to compare against, determine which note is closer.
                // Note 1 is favored slightly in that it will be used in the case of equivalence
                fn compare_notes<'a>(note_1: &'a Fingering, note_2: &'a Fingering, siblings: &Vec<&Fingering>) -> &'a Fingering
                {
                    let note_1_intersection = intersection(note_1, siblings.iter().map(|f|*f));
                    let note_1_diff = difference(note_1, siblings.iter().map(|f|*f));
                    let note_2_intersection = intersection(note_2, siblings.iter().map(|f|*f));
                    let note_2_diff = difference(note_2, siblings.iter().map(|f|*f));
                    if note_2_intersection - note_2_diff > note_1_intersection - note_1_diff
                    {
                        note_2
                    }
                    else
                    {
                        note_1
                    }
                }

                // Determine what fingering to actually use
                previous_choice = Some(if fingerings.len() == 1
                {
                    &fingerings[0]
                }
                else if i == 0
                {
                    let siblings = self.notes()[i + 1].fingerings.iter().collect();
                    fingerings.iter().fold(&fingerings[0], |note_1, note_2| &compare_notes(note_1, note_2, &siblings))
                }
                else
                {
                    let sibling = vec!(previous_choice.unwrap());
                    fingerings.iter().fold(&fingerings[0], |note_1, note_2| &compare_notes(note_1, note_2, &sibling))
                });

                // Return the choice made above (it's actually the "current_choice" at this point)
                previous_choice
            })
            .map(|fingering| &fingering.image)
            .collect()
    }
}
