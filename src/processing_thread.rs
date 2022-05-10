use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::sync_channel;
use std::sync::{Arc, Mutex, Condvar};

use crate::inputthread::*;

pub fn processing_thread (exit: Arc<AtomicBool>, rx: std::sync::mpsc::Receiver<Packet>) {   
    println!("{}", "Processing thread started");  
    while !exit.load(Ordering::Relaxed) {
       let data: Packet = rx.recv().unwrap();
       let data: &[f32] = &data.data;
       convolve(data, data);
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

#[test]
fn short_and_long (){
    let res = convolve(&vec![2.,3.,4.], &vec![1.,2.,3.,4.]).unwrap();
    assert_eq!(res, vec![2., 7., 16., 25., 24., 16.]);
}

#[test]
fn long_and_short (){
    let res = convolve(
        &vec![34.,123., -3.33, -3214.57, 43190., 194.], 
        &vec![-9824.432,2.,3.,4.]).unwrap();
    assert_eq!(res, vec![-334030.7, -1208337.1, 33063.36, 31581822.0, -424323170.0, -1829216.8, 117099.72, 173342.0, 776.0]);
}

#[test]
fn same_length (){
    let res = convolve(
        &vec![1.,2.,3.,4.], 
        &vec![1.,2.,3.,4.]).unwrap();
    assert_eq!(res, vec![1., 4., 10., 20., 25., 24., 16.]);
}

#[test]
fn zero_length (){
    let res = convolve(
        &vec![], 
        &vec![1.,2.,3.,4.]);
    assert_eq!(res, None);
}
