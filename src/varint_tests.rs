#[cfg(test)]
mod tests {
    #[cfg(any(feature = "tokio_async", feature = "futures_async"))]
    use crate::reader::VarIntAsyncReader;
    #[cfg(any(feature = "tokio_async", feature = "futures_async"))]
    use crate::writer::VarIntAsyncWriter;

    use crate::reader::VarIntReader;
    use crate::varint::VarInt;
    use crate::varint::VarIntMaxSize;
    use crate::writer::VarIntWriter;

    #[test]
    fn test_varint_max_size() {
        assert_eq!(u64::varint_max_size(), 10);
        assert_eq!(u32::varint_max_size(), 5);
        assert_eq!(u16::varint_max_size(), 3);
        assert_eq!(u8::varint_max_size(), 2);
        assert_eq!(i64::varint_max_size(), 10);
        assert_eq!(i32::varint_max_size(), 5);
        assert_eq!(i16::varint_max_size(), 3);
        assert_eq!(i8::varint_max_size(), 2);
    }

    #[test]
    fn test_required_space() {
        assert_eq!((0u32).required_space(), 1);
        assert_eq!((1u32).required_space(), 1);
        assert_eq!((128u32).required_space(), 2);
        assert_eq!((16384u32).required_space(), 3);
        assert_eq!((2097151u32).required_space(), 3);
        assert_eq!((2097152u32).required_space(), 4);
    }

