use std::fs::File;
use std::io::Write;

pub fn write_vec(file: &mut File, samples: &[f32]) -> Result<(), std::io::Error> {
    let samples_u8 = unsafe {
        std::slice::from_raw_parts(
            samples.as_ptr() as *const u8,
            samples.len() * std::mem::size_of::<f32>(),
        )
    };
    file.write_all(samples_u8)
}
