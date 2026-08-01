# chess

The full game of chess with the rules written from scratch: castling, en passant, promotion, check, checkmate and stalemate are all enforced. Both sides are played by hand on one board. The starting position is itself a setting - the board is dealt from a FEN-style layout string, so lopsided armies and odd board sizes are fair play.

## Playing

- Tap one of your pieces to see every legal destination lit up; tap a highlighted square to move there.
- Moves can also be given by square names, e2 to e4 style.
- A pawn reaching the far rank promotes - a queen unless you ask for a rook, bishop or knight.
- Checkmate ends the game for the side that delivered it; stalemate is a draw.
- *layout* deals any position up to 26 files wide, *obfuscate* scrambles the pieces into random symmetric marks, and *reskin* rolls fresh colors every so many moves.
