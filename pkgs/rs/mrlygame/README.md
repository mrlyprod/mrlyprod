# mrlygame

The content generator. One seed grows a quest: chained life runs over drawn tiles, where the classic B3/S23 rule alternates with sequence-driven rules on drawn masks. Each chapter runs until it settles, pivots at a multiple of four, and hands its last grid to the next as a seed. An attempt that exhausts its chapters without a chapter settling alive is discarded and redrawn, so a quest always ends on a living still.

The emitter presses one quest to disk: neighbor-coloured frames, a chapter-ramped cumulative heatmap animation, a two-part soundtrack, and a record holding the replayable story plus an assembly manifest with tempos, per-frame durations, the freeze policy, segment boundaries, canvas size and the audio file name. No video is encoded here; an assembler downstream owns that.

## Using

- `cargo run -p mrlygame -- emit <dir> [seed]` generates one quest (seed 7 by default).

## Files

- `frames/NNNN.png` holds one frame per generation, coloured by live-neighbor count.
- `heatmap/NNNN.png` holds one cumulative heatmap per generation, ramped per chapter.
- `audio.wav` carries the frames half, then the heatmap half.
- `quest.json` holds the story and the manifest; the same seed rebuilds identical bytes.
