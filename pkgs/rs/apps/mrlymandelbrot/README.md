# mandelbrot

An endless zoom into the Mandelbrot set. Each cycle hunts for a point near the edge of the set, dives toward it for a while, fades out, and starts somewhere new. The escape counts are computed per pixel in plain Rust, with an optional GPU shader doing the same math; the target is chosen from a seed, so the same seed takes the same journey.

## Dials

- **zoom** sets how fast each step magnifies, **cycle** how long a dive lasts before it restarts.
- **depth** is how many iterations decide whether a point escapes; **band** stripes those counts into color.
- **drift** animates the palette through the bands, **spin** slowly rotates the plane.
- **fade** sets the crossfade in and out of each cycle.
- **primary** and **accent** are the two colors everything is mixed between.
