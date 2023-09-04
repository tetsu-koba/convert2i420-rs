use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::os::fd::AsRawFd;
use std::process;
mod from_i422;
mod from_nv12;
mod from_yuyv;
mod pipe;

type ConvertFunc = fn(&[u8], &mut [u8], u32, u32);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 6 {
        println!(
            "Usage: {} input_file output_file width height pixelformat",
            args[0]
        );
        process::exit(1);
    }

    let mut infile = File::open(&args[1])?;
    let mut outfile = File::create(&args[2])?;
    let width: u32 = args[3].parse()?;
    let height: u32 = args[4].parse()?;
    let pixel_format = &args[5].to_uppercase();

    let is_pipe_output = if cfg!(linux) && pipe::is_pipe(outfile.as_raw_fd()) {
        pipe::set_pipe_max_size(outfile.as_raw_fd())?;
        true
    } else {
        false
    };
    let mut output_data = vec![0u8; (width * height * 3 / 2) as usize];
    let f: ConvertFunc;
    let mut input_data: Vec<u8>;

    if pixel_format == "YUYV" {
        input_data = vec![0u8; (width * height * 2) as usize];
        f = from_yuyv::yuyv_to_i420;
    } else if pixel_format == "NV12" {
        input_data = vec![0u8; (width * height * 3 / 2) as usize];
        f = from_nv12::nv12_to_i420;
    } else if pixel_format == "YUV422" || pixel_format == "I422" {
        input_data = vec![0u8; (width * height * 2) as usize];
        f = from_i422::i422_to_i420;
    } else {
        eprintln!("Doesn't support {}", pixel_format);
        process::exit(1);
    }

    loop {
        if let Err(e) = infile.read_exact(&mut input_data) {
            if e.kind() == std::io::ErrorKind::UnexpectedEof {
                break;
            }
            return Err(e.into());
        }
        f(&input_data, &mut output_data, width, height);

        // Write to the output file
        if let Err(e) = if is_pipe_output {
            pipe::vmsplice_single_buffer(&output_data, outfile.as_raw_fd())
        } else {
            outfile.write_all(&output_data)
        } {
            if e.kind() == std::io::ErrorKind::BrokenPipe {
                break;
            }
            return Err(e.into());
        }
    }

    Ok(())
}
