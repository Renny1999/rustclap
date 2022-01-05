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
    for config in supported_configs_range {
        let channels = config.channels();
        let min_fs = config.min_sample_rate();
        let max_fs = config.max_sample_rate();
        let buffer_size = config.buffer_size();
        let sample_format = config.sample_format();

        println!("channels: {}\nmix fs: {}\nmax fs: {}", channels, min_fs.0, max_fs.0);
    }
//    let stream = device.build_input_stream(
//            &config,
//
//
//        );

}
