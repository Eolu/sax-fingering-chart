# Saxophone Fingering Chart Generator   

This program takes a MIDI file as input and generates fingering charts for saxophones matching the notes in that midi file. Time, note length, and other musical elements are not preserved, this is just a way to learn the fingering for each note.

Fingerings are decided bya simple weighted algorithm that tries to pick fingerings for a note based the keys involved in the previous and next note. There may be a better way to do this, I'm open to ideas.

## Usage   
- fingering_chart [midi_file...]

Just drag a midi file (assuming you use an OS that supports that, eg Windows) onto `fingering_chart`, and wait for charts to generate! You can also run it via the command-line by specifying one or more midi files as shown above. Make sure the `cfg.ron` file is in the same directory as `fingering_chart`. See below for additional settings.

### cfg.ron
cfg.ron is the configuration file:
- `source_charts` Sets the directory containing source charts. This directory must contain a png file for each note used.
- `transposition` Sets the transposition. May use `C`, `Bb`, or `Eb`.
- `output_path` Sets the output path. If this is a non-existent directory, will attempt to create.
- `output_format` Sets the output format. May use `Tracks`, `Rows`, or `Separate`. `Tracks` outputs charts for each midi track as a single file. `Rows` splits tracks up into multiple files each containing a horizontal row of notes. `Separate` outputs each individual note as a separate file.
- `spacing` Sets the visual spacing between consecutive notes. Not applicable when using `Separate` output format.
- `notes_per_row` Sets the number of notes per row. Not applicable when using `Separate` output format.

## Planned Features   
- Fingerings for altissimo/trill
- Detect if a midi file has notes out of range, have an option to automatically attempt to transpose the midi file to fit in the sax range (by octave first, then by semitone). Currently notes out of range are just left out with a warning

## Example   
![The Lizards](https://raw.githubusercontent.com/Eolu/fingering_chart/master/examples/lizards.png)   
