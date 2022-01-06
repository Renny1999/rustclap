extern crate cpal;
use cpal::traits::HostTrait;
use cpal::traits::*;

pub fn input_thread (buffer : &mut[f32]){
    let host = cpal::default_host();
    let devices = host.input_devices();
    let devices = match devices {
        Ok(devicelist) => devicelist,
        Err(_error) => panic!("No device found!"),
    };

    let devices : Vec<cpal::Device> = devices.collect();

    //    for d in devices.iter() {
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
    let mut supported_configs_range = device.supported_input_configs()
                                            .expect("error while qusrying configs");
    // let supported_configs = supported_configs_range.next()
    //                         .expect("no supported config?!")
    //                         .with_max_sample_rate();
    // 
    let supported_configs_range: Vec<cpal::SupportedStreamConfigRange> = supported_configs_range.collect();
    let i = 0;
    for i in 0..supported_configs_range.len(){ 
 //    for config in supported_configs_range {
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
        //channels : selected_config.channels(),
        //sample_rate: selected_config.max_sample_rate(),
        //buffer_size: cpal::BufferSize::Fixed(512),
        channels : 1,
        sample_rate: cpal::SampleRate(48000),
        buffer_size: cpal::BufferSize::Fixed(512),
    };

    let stream = device.build_input_stream (
        &config,
        //do_something,
        move |data : &[f32], _: &_| {
            // do something
            println!("hello");
        }, 
        move |err| {
            // react to errors here
            panic!("{}", err);
        },
    ).unwrap();
    println!("stream created");

    
    let res = match stream.play(){
        Ok(_) => {},
        Err(err) => panic!("{}",err), 
    };
}

fn do_something (data: &[f32], _: &cpal::InputCallbackInfo) {
    // do something here
} 
