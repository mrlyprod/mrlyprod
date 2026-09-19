---
title: The Kronecker product
lead: Stamp a small picture into every filled cell of itself and it grows a level; that one move builds every design on this site.
prerequisites:
---

Take a small picture drawn on a grid of cells, some filled and some empty. The one on the left of the figure is three cells by three with only the centre empty: eight filled cells around one hole.

The Kronecker product is a way to multiply two such pictures. To multiply picture A by picture B, take A and replace every filled cell of A by a whole copy of B, and every empty cell of A by an empty block the size of B. The result is a bigger grid, as wide as the width of A times the width of B.

Multiply the eight-around-a-hole picture by itself and you get the middle panel of the figure: nine by nine, sixty-four filled cells, one hole of three by three in the middle and eight small holes around it, one inside each copy. Multiply once more by the same picture and you get the right panel: twenty-seven by twenty-seven, five hundred and twelve filled cells, and holes of three sizes.

The order matters. A times B replaces the cells of A with copies of B, so the big shape comes from A and the fine detail comes from B. With the same picture on both sides the difference vanishes, and that is the case that grows a fractal: a picture multiplied by itself again and again looks the same at every scale.

Counting is the easy part. A filled cell of A meeting a filled cell of B gives one filled cell of the product, so the filled cells multiply: eight times eight is sixty-four, and eight times eight times eight is five hundred and twelve. The side multiplies too: three, nine, twenty-seven. After `n` rounds the picture has `8^n` filled cells on a side of `3^n`, which is how a fractal's dimension will be read off later.

The name comes from matrices. Write a picture as a table of ones and zeros, one for filled and zero for empty. The Kronecker product of two tables multiplies every entry of the first by the whole second table and lays those blocks out in the pattern of the first. A one times the table is the table, a zero times the table is a block of zeros, and that is exactly the stamping rule above, so the picture and the matrix say the same thing.

## In the tree

Every design here is one small picture, a rule on the corners of a cube, grown by the Kronecker product. The [core note](/research/core/) states it as move two, [the sponge demo](/demos/sponge/) grows a cube rule level by level, and [the words demo](/demos/words/) multiplies two different pictures so the order shows.
