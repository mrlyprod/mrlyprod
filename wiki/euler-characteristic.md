---
title: The Euler characteristic
lead: Count the corners, subtract the edges, add the faces; the answer is 2 for anything shaped like a ball, and bending the shape never changes it.
prerequisites: sierpinski-carpet
---

Take a cube and count three things. It has 8 corners, 12 edges and 6 faces. Now compute corners minus edges plus faces: `8 - 12 + 6 = 2`. The left panel of the figure is that cube, its 8 corners marked, its 12 edges drawn, three of its six faces facing you and three behind.

Try another solid. A tetrahedron has 4 corners, 6 edges and 4 faces, and `4 - 6 + 4 = 2`. An octahedron has 6 corners, 12 edges and 8 faces, and `6 - 12 + 8 = 2`. A football, a pyramid, a brick with a bite out of it: every one of them gives 2.

The reason is that the count does not care about shape at all. Imagine the solid made of rubber and inflated until it is a sphere. The corners, edges and faces are now drawn on the sphere, and pushing them around, or cutting a face in two, or joining two edges, changes the three counts only in ways that cancel. Cut a face in half and you add one edge and one face, which cancel. So the answer depends on the sphere and not on the drawing, and for a sphere the answer is 2.

That number is the Euler characteristic. It is what is left of a shape when you forget every length and every angle: a single integer that survives any amount of stretching.

Change the shape and the number changes. A doughnut has a hole running through it, and no amount of inflating will turn it into a ball. Draw corners, edges and faces on a doughnut and the count comes out 0 every time, not 2. Two holes give -2, three give -4. Each hole costs 2, so the number is really a count of holes with a sign on it.

Pictures made of cells are the same idea with a shorter recipe. For a flat shape built out of little squares, the Euler characteristic is the number of separate pieces minus the number of holes. A solid blob is one piece with no holes, so it is 1. A square ring is one piece around one hole, so it is 0. A blob with two holes in it is -1.

The recipe is the same count as before if you insist on the long form: corners of the little squares, minus their edges, plus the squares themselves. The ring that is the Sierpinski carpet's first level has 16 corners, 24 edges and 8 cells, and `16 - 24 + 8 = 0`, which is what one piece around one hole should give.

The carpet at level 2 is the right panel of the figure. It is still one connected piece, and it has 9 holes: the big three by three hole in the middle and eight single-cell holes around it, each ringed in the figure. So its Euler characteristic is `1 - 9 = -8`. Level 3 has 73 holes and gives `1 - 73 = -72`, and level `n` has `(8^n - 1)/7` holes, so the number falls away as fast as the holes arrive.

That is what the count is for. Pieces and holes are the two things you can see in a picture without measuring anything, and the Euler characteristic bundles them into one integer that no stretching can move. It is the crudest description of a shape that is still worth having, and because it is crude it is cheap to compute and hard to fool.

## In the tree

[Structure against noise](/research/connectivity/) counts pieces, holes and the Euler characteristic of the designs, and those three are the readings that notice the order in which tiles are stamped together, where fill, side and density cannot tell one order from another. The shape the count is taken on here is [the Sierpinski carpet](/wiki/sierpinski-carpet/).
