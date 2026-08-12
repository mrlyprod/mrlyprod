# files

A shelf for up to 24 files, held entirely as base64 text. Each file arrives as a name, a mime type and its data, lands at the front of the shelf, and is handed back as a data URI with its size in bytes worked out from the encoding.

## Using

- New files go in at the front; past 24, the oldest falls off.
- A file needs a name and data; one without a mime type is shelved as plain binary.
- Every entry is stamped with the moment it arrived.
- **drop** removes one file by its position on the shelf.
- **clear** empties the shelf and reports how many files it removed.
