# hash

A fingerprint for any piece of text, up to 256 characters. The text is digested not by a textbook algorithm but by a cellular automaton - a 32x32 grid run for 16 rounds under a chosen rule - down to a 256-bit digest. That digest is shown as hex and drawn as a 64x64 fingerprint, inked in one of thirteen colors picked by the digest's first byte.

## Using

- Type any text and it is digested at once; empty text is refused.
- The *rule* can be life, maze, replicator or anneal - changing it redraws the fingerprint.
- Different text draws a different picture.
- Reset returns to the starting text, mrly, under the life rule.
