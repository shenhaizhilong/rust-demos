#[cfg(test)]
mod tests {
    use std::ffi::{CString, OsStr, OsString};
    use std::os::unix::ffi::OsStringExt;

    #[test]
    fn test1() {
        // Box<&str>：这是一个罕见的类型，通常不会在 Rust 代码中出现。它是一个指向 str 的堆分配引用。因此，以下例子是比较不常见的。
        let s = Box::new("Hello, World!");
        println!("{}", s);

        // Box：这是一个堆分配的固定长度字符串。它的使用比较罕见，只有在一些特殊的场合才会用到
        let s2: Box<str> = Box::from("Hello, World! 2");
        println!("{}", s2);

        //&String：这是一个指向 String 的引用，常用于函数参数，以便可以接受 String 类型也可以接受 &str 类型。
        let s3 = String::from("Hello, s3");
        let t = &s3;
        println!("{}", t);
        println!("{}", &t);

        // str：这是一个不可变的固定长度的字符串类型，通常以切片 &str 的形式出现。常用于固定的、不需要修改的字符串。
        let s4 = "Hello, world! s4";
        println!("{}", s4);


        // String：这是一个动态的，可增长的字符串类型。它被分配在堆上并且可以修改。常用于需要创建或修改字符串的场合。
        let mut mut_s = String::from("hello, ");
        mut_s.push_str("world! mut_s");
        println!("{}", mut_s);
    }

    #[test]
    fn test_os_str() {
        // OsString 和 OsStr：OsString 和 OsStr 是用于处理 OS 字符串的类型。这些类型考虑了不同操作系统中的字符串表示可能不同的事实，
        // 在 Unix 中，字符串是以 \0 结尾的字节序列 (UTF-8)，
        // 在 Windows 中，字符串在宽字节集中则由 16 位的非负整数序列(UTF-16)组成，以\0\0结尾, 比如 CreateFileW 函数。
        //
        // OsString 是可增长的，类似于 String，而 OsStr 是固定长度的，类似于 str。
        let mut  s1 = OsString::from_vec(vec![65, 66, 67]);
        s1.push("s");
        println!("{:?}", s1);

        let s2 = OsStr::new("hello, OsStr");
        println!("{:?}", s2);
    }

    #[test]
    fn test_c_str() {
        // CString 和 CStr：
        // CString 和 CStr 是用于处理 C 字符串的类型。
        // C 字符串是由 null 结束的字节序列。这两种类型常常在调用 C 语言的库函数时使用。
        // CString 是可增长的，类似于 String，而 CStr 是固定长度的，类似于 str。

        let c_string = CString::new("Hello, World!").unwrap();
        println!("{:?}", c_string);

    }

    #[test]
    fn test2() {

    }
}
