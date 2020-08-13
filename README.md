# Saxophone Fingering Chart Generator   

This program takes a MIDI file as input and generates fingering charts for saxophones matching the notes in that midi file. Time, note length, and other musical elements are not preserved, this is just a way to learn the fingering for each note.

## Usage   
- fingering_chart midi_path [-o output_path] [-t transposition] [-c source_charts] [-f format] [-n notes_per_row] [-s spacing]
### Args
- `-o` Sets the output path. If this is a non-existent directory, will attempt to create. Default value is `./out`
- `-t` Sets the transposition. May use `C`, `Bb`, or `Eb`. Default value is `Bb` (cause I play tenor)
- `-c` Sets the directory containing source charts. This directory must contain a png file for each note used. Default value is `./fingerings`
- `-f` Sets the output format. May use `tracks`, `rows`, or `separate`. `tracks` outputs charts for each midi track as a single file. `rows` splits tracks up into multiple files each containing a horizontal row of notes. `separate` outputs each individual note as a separate file. Default value is `tracks`.
- `-n` Sets the number of notes per row. Not applicable when using `separate` output format. Default value is `14`
- `-s` Sets the visual spacing between consecutive notes. Not applicable when using `separate` output format. Default value is `10`

## Planned Features   
- A configuration file instead of commaand line args. Possibly allow both or either.
- Alternate fingerings. Right now a single fingering is used per note, and it may not necessarily be the optimal choice given the song. I'm considering adding some logic that takes the previous/next note into account to decide which fingering to show on the chart.
- Fingerings for altissimo/trill
- Detect if a midi file has notes out of range, have an option to automatically attempt to transpose the midi file to fit in the sax range (by octave first, then by semitone). Currently notes out of range re just left out with a warning

## Example   
![The Lizards](https://raw.githubusercontent.com/Eolu/fingering_chart/master/examples/lizards.png)   
