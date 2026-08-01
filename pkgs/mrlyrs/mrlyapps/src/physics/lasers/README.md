# lasers

Laser fans bouncing around inside a fractal. The walls come from a generated tile - a carpet, a net, an H-tree - and each emitter you place casts a fan of rays that reflect from wall to wall while the whole fan slowly rotates. The simulation is seeded and traces the same beams whether it is drawn line by line on the CPU or handed to a GPU shader.

## Playing

- Tap any open cell to place an emitter. Taps on a wall are refused.
- Each emitter casts up to 128 *rays*, each followed through up to 256 *bounces*.
- *spread* fans the rays out - none, narrow, wide, half, or a full circle - and *spin* rotates them at up to 2.0.
- *design*, *number*, *level*, *padding*, *invert* and *subpixel* rebuild the wall mask, clearing the emitters.
- *accent* colors the beams, and brightness stacks where rays cross. *play* pauses the run.
