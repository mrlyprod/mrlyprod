export const HUES = ['red', 'orange', 'yellow', 'green', 'mint', 'teal', 'cyan', 'blue', 'indigo', 'purple', 'pink', 'brown'];

export const tints = (blank) => [['', blank], ...HUES.map((hue) => [hue, hue[0].toUpperCase() + hue.slice(1)])];
