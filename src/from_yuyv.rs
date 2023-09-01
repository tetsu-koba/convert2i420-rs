pub fn yuyv_to_i420(yuyv_data: &[u8], i420_data: &mut [u8], width: u32, height: u32) {
    assert!(width % 2 == 0);
    assert!(height % 2 == 0);
    assert!(yuyv_data.len() >= (width * height * 2) as usize);
    assert!(i420_data.len() >= (width * height * 3 / 2) as usize);

    let y_size = (width * height) as usize;
    let (y_plane, uv_plane) = i420_data.split_at_mut(y_size);
    let (u_plane, v_plane) = uv_plane.split_at_mut(y_size / 4);

    let mut uv_idx: usize = 0;
    let mut y = 0;
    while y < height {
        let mut x = 0;
        while x < width {
            let yuyv_idx = (y * width + x) as usize * 2;

            y_plane[(y * width + x) as usize] = yuyv_data[yuyv_idx];
            y_plane[(y * width + x + 1) as usize] = yuyv_data[yuyv_idx + 2];
            if y % 2 == 0 {
                u_plane[uv_idx] = yuyv_data[yuyv_idx + 1];
                v_plane[uv_idx] = yuyv_data[yuyv_idx + 3];
                uv_idx += 1;
            }
            x += 2;
        }
        y += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::yuyv_to_i420;

    #[test]
    fn yuyv_to_yuv420_conversion() {
        let yuyv_data = [16, 128, 17, 80, 18, 129, 19, 81];
        let mut yuv420_data = [0; 6];

        yuyv_to_i420(&yuyv_data, &mut yuv420_data, 2, 2);

        let expected_y_plane = [16, 17, 18, 19];
        let expected_u_plane = [128];
        let expected_v_plane = [80];

        assert_eq!(&expected_y_plane[..], &yuv420_data[0..4]);
        assert_eq!(&expected_u_plane[..], &yuv420_data[4..5]);
        assert_eq!(&expected_v_plane[..], &yuv420_data[5..6]);
    }

    #[test]
    fn yuyv_to_yuv420_conversion_2() {
        let yuyv_data = [
            16, 0x80, 17, 0x50, 18, 0x81, 19, 0x51, 20, 0x82, 21, 0x52, 22, 0x83, 23, 0x53,
        ];
        let mut yuv420_data = [0; 12];

        yuyv_to_i420(&yuyv_data, &mut yuv420_data, 4, 2);

        let expected_y_plane = [16, 17, 18, 19, 20, 21, 22, 23];
        let expected_u_plane = [0x80, 0x81];
        let expected_v_plane = [0x50, 0x51];

        assert_eq!(&expected_y_plane[..], &yuv420_data[0..8]);
        assert_eq!(&expected_u_plane[..], &yuv420_data[8..10]);
        assert_eq!(&expected_v_plane[..], &yuv420_data[10..12]);
    }

    #[test]
    fn yuyv_to_yuv420_conversion_with_diverse_data() {
        let yuyv_data = [
            50, 200, 60, 210, 70, 220, 80, 230, 90, 240, 100, 250, 110, 210, 120, 220, 130, 230,
            140, 240, 150, 250, 160, 210, 170, 220, 180, 230, 190, 240, 200, 250,
        ];
        let mut yuv420_data = [0; 48];

        yuyv_to_i420(&yuyv_data, &mut yuv420_data, 4, 4);

        let expected_y_plane = [
            50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160, 170, 180, 190, 200,
        ];
        let expected_u_plane = [200, 220, 230, 250];
        let expected_v_plane = [210, 230, 240, 210];

        assert_eq!(&expected_y_plane[..], &yuv420_data[0..16]);
        assert_eq!(&expected_u_plane[..], &yuv420_data[16..20]);
        assert_eq!(&expected_v_plane[..], &yuv420_data[20..24]);
    }
}
