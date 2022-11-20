pub fn get_raw_bytes(ptr: *const u8) -> Vec<u8> {
    get_bytes_from_pointer(ptr)
}

fn get_bytes_from_pointer(ptr: *const u8) -> Vec<u8> {
    let size_of_size_field = std::mem::size_of::<u32>();
    let size_data =
        unsafe { Vec::from_raw_parts(ptr as *mut u8, size_of_size_field, size_of_size_field) };
    let size_a: [u8; 4] = size_data.clone().try_into().unwrap();
    std::mem::forget(size_data);
    let size = u32::from_le_bytes(size_a);

    let data_start = unsafe { ptr.add(size_of_size_field) };

    let mut data = Vec::with_capacity(size as usize);
    unsafe {
        for i in 0..size {
            data.push(data_start.add(i as usize).read());
        }
    }
    data
}

pub fn wrap_bytes(bs: &[u8]) -> Vec<u8> {
    let size = bs.len() as u32;
    let mut bb = size.to_le_bytes().to_vec();
    bb.extend(bs);
    bb
}

#[cfg(test)]
mod tests {
    use crate::mem::*;

    #[test]
    fn wrap_bytes_should_work() {
        let bs = vec![0x1, 0x2];
        let bs = wrap_bytes(&bs);
        assert_eq!(bs, vec![2, 0, 0, 0, 1, 2]);
    }

    #[test]
    fn get_bytes_from_pointer_should_work() {
        let data_with_size: Vec<u8> = vec![2, 0, 0, 0, 1, 2];
        let p = &data_with_size[0] as *const u8;
        let data = get_raw_bytes(p);
        assert_eq!(data, vec![1, 2])
    }
    #[test]
    fn convert_should_work() {
        let data = "Hello World!".as_bytes();
        let data_wrapped = wrap_bytes(data);
        println!("{:?}", data_wrapped);

        let data_1 = get_raw_bytes(&data_wrapped[0] as *const u8);
        assert_eq!(data, data_1);
    }
}
