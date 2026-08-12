# julia

An endless dive through Julia sets. Pick the complex constant from six presets or dial in your own, and a wayfinder hunts down a spot near the set's edge, dives toward it for a cycle, fades out, and begins again somewhere new. The dive is seeded, so one seed always takes the same journey, and the same math runs on the CPU or a GPU shader.

## Dials

- *preset* offers six constants like -0.4+0.6i; *custom* frees *cre* and *cim* anywhere from -2 to 2.
- *zoom* magnifies each step by up to 1.05; *cycle* is a dive's length, 30 to 3000 steps.
- *depth* caps the iterations at 16 to 600; *band* stripes the escape counts into color and *drift* animates the palette.
- *spin* slowly rotates the plane; *fade* crossfades each cycle in and out.
- *width* and *height* size the frame up to 512 pixels; *primary* and *accent* are the two ends of the palette.
