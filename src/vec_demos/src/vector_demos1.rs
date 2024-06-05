#[cfg(test)]
mod test {
    use std::convert::TryInto;

    #[test]
    fn test1() {
        let vec: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let arr: [u8; 16] = vec.try_into().unwrap();
        println!("{:?}", arr);

        let mut vec1 = vec![1, 2, 3, 4];
        let taken_vec = vec1.truncate(2); // 返回包含前两个元素的新向量，现在vec是[]

        let mut vec2 = vec![1, 2, 3, 4];
        let new_vec: Vec<i32> = vec2.drain(0..2).collect(); // 创建一个新向量，只包含前两个元素，并释放掉其余的内存
        println!("{:?}", new_vec);

        let a = [1, 2, 3, 4, 5];

        let slice = &a[1..3];

        assert_eq!(slice, &[2, 3]);



        // If the vector's length is not exactly 16, you can create a new array with the correct size
        // let mut vec_with_padding: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18];

        // // Use the take() method to only take the first 16 elements
        // let truncated_vec = &vec_with_padding.take(16);
        //
        // // Try to convert again
        // match truncated_vec.try_into() {
        //     Ok(array) => {
        //         println!("Converted Vec<u8> with padding to [u8; 16]: {:?}", array);
        //     }
        //     Err(_) => {
        //         println!("Failed to convert Vec<u8> with padding to [u8; 16].");
        //     }
        // }
    }

}