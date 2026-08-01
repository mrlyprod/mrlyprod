# specimen

A test bench for the system's widgets: one page holding a sample of every control, wired to real state. There is a line of sample text (the quick brown fox, until you change it), an overlay, a plain toggle, a three-way picker and a numeric span. Nothing here is saved - the page resets every session, which is the point.

## Dials

- *sample* is free text; it is trimmed, and an empty value is refused.
- *overlay* and *toggle* are on/off switches - one for the overlay layer, one bare.
- *pick* chooses between alpha, beta and gamma.
- *span* takes any whole number from 0 to 10, starting at 5.
- A wrongly typed or out-of-range value is rejected with a plain reason.
