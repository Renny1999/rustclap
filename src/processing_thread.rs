use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::sync_channel;
use std::sync::{Arc, Mutex, Condvar};
use std::fs::File;

use crate::inputthread::*;

const PREFIX: &str = "PROCESSING";

pub fn processing_thread (exit: Arc<AtomicBool>, rx: std::sync::mpsc::Receiver<Packet>) {   
    println!("{}", "Processing thread started");  
    let mut packet: Packet;

    let mut file;
    match std::fs::File::create("output.wav") {
      Err(_e) => {
        panic!("failed to create file");
      },
      Ok(f) => {
        file = f;
      }
    }
    while !exit.load(Ordering::Relaxed) {
        packet = match rx.recv(){
            Ok(p) => {
              println!("got data");
              match write_vec(&mut file, &p.data) {
                 Ok(_) => { println!("wrote to file") },
                 Err(_) => {panic!("error writing to file")},
              }
              p
            }
            Err(e) => {
                println!("{} {}", PREFIX, e);
                println!("{} failed to get data", PREFIX);
                return;
            }
        };
        let data = packet.data;
        convolve(&data, &data);
    }
}  

/*
 * Performs convolution with zero padding
 * the output has length a.len() + b.len() - 1
 */
pub fn convolve(a: &[f32], b: &[f32]) -> Option<Vec<f32>> {
    if a.len() == 0 || b.len() == 0 {
        return None;
    }

    let longer: &[f32];
    let shorter: &[f32];
    if a.len() > b.len() {
        longer = a;
        shorter = b;
    }else {
        longer = b;
        shorter = a;
    }
    let llong = longer.len();
    let lshort = shorter.len();

    let output_size = llong + lshort - 1;
    // outer loop iterates over a
    let mut output = Vec::<f32>::with_capacity(output_size);
    // finish all the padding parts
    for j in 0..lshort {
        let mut longi = (j as i32) - ((lshort - 1) as i32);
        let mut sum = 0.;
        for i in 0..lshort {
            if longi < 0 {
                sum += 0.;
            }else {
                sum += longer[longi as usize] * shorter[lshort-1-i];
            }
            longi += 1;
        }
        output.push(sum);
    }

    for i in 1..llong {
        let mut sum = 0.;
        for j in 0..lshort {
            if i+j >= llong {
                sum += 0.;
            } else {
                sum += longer[i+j] * shorter[lshort-1-j];
            }
        }
        output.push(sum);
    }

    return Some(output);
}

pub fn correlate(a: &[f32], b: &[f32]) -> Option<Vec<f32>> {
    let res;
    let reversed = reverse_slice(b).unwrap();

    res = convolve(a, &reversed);
    res
}

pub fn reverse_slice<T: Copy>(a: &[T]) -> Option<Vec<T>> {
    let mut res = Vec::<T>::with_capacity(a.len());
    for num in a.iter().rev() {
        res.push(*num);
    }
    
    return Some(res);
}

#[test]
fn conv_short_and_long (){
    let res = convolve(&vec![2.,3.,4.], &vec![1.,2.,3.,4.]).unwrap();
    assert_eq!(res, vec![2., 7., 16., 25., 24., 16.]);
}

#[test]
fn conv_long_and_short (){
    let res = convolve(
        &vec![34.,123., -3.33, -3214.57, 43190., 194.], 
        &vec![-9824.432,2.,3.,4.]).unwrap();
    assert_eq!(res, vec![-334030.7, -1208337.1, 33063.36, 31581822.0, -424323170.0, -1829216.8, 117099.72, 173342.0, 776.0]);
}

#[test]
fn conv_same_length (){
    let res = convolve(
        &vec![1.,2.,3.,4.], 
        &vec![1.,2.,3.,4.]).unwrap();
    assert_eq!(res, vec![1., 4., 10., 20., 25., 24., 16.]);
}

#[test]
fn conv_zero_length (){
    let res = convolve(
        &vec![], 
        &vec![1.,2.,3.,4.]);
    assert_eq!(res, None);
}

#[test]
fn corr_with_self (){
    let res = correlate(
        &vec![1.,2.,3.,4.,5.],
        &vec![1.,2.,3.,4.,5.]).unwrap();
    assert_eq!(res, vec![5.00,14.00,26.00,40.00,55.00,40.00,26.00,14.00,5.00]);
}

#[test]
fn rev_whatever (){
    let res = reverse_slice(&vec![1.,2.,3.,4.]).unwrap();
    assert_eq!(&res, &vec![4.,3.,2.,1.]);
}
