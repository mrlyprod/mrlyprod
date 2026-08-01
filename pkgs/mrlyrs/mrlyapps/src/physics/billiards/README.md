# billiards

Particles bouncing around inside a fractal. The walls are built from a generated tile - a carpet, a net, an H-tree - so instead of a flat table you get a chamber full of holes and narrow corridors, and the particles smear a slowly fading trail behind them as they travel. The simulation is seeded and runs the same whether it is drawn on the CPU or handed to a GPU shader.

## Playing

- Tap any open cell to break a fan of particles outward from that spot. Taps on a wall are refused.
- Each break adds to what is already bouncing, so the table fills up as you play.
- *design*, *number* and *level* choose the fractal; *padding* and *subpixel* set its border and detail.
- *speed*, *size* and *count* set how the particles move and how many arrive per break.
- *trail* controls how quickly the painted trail fades. *play* pauses everything.
