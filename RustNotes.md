## for loops
* iterators:
  * if I have a iterator ```iter``` and I wanted to loop over the elements in 
    it, I can simply do
    ```
     for v in iter {
       // do something
     }
    ```
  * However, the loop takes ownership of the iterator, so the second loop in 
    the example below will cause an error
    ```
     for v in iter {
       // do something
     }

     for v in iter {     // use after move error
       // do something
     }
    ```
  * iterators have a range associated with them, and iterating over the 
    elements will consume the range. We the following code will compile, but 
    because the first loops consumes the range, the second loop will not 
    execute
    ```
     for v in &mut iter {
       // do something
     }

     for v in &mut iter {     // will not execute
       // do something
     }
    ```
  * we do not need a to declare the iterator as mutable if we want to use
    for loop to iterate over it because as the for-loop will takes ownership
    of the iterator, it internally makes the iterator mutable.
    
