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
    
* enums
  * example enum
    * ```Data1``` does not have any actual data associated with it
    * ```Data2``` has a ```char``` associated with it
    * ```Complex_Data``` has a C-style struct associated with it 
  ```
  enum Example {
    Data1,
    Data2(char),
    Complex_Data {
      a: i32,
      b: i32
    },
  };
  ```
  * we can get the values of an enum using ```match```
  ```
  // create an example enum
  let example_comlex_data = Example::Complex_Data {1,2}; 
  let mut complex_data = [0,0];
  match example_comlex_data {
    Example::Data1 => {// do something},
    Example::Data2(val) => {// do something},
    Example::Complex_Data {a,b} 
    => {
      complex_data[0] = *a; // need to dereference
      complex_data[0] = *b; // need to dereference
    }
  }
  ```

* Option
  * Option is something that either ```Some``` or is ```None```
  * ```Some``` has a value
  * ```None``` does not have a value
  * can be matched with ```match``` 

* Result
  * used for error handling 
  * can be matched with ```match```
  * ```?``` can be placed aftes the function to quickly acquire the value 
    from ```Ok``` or return the error for ```Err``` without using ```match```
   ```
   // f will be the file if Ok 
   // otherwise the function returns the error 
   let f = File::open("username.txt")?;
   let dummy = 0; // this code will not execute if File::open() failed
   ```
*
