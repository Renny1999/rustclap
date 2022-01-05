use inputthread::input_thread;

mod inputthread;
//use inputthread::input_thread;

fn main() {
    let mut buf :[f32; 5] = [0.;5];
    input_thread(&mut buf);
}

fn print_array(arr : &[f32]) {
    for i in 0..arr.len() {
        println!("{}", arr[i]);
    }
}
