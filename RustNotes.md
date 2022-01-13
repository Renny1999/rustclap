## ownerships
* reference
  * (shared/mutable) referencs to a struct object
    * when I have a (shared/mutable) referencs to a struct object, **remember**
      that essentially, we have a pointer to that object.
    * this means that if we access a member of that object, by default, we
      are not borrowing the member.
    * Example:
        * as you can see, `obj_ref` is just a pointer to `obj`, 
          and `let obj_ref_vec = obj_ref.v;` will not work because it is trying 
          to move an unowned nor borrowed value (`obj.v`) to `obj_ref_vec` 
      ```
      pub struct Obj {
          v: Vec<f32>,
      }
      
      fn main() {
          let mut obj = Obj{v: vec![0.0]};
          let obj_ref = &mut obj;
          let obj_ref_vec = objb.v; // error: consider borrowing here: `&obj_ref.v` 
      }
      ```
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
    
## enums
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

## Option
  * Option is something that either ```Some``` or is ```None```
  * ```Some``` has a value
  * ```None``` does not have a value
  * can be matched with ```match``` 

## Result
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
## Closures & 'static
  * closure is an anonymous function
    ```
    |parameter1, parameter2| {
      // function body  
    }
    ```
  * closure has 3 types:
    * ```FnOnce```: the closure can be executed only once
    * ```Fn```: the closure can be called multiple times without mutating state
    * ```FnMut```: the closure can be executed multiple times and can 
                    mutate state
  * EXAMPLE
    * the code below takes two closures as callback functions 
      ```
        let path = "output.raw";
        let mut output = File::create(path).unwrap();
        let stream = device.build_input_stream (
            &config.into(),
            |data : &[f32], _: &_| {
                match write_vec(&mut output, data) {
                    Ok(_) => {println!("write to file successful")}, 
                    Err(_) => {panic!("error writing to file")},
                }
            }, 
            move |err| {
                // react to errors here
                panic!("{}", err);
            },
        ).unwrap();
      ```
    * where:
      ```
      fn build_input_stream_raw<D, E>(
          &self,
          config: &StreamConfig,
          sample_format: SampleFormat,
          data_callback: D,
          error_callback: E
      ) -> Result<Self::Stream, BuildStreamError>
      where
          D: FnMut(&Data, &InputCallbackInfo) + Send + 'static,
          E: FnMut(StreamError) + Send + 'static,  
       ```
      * the code compiles with the error:
     ```
            error[E0373]: closure may outlive the current function, but it borrows `output`, which is owned by the current function
         --> src\inputthread.rs:91:9
          |
      91  |         |data : &[f32], _: &_| {
          |         ^^^^^^^^^^^^^^^^^^^^^^ may outlive borrowed value `output`
      92  |             // pass data to main thread or clap detection thread
      93  |             match write_vec(&mut output, data) {
          |                                  ------ `output` is borrowed here
          |
      note: function requires argument type to outlive `'static`
         --> src\inputthread.rs:89:18
          |
      89  |       let stream = device.build_input_stream (
          |  __________________^
      90  | |         &config.into(),
      91  | |         |data : &[f32], _: &_| {
      92  | |             // pass data to main thread or clap detection thread
      ...   |
      101 | |         },
      102 | |     ).unwrap();
          | |_____^
      help: to force the closure to take ownership of `output` (and any other referenced variables), use the `move` keyword
          |
      91  |         move |data : &[f32], _: &_| {
          |         ++++
      ```
      * this error is happening because the main thread can exit before the 
        audio thread that will be spawned by ```stream```
      * if that happens, the audio thread will be using the ```output``` variable 
        that has been freed by the main thread as the main thread exited
        * this results in **use-after-free** memory safety violation
      * to ensure memory safety the ```'static``` bound on 
        ```
        D: FnMut(&Data, &InputCallbackInfo) + Send + 'static,
        ```
        requires all variables in the closure to live for the entire lifetime 
        of the program
      * in this example, it means that ```output``` has to outlive the main 
        thread
      * in order to meet the ```'static``` constraint, we need the closure to 
        have ownership of ```output```
      * FIX
        * for the reasons described above, move closures will solve the problem
        ```
        // ...
        move |data : &[f32], _: &_| {
        // ...
        ```
      * P.S. 
        * if the closure does not borrow any value from the main thread, then
          the error would not have happened
          * this code is fine
            ```
            |data : &[f32], _: &_| {
                // doing nothing
            }, 
            ```
