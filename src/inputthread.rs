extern crate cpal;
use cpal::traits::HostTrait;
use cpal::traits::*;
use std::io::Error;
use std::fs::File;
use std::io::Write;

pub fn input_thread (buffer : &mut[f32]){
    let host = cpal::default_host();
    let devices = host.input_devices();
    let devices = match devices {
        Ok(devicelist) => devicelist,
        Err(_error) => panic!("No device found!"),
    };

    let devices : Vec<cpal::Device> = devices.collect();

    println!("Select the input device:");
    for num in 0..(&devices).len() {
        let d = &devices[num as usize];
        let name = match d.name() {
            Ok(name) => name,
            Err(_) => "no name".to_string(),
        };
        println!("{}\t{}", num, name);
    }

    let mut selection = String::new();
    std::io::stdin().read_line(&mut selection).unwrap();
    println!("User input : {}", selection); 

    let selection = selection.trim().parse::<u32>().unwrap();
    println!("User selected {}", selection);

    let device = &devices[selection as usize];  
    println!("{}", device.name().unwrap());
    let supported_configs_range = device.supported_input_configs()
                                            .expect("error while qusrying configs");
     
    // store the supported configs in a vector  
    let supported_configs_range: Vec<cpal::SupportedStreamConfigRange> 
                        = supported_configs_range.collect();

    for i in 0..supported_configs_range.len(){ 
        let config = &supported_configs_range[i];    
        let channels = config.channels();
        let min_fs = config.min_sample_rate();
        let max_fs = config.max_sample_rate();

        let mut buffer_size = [0,0];
        match config.buffer_size() {
            cpal::SupportedBufferSize::Range{min, max} 
            => {
                    buffer_size[0] = *min;
                    buffer_size[1] = *max;
                    println!("{}", min);
                    println!("{}", max);
            },
            cpal::SupportedBufferSize::Unknown 
            => {},
        };
        let sample_format = match config.sample_format() {
            cpal::SampleFormat::I16 => "I16",
            cpal::SampleFormat::U16 => "U16",
            cpal::SampleFormat::F32 => "F32",
        };

        println!("config {}\n\tchannels: {}\n\tmix fs: {}\n\tmax fs: {}\n\tformat : {}\n\tbuffer min : {}\n\tbuffer max : {}"
            , i, channels, min_fs.0, max_fs.0, sample_format, buffer_size[0], buffer_size[1]);
    }
        
    println!("Select a config :"); 

    let mut selection = String::new();
    std::io::stdin().read_line(&mut selection).unwrap();
    let selection = selection.trim().parse::<u32>().unwrap();

    println!("User input : {}", selection); 

    let selected_config = &supported_configs_range[selection as usize];
    let config : cpal::StreamConfig = cpal::StreamConfig{
        channels : selected_config.channels(),
        sample_rate: selected_config.max_sample_rate(),
        buffer_size: cpal::BufferSize::Default, // only this worked
    };

    let path = "output.raw";
    let mut output = File::create(path).unwrap();
    let stream = device.build_input_stream (
        &config.into(),
        move |data : &[f32], _: &_| {
            // pass data to main thread or clap detection thread
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
    println!("stream created");
    let _res = match stream.play(){
        Ok(_) => {println!("should be playing at this time")},
        Err(err) => panic!("{}",err), 
    };
    
    std::thread::sleep(std::time::Duration::from_secs(10));
}

fn write_vec(file: &mut File, samples: &[f32]) -> Result<(), std::io::Error> {
    let samples_u8 = unsafe {
        std::slice::from_raw_parts(
            samples.as_ptr() as *const u8,
            samples.len() * std::mem::size_of::<f32>(),
        )
    };
    file.write_all(samples_u8)
}


