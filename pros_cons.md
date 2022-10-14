Upsides

The main benefits of a linked list are the abilities to grow very large in size cheaply,
always maintain a certain direction, and allow to access items individually.
What makes this data structure unique?

There are a few points:
Low overhead allocation per item.
Item count is only limited by heap memory.
Mutation while iterating is possible.
A direction is strictly enforced—there is no going back.
Implementation is fairly simple (even in Rust).
Efficient append, prepend, delete, and insert operations—compared to an array
(no shifting required).
Generally, the linked list performs well in an environment where limited memory does not
allow overhead allocation (as dynamic arrays do), or as a basis for an exotic lock-free data
structure.

Downsides
The linked list has some obvious shortcomings:
Indexing is inefficient, since every node has to be looked at.
Iteration in general involves a lot of jumping around on the heap, which takes
more time and makes the operation hard to cache.
Reversing a list is very inefficient.
The last point is important, so, commonly, a linked-list implementation will have a link
back as well, which makes it a doubly linked list.
