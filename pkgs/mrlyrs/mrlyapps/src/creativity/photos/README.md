# photos

A wall for the twelve most recent photos. Every new shot lands at the front, and once the wall is full the oldest falls off the back. A running counter remembers how many shots were ever taken, even after the pictures themselves have rolled away.

## Using

- New photos go in at the front of the wall; past twelve, the oldest is gone for good.
- Each photo is a small indexed image carrying its own palette.
- A malformed image is refused and never lands on the wall.
- **clear** empties the wall and says how many photos it removed.
- The wall is saved with the rest of your state, so it survives a reload.
