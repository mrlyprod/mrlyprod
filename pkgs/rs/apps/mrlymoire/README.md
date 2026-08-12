# moire

Two dot lattices laid over each other until they interfere. A base layer repeats a corner rule nine times across a 100 by 100 field; the overlay is the same rule rotated in quarter turns and repeated a few more or fewer times, and where the layers slip out of register the moire fringes appear. The two masks are summed and painted on a nine-step ramp from black to cyan.

## Dials

- *offset* shifts the overlay's count away from the base 9, anywhere from -6 to 6.
- *angle* turns the overlay's rule by 0, 90, 180, or 270 degrees.
- *lattice* lays the dots on a square or a hex grid.
- Cells covered by both layers glow brightest; cells covered by neither stay black.
- reset returns to offset 4, angle 90, and the square lattice.