    #[test]
    fn test_encode_u64() {
        assert_eq!((0u32).encode_var_vec(), vec![0b00000000]);
        assert_eq!((300u32).encode_var_vec(), vec![0b10101100, 0b00000010]);
        assert_eq!(
            u64::MAX.encode_var_vec(),
            vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01]
        );
    }

    #[test]
    fn test_identity_u64() {
        for i in 1u64..100 {
            assert_eq!(
                u64::decode_var(i.encode_var_vec().as_slice()).unwrap(),
                (i, 1)
            );
        }
        for i in 16400u64..16500 {
            assert_eq!(
                u64::decode_var(i.encode_var_vec().as_slice()).unwrap(),
                (i, 3)
            );
        }
    }

    #[test]
    fn test_decode_max_u64() {
        let max_u64 = u64::MAX.encode_var_vec();
        let mut max_vec_encoded = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01];
        assert_eq!(max_u64, max_vec_encoded);
        assert_eq!(
            u64::decode_var(max_vec_encoded.as_slice()).unwrap().0,
            u64::MAX
        );

        // one more than max returns none
        max_vec_encoded[9] += 1;
        assert_eq!(u64::decode_var(max_vec_encoded.as_slice()), None);
        max_vec_encoded[9] = 127;
        assert_eq!(u64::decode_var(max_vec_encoded.as_slice()), None);
    }

    #[test]
    fn test_decode_max_u16() {
        let max_u16 = u16::MAX.encode_var_vec();
        let mut max_vec_encoded = vec![0xFF, 0xFF, 0x03];
        assert_eq!(max_u16, max_vec_encoded);
        assert_eq!(
            u16::decode_var(max_vec_encoded.as_slice()).unwrap().0,
            u16::MAX
        );

        // one more than max returns none
        max_vec_encoded[2] += 1;
        assert_eq!(u16::decode_var(max_vec_encoded.as_slice()), None);
        max_vec_encoded[2] = 127;
        assert_eq!(u16::decode_var(max_vec_encoded.as_slice()), None);
    }

    #[test]
    fn test_decode_max_u8() {
        let max_u8 = u8::MAX.encode_var_vec();
        let mut max_vec_encoded = vec![0xFF, 0x01];
        assert_eq!(max_u8, max_vec_encoded);
        assert_eq!(
            u8::decode_var(max_vec_encoded.as_slice()).unwrap().0,
            u8::MAX
        );
        // 4_294_967_295

        // one more than max returns none
        max_vec_encoded[1] += 1;
        assert_eq!(u8::decode_var(max_vec_encoded.as_slice()), None);
        max_vec_encoded[1] = 127;
        assert_eq!(u8::decode_var(max_vec_encoded.as_slice()), None);
    }

    #[test]
    fn test_decode_max_u32() {
        let max_u32 = u32::MAX.encode_var_vec();
        let mut max_vec_encoded = vec![0xFF, 0xFF, 0xFF, 0xFF, 0x0F];
        assert_eq!(max_u32, max_vec_encoded);
        assert_eq!(
            u32::decode_var(max_vec_encoded.as_slice()).unwrap().0,
            u32::MAX
        );
        // 4_294_967_295

        // one more than max returns none
        max_vec_encoded[4] += 1;
        assert_eq!(u32::decode_var(max_vec_encoded.as_slice()), None);
        max_vec_encoded[4] = 127;
        assert_eq!(u32::decode_var(max_vec_encoded.as_slice()), None);
    }

    #[test]
    fn test_encode_i64() {
        assert_eq!((0i64).encode_var_vec(), (0u32).encode_var_vec());
        assert_eq!((150i64).encode_var_vec(), (300u32).encode_var_vec());
        assert_eq!((-150i64).encode_var_vec(), (299u32).encode_var_vec());
        assert_eq!(
            (-2147483648i64).encode_var_vec(),
            (4294967295u64).encode_var_vec()
        );
        assert_eq!(
            (i64::max_value()).encode_var_vec(),
            &[0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01]
        );
        assert_eq!(
            (i64::min_value()).encode_var_vec(),
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01]
        );
    }

    #[test]
    fn test_decode_min_i64() {
        let min_vec_encoded = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01];
        assert_eq!(
            i64::decode_var(min_vec_encoded.as_slice()).unwrap().0,
            i64::min_value()
        );
    }

    #[test]
    fn test_decode_max_i64() {
        let max_vec_encoded = vec![0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01];
        assert_eq!(
            i64::decode_var(max_vec_encoded.as_slice()).unwrap().0,
            i64::max_value()
        );
    }

    #[test]
    fn test_encode_i16() {
        assert_eq!((150i16).encode_var_vec(), (300u32).encode_var_vec());
        assert_eq!((-150i16).encode_var_vec(), (299u32).encode_var_vec());
    }

    #[test]
    fn test_reader_writer() {
        let mut buf = Vec::with_capacity(128);

        let i1: u32 = 1;
        let i2: u32 = 65532;
        let i3: u32 = 4200123456;
        let i4: i64 = i3 as i64 * 1000;
        let i5: i32 = -32456;

        assert!(buf.write_varint(i1).is_ok());
        assert!(buf.write_varint(i2).is_ok());
        assert!(buf.write_varint(i3).is_ok());
        assert!(buf.write_varint(i4).is_ok());
        assert!(buf.write_varint(i5).is_ok());

        let mut reader: &[u8] = buf.as_ref();

        assert_eq!(i1, reader.read_varint().unwrap());
        assert_eq!(i2, reader.read_varint().unwrap());
        assert_eq!(i3, reader.read_varint().unwrap());
        assert_eq!(i4, reader.read_varint().unwrap());
        assert_eq!(i5, reader.read_varint().unwrap());

        assert!(reader.read_varint::<u32>().is_err());
    }

    #[cfg(any(feature = "tokio_async", feature = "futures_async"))]
    #[tokio::test]
    async fn test_async_reader() {
        let mut buf = Vec::with_capacity(128);

        let i1: u32 = 1;
        let i2: u32 = 65532;
        let i3: u32 = 4200123456;
        let i4: i64 = i3 as i64 * 1000;
        let i5: i32 = -32456;

        buf.write_varint_async(i1).await.unwrap();
        buf.write_varint_async(i2).await.unwrap();
        buf.write_varint_async(i3).await.unwrap();
        buf.write_varint_async(i4).await.unwrap();
        buf.write_varint_async(i5).await.unwrap();

        let mut reader: &[u8] = buf.as_ref();

        assert_eq!(i1, reader.read_varint_async().await.unwrap());
        assert_eq!(i2, reader.read_varint_async().await.unwrap());
        assert_eq!(i3, reader.read_varint_async().await.unwrap());
        assert_eq!(i4, reader.read_varint_async().await.unwrap());
        assert_eq!(i5, reader.read_varint_async().await.unwrap());
        assert!(reader.read_varint_async::<u32>().await.is_err());
    }

    #[test]
    fn test_unterminated_varint() {
        let buf = vec![0xffu8; 12];
        let mut read = buf.as_slice();
        assert!(read.read_varint::<u64>().is_err());
    }

    #[test]
    fn test_unterminated_varint_2() {
        let buf = [0xff, 0xff];
        let mut read = &buf[..];
        assert!(read.read_varint::<u64>().is_err());
    }

    #[test]
    fn test_decode_extra_bytes_u64() {
        let mut encoded = 0x12345u64.encode_var_vec();
        assert_eq!(u64::decode_var(&encoded[..]), Some((0x12345, 3)));

        encoded.push(0x99);
        assert_eq!(u64::decode_var(&encoded[..]), Some((0x12345, 3)));

        let encoded = [0xFF, 0xFF, 0xFF];
        assert_eq!(u64::decode_var(&encoded[..]), None);

        // Overflow
        let mut encoded = vec![0xFF; 64];
        encoded.push(0x00);
        assert_eq!(u64::decode_var(&encoded[..]), None);
    }

    #[test]
    fn test_decode_extra_bytes_i64() {
        let mut encoded = (-0x12345i64).encode_var_vec();
        assert_eq!(i64::decode_var(&encoded[..]), Some((-0x12345, 3)));

        encoded.push(0x99);
        assert_eq!(i64::decode_var(&encoded[..]), Some((-0x12345, 3)));

        let encoded = [0xFF, 0xFF, 0xFF];
        assert_eq!(i64::decode_var(&encoded[..]), None);

        // Overflow
        let mut encoded = vec![0xFF; 64];
        encoded.push(0x00);
        assert_eq!(i64::decode_var(&encoded[..]), None);
    }

    #[test]
    fn test_regression_22() {
        let encoded: Vec<u8> = (0x112233u64).encode_var_vec();
        assert_eq!(
            encoded.as_slice().read_varint::<i8>().unwrap_err().kind(),
            std::io::ErrorKind::InvalidData
        );
    }
}
