# mrlymusic

The sound of Mrly. The synth end holds the four waveforms and their additive recipes, the midi table that pitches them in millihertz, the timbre that pairs a partial's shape with the series weighting it, and the renderers that turn a note into faded float samples, unit-peak tones, single-cycle wavetables or 16-bit pcm. The composer end walks voices through a chord progression: each voice owns a note pool, a set of movements and a timbre, each progression letter earns it a bar, and the bars concatenate into frames of midi notes that mix and normalize into one track. A wave encoder writes the samples out as a riff file.

## Parts

- **audio** holds the waves, the midi frequencies, the timbre and the renderers.
- **music** holds the voices, the movements, the chords and the composer.
- **wav** encodes mono 16-bit pcm as a riff wave file.
