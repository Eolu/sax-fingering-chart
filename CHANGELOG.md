# Changelog

## [0.2.1] - 9/5/2020

- Realized charts were generating at 50% size right after publishing 2.0. Here's the fix.

## [0.2.0] - 9/5/2020

- Charts arte now much less busy, opting to not show notes when they aren't relevant to a fingering. The octave key and 6 front keys are always shown.
- Fingering charts are now 100% precedurally generated. No more archives of chart files! Releases will be a single executable with a config file.
- Got rid of a silly note-loading macro that was almost certainly a bad idea from the start. Now using the magic of ron and serde. You can edit the configuration file to add more notes as well as modify the fingering of existing notes!

#### Future plans:
- Still gives up if config file is not present, considering solutions (eg generating a default if it's not there).

## [0.1.3] - 8/29/2020

- Fingering decision logic slightly simplified.
- More refactoring and began adding some tests.
- Added some useful documention inside keys.cfg.

## [0.1.2] - 8/19/2020

- Added logic which tries to make vaguely intelligent decisions about which fingering chart to pick. It checks the fingering before and after,
and tries to pick one with more keys in common to its neighbors.
- To support the above change, added another configuration file that gives some control over notes. Not well-documented yet, but that's coming soon.
- Code refactoring to handle the increasing size of this project.

#### Future plans:

- Considering generating key images at runtime instead of storing them in a bunch of images. It would be fairly easy to do so now that notes store key information.
- Altissimo/Trill fingerings are looking like good ideas for the next update, although a bit more infrastructure may be needed.

## [0.1.0] - 8/14/2020

Initial release.

It doesn't contain all of the intended features, but it works! The worst feature of this release to look out for is that when using a midi file with notes outside the supported range, those notes will left out of the charts at the end. A warning is printed to the console, but even that may be missed. Therefore, make sure you only try to convert songs with notes ranging from Bb2 to F5 (slightly higher or lower when taking transposition into account)!