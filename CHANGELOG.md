# Changelog

## [0.1.2] - 8/19/2020

- Added logic which tries to make vaguely intelligent decisions about which fingering chart to pick. It checks the fingering before and after,
and tries to pick one with more keys in common to its neighbors.
- To support the above change, added another configuration file that gives youy some control over notes. Not well-documented yet, but that's coming soon.
- Code refactoring to handle the increasing size of this project.

#### Future plans:

- Considering generating key images at runtime instead of storing them in a bunch of images. It would be fairly easy to do so now that notes store key information.
- Altissimo/Trill fingerings are looking like good ideas for the next update, although a bit more infrastructure may be needed.

## [0.1.0] - 8/14/2020

Initial release.

It doesn't contain all of the intended features, but it works! The worst feature of this release to look out for is that when using a midi file with notes outside the supported range, those notes will left out of the charts at the end. A warning is printed to the console, but even that may be missed. Therefore, make sure you only try to convert songs with notes ranging from Bb2 to F5 (slightly higher or lower when taking transposition into account)!