pub fn nv12_to_i420(nv12_data: &[u8], i420_data: &mut [u8], width: u32, height: u32) {
    assert!(width % 2 == 0);
    assert!(height % 2 == 0);
    assert!(nv12_data.len() >= ((width * height * 3) / 2) as usize);
    assert!(i420_data.len() >= ((width * height * 3) / 2) as usize);

    let y_size = (width * height) as usize;
    let uv_size = y_size / 2;

    // Copy the Y plane
    i420_data[0..y_size].copy_from_slice(&nv12_data[0..y_size]);

    // Separate the U and V planes
    let (u_plane, remaining) = i420_data[y_size..].split_at_mut(y_size / 4);
    let v_plane = &mut remaining[0..(y_size / 4)];
    let nv12_uv_plane = &nv12_data[y_size..];

    let mut i = 0;
    while i < uv_size {
        u_plane[i / 2] = nv12_uv_plane[i];
        v_plane[i / 2] = nv12_uv_plane[i + 1];
        i += 2;
    }
}

#[cfg(test)]
mod tests {
    use super::nv12_to_i420;

    #[test]
    fn nv12_to_i420_test() {
        // Image dimensions
        let width: u32 = 4;
        let height: u32 = 4;

        // NV12 input data
        let nv12_data: &[u8] = &[
            0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B, 0x8C, 0x8D,
            0x8E, 0x8F, 0x40, 0xA0, 0x41, 0xA1, 0x42, 0xA2, 0x43, 0xA3,
        ];

        // Expected I420 output data
        let expected_i420_data: &[u8] = &[
            0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B, 0x8C, 0x8D,
            0x8E, 0x8F, 0x40, 0x41, 0x42, 0x43, 0xA0, 0xA1, 0xA2, 0xA3,
        ];

        // Allocate buffer for the converted data
        let mut i420_data = vec![0u8; (width * height * 3 / 2) as usize];

        // Call the conversion function
        nv12_to_i420(nv12_data, &mut i420_data, width, height);

        // Check if the conversion was successful
        assert_eq!(i420_data, expected_i420_data);
    }
}
