# waves

Ripples spreading through a fractal pool. The walls are built from a generated tile - a carpet, a net, an H-tree - and a tap drops a disturbance that rings outward, reflects off the walls, and interferes with everything already in motion. Crests and troughs are painted in two colors of your choosing, and the seeded simulation runs the same on the CPU or in a GPU shader.

## Playing

- Tap any open cell to drop a ripple. Taps on a wall are refused.
- *speed*, *damp*, *freq*, *sigma* and *amp* shape each wave - how fast it travels, how quickly it dies, how tight and tall it starts.
- *reflect* sets how much of a wave the walls give back, from 0 to 1.
- *design*, *number*, *level*, *padding*, *invert* and *subpixel* rebuild the wall mask and still the pool.
- *accent* colors the crests, *anti* the troughs, and *gain* amplifies both. *play* freezes time.
