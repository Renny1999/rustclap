use std::sync::{Arc, Mutex, Condvar};
use std::sync::atomic::{AtomicBool,Ordering};
use std::sync::mpsc::sync_channel;
use std::thread;

mod inputthread;
mod processing_thread;

use inputthread::input_thread;
use processing_thread::processing_thread;

fn main() {
    let exit = Arc::new(AtomicBool::new(false)); // exit flag
    let input_exit = exit.clone(); // exit flag for capture thread
    let process_exit = exit.clone(); // exit flag for capture thread
    let (input_tx, process_rx) = sync_channel(1024); 
    
    /* bucause the input thread requires use input in the beginning, 
     * main thread has to wait until the input thread is done with user 
     * input before it can ask user to type 'exit' 
     * 
     * this will be accomplished with a mutex and condition variable    
     */     
        
    // mutex and condition variable 
    let proceed = Arc::new((Mutex::new(false), Condvar::new()));
    // mutex and condition variable for the input thread
    let proceed_clone = Arc::clone(&proceed);
        
    // get the mutex and the condition variable 
    let (lock, condition_variable) = &*proceed;
    // acquire the MutexGuard (lock)    
    let mut resume = lock.lock().unwrap();
    // this mutex guard contains a bool that indicates whether the main thread 
    // can proceed  
    *resume = false;
    // spawn input thread while holding the mutex
    thread::spawn(move ||input_thread(input_exit, proceed_clone, input_tx));
    println!("spawning processing thread");
    thread::spawn(move ||processing_thread(process_exit, process_rx));
    // wait to be signaled and when woken up, resume needs to be set to true
    while !*resume  {
        resume = condition_variable.wait(resume).unwrap();
    }
        
    // main thread may proceed
    
    while !exit.load(Ordering::Relaxed) {
        let mut ans = String::new();
        std::io::stdin().read_line(&mut ans).unwrap();
        let ans = ans.trim();
        println!("{}", ans); 
        if ans.eq("exit") {
            exit.store(true, Ordering::Relaxed);
        }
    }
}

fn print_array(arr : &[f32]) {
    for i in 0..arr.len() {
        println!("{}", arr[i]);
    }
}
