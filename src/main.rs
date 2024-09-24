use std::fs::File;
use std::io::{self, Write};
use rand::Rng;

//for testing, we will create an image with the size of 256 x 256
fn main(){
    let width: u32 = 2560;
    let height: u32 = 1440;
    let path: &str = "output.bmp";
    let size = width * height * 3;
    let mut pixel_array = vec![0; size as usize];
    generate_random_pixel_array(size - 1, &mut pixel_array);
    match create_bmp(width, height, &pixel_array, path){
        Ok(()) => println!("BMP created, output file is: {:?}", path),
        Err(e) => eprintln!("Failed to create BMP: {:?}", e),
    }
}

fn get_bitmap_size(width: u32, height: u32) -> u32{
    (width * height * 3) + 40 + 14
}

fn create_bmp(width: u32, height: u32, pixel_array: &[u8], output_path: &str) -> io::Result<()> {
    let total_size = get_bitmap_size(width, height);
    let mut file = File::create(output_path)?;
    //Write BMP File Header
    file.write_all(&[
        0x42, 0x4D,             //B, M signature
        //file sizes in 8 bits because rust won't accept u32
        (total_size & 0xFF) as u8,
        ((total_size>>8) & 0xFF) as u8,
        ((total_size>>16) & 0xFF) as u8,
        ((total_size>>24) & 0xFF) as u8,       
        0x00, 0x00, 0x00, 0x00, //Reserved bytes
        0x36, 0x00, 0x00, 0x00, //File size in bytes before the pixel array starts
    ])?;
    // Write DIB Header 
    file.write_all(&[
        0x28, 0x00, 0x00, 0x00,
        (width & 0xFF) as u8,
        ((width >> 8) & 0xFF) as u8,
        0x00, 0x00,
        (height & 0xFF) as u8,
        ((height >> 8) & 0xFF) as u8,
        0x00, 0x00,
        0x01, 0x00,
        0x18, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x13, 0x00, 0x00, 0x00,
        0x13, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00 ,0x00,
        0x00, 0x00, 0x00, 0x00,
    ])?;
    
    let padding = (4 - (width * 3) % 4) % 4;
    for i in (0..height).rev() {
        for j in 0..width{
            let k = (i * width + j) as usize * 3;
            file.write_all(&[pixel_array[k + 2], pixel_array[k + 1], pixel_array[k]])?;
        } 
        file.write_all(&vec![0; padding as usize])?;
    }

    Ok(())

}

fn generate_random_pixel_array(array_size: u32, pixel_array: &mut [u8]){
    let mut rng = rand::thread_rng();
    for i in 0..array_size{
        pixel_array[i as usize] = rng.gen_range(0..255);
    }
}