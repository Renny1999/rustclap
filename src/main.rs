use std::sync::{Arc, Mutex, Condvar};
use std::sync::atomic::{AtomicBool,Ordering};
use std::sync::mpsc::sync_channel;
use std::thread;
mod inputthread;
mod processing_thread;
mod util;

use inputthread::input_thread;
use processing_thread::processing_thread;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub struct App {
    gl: GlGraphics,
    rotation: f64,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x,y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c,gl |{
            clear(GREEN, gl);

            let transform = c
                .transform
                .trans(x,y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {

    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("spinning-square", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();



    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }

    return ;

    let exit = Arc::new(AtomicBool::new(false)); // exit flag
    let input_exit = exit.clone(); // exit flag for capture thread
    let process_exit = exit.clone(); // exit flag for capture thread
    let (input_tx, process_rx) = sync_channel(1024); 
 
    /* because the input thread requires user input in the beginning, 
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
    thread::spawn(|| input_thread(input_exit, proceed_clone, input_tx));
    println!("spawning processing thread");
    thread::spawn(||processing_thread(process_exit, process_rx));
    // wait to be signaled and when woken up, resume needs to be set to true
    while !*resume {
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

#[allow(unused)]
fn print_array(arr : &[f32]) {
    for i in 0..arr.len() {
        println!("{}", arr[i]);
    }
}


