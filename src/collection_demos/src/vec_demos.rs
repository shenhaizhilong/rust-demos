
#[cfg(test)]
mod tests {
    #[test]
    fn vec_demo() {

        let mut  v1 = vec![1, 2, 3];
        println!("{:?}", v1);
        assert_eq!(v1, vec![1, 2, 3]);
        assert_eq!(v1.len(), 3);

        v1.push(4);
        assert_eq!(v1.len(), 4);
        assert_eq!(v1.pop(), Some(4));
        assert_eq!(v1.len(), 3);


    }

    #[test]
    fn vec_demo2() {
        let mut v1 = vec![0; 10];
        v1[0] = 1;
        v1[1] = 1;
        for i in 2..v1.len() {
            v1[i] = v1[i-1] + v1[i-2];
        }

        println!("{:?}", v1);
    }
}