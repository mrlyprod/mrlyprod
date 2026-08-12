# colors

A browser for the house palette of fifteen named colors, one page per color. Each page shows the color's name, hex code and rgb values, and the whole palette can leave as a JSON file. A keep-and-drop library of favorites starts out holding every color, ready to be trimmed down.

## Using

- **next** and **prev** page through the fifteen colors and wrap at either end.
- Setting *name* jumps straight to a color; **reset** returns to the first page.
- **keep** puts the current color in the library; keeping it twice is refused.
- **drop** takes a color back out of the library by name.
- **export** downloads the full palette as palette.json, names and hex codes together.
