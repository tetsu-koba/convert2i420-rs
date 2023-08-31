pub fn i422_to_i420(i422_data: &[u8], i420_data: &mut [u8], width: u32, height: u32) {
    assert!(width % 2 == 0);
    assert!(height % 2 == 0);
    assert!(i422_data.len() >= (width * height * 2) as usize);
    assert!(i420_data.len() >= (width * height * 3 / 2) as usize);

    let y_size = (width * height) as usize;

    // Copy the Y plane
    i420_data[0..y_size].copy_from_slice(&i422_data[0..y_size]);

    // Copy the U and V planes
    let (u_plane, v_plane) = i420_data[y_size..].split_at_mut(y_size / 4);
    let (i422_u_plane, i422_v_plane) = i422_data[y_size..].split_at(y_size / 2);

    let width = width as usize;
    let height = height as usize;

    for y in 0..(height / 2) {
        for i in 0..(width / 2) {
            u_plane[y * width / 2 + i] = i422_u_plane[y * width + i];
        }
    }

    for y in 0..(height / 2) {
        for i in 0..(width / 2) {
            v_plane[y * width / 2 + i] = i422_v_plane[y * width + i];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::i422_to_i420; // Replace this line if your function is defined elsewhere.

    #[test]
    fn test_i422_to_i420() {
        // Image dimensions
        let width: u32 = 4;
        let height: u32 = 4;

        // I422 input data
        let i422_data: [u8; 32] = [
            0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B, 0x8C, 0x8D,
            0x8E, 0x8F, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0xA0, 0xA1, 0xA2, 0xA3,
            0xA4, 0xA5, 0xA6, 0xA7,
        ];

        // Expected I420 output data
        let expected_i420_data: [u8; 24] = [
            0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B, 0x8C, 0x8D,
            0x8E, 0x8F, 0x40, 0x41, 0x44, 0x45, 0xA0, 0xA1, 0xA4, 0xA5,
        ];

        // Allocate buffer for the converted data
        let mut i420_data = vec![0u8; (width * height * 3 / 2) as usize];

        // Call the conversion function
        i422_to_i420(&i422_data, &mut i420_data, width, height);

        // Check if the conversion was successful
        assert_eq!(i420_data, expected_i420_data);
    }
}
