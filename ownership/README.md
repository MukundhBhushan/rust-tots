Stack: stores values in the order it gets them. Removes values in the reverse order. LIFO
push and pop are the operations
data loaded in the stack must contain a known fixed value
fast data retrivals 
stores: string bool int 

Heap: stores data in unorganized format 
can store data in varied sizes
uses pointers 
slower data accessing
more conplex data types

Ownerrship
Rules
1. each value has a variable assigned to it which is called its owner
the first rules talk mainly about the scope of the variable
2. there can be one owner of a value at a time
3. when the owner goes out of the scope the value is droped


Ownership: Memory allocation
to mutate the variable with unknown amout at compile time to the heap.

The memory must be requested at runtime <- universal and same across all programing languages
Return the memory back to the operating system when done with our variable <- this is where rust is different.

In other programing languages this step is taken cre of with the help of a Garbage collector. A GC keeps track of all the memory been allocated and clears up space whn not needed any longer.
However, Rust does not have a garbage collector. 
Rust releases the memory when the variable is out of scope
Rust use the "drop" function to release memory

Memory allocation in heap
let s1 = String::from("Mukundh");
let s2 = s1;

a stack containg the variable s1 contains info such as 
name value
ptr  points to heap
len  7
capacity 5

heap 
index value
0      M
1      k

Now concider s1 and s2 go out of scope then they will be freeing the same memory this is known as double free error.
to tackle this the Rust releases the vsriable s1 leaving only s2 

Referncing and ownership
