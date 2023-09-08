use std::env;
use std::fs::File;
use std::io::Read;
use std::process;
mod from_i422;
mod from_nv12;
mod from_yuyv;
mod pipe;

type ConvertFunc = fn(&[u8], &mut [u8], u32, u32);

fn convert_to_i420(
    mut infile: File,
    outfile: File,
    width: u32,
    height: u32,
    pixel_format: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = pipe::Writer::new(outfile);
    let mut output_data = vec![0u8; (width * height * 3 / 2) as usize];
    let f: ConvertFunc;
    let mut input_data: Vec<u8>;

    match pixel_format {
        "YUYV" => {
            input_data = vec![0u8; (width * height * 2) as usize];
            f = from_yuyv::yuyv_to_i420;
        }
        "NV12" => {
            input_data = vec![0u8; (width * height * 3 / 2) as usize];
            f = from_nv12::nv12_to_i420;
        }
        "YUV422" | "I422" => {
            input_data = vec![0u8; (width * height * 2) as usize];
            f = from_i422::i422_to_i420;
        }
        _ => {
            return Err(format!("Doesn't support {}", pixel_format).into());
        }
    }

    loop {
        if let Err(e) = infile.read_exact(&mut input_data) {
            if e.kind() == std::io::ErrorKind::UnexpectedEof {
                break;
            }
            return Err(e.into());
        }
        f(&input_data, &mut output_data, width, height);

        if let Err(e) = writer.write_all(&output_data) {
            if e.kind() == std::io::ErrorKind::BrokenPipe {
                break;
            }
            return Err(e.into());
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 6 {
        println!(
            "Usage: {} input_file output_file width height pixelformat",
            args[0]
        );
        process::exit(1);
    }

    let infile = File::open(&args[1])?;
    let outfile = File::create(&args[2])?;
    let width: u32 = args[3].parse()?;
    let height: u32 = args[4].parse()?;
    let pixel_format = &args[5].to_uppercase();

    match pixel_format as &str {
        "YUYV" | "NV12" | "YUV422" | "I422" => {}
        _ => {
            eprintln!("Doesn't support {}", pixel_format);
            process::exit(1);
        }
    }

    convert_to_i420(infile, outfile, width, height, pixel_format)?;

    Ok(())
}
