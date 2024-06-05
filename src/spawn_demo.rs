#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;

    /**
     Rust’s ownership model guarantees that each thread has exclusive access to the necessary resources,
    effectively preventing data races and other concurrency-related bugs.

    Rust 所有权模型保证每个线程都可以独占访问必要的资源，从而有效防止数据竞争和其他与并发相关的错误。
    **/
    #[test]
    fn test1() {
        let mut handles = vec![];
        let mut x: i32 = 0;

        for i in 0..10 {
            handles.push(thread::spawn(move || {
                x += 1;
                println!("Hello from thread {} with x = {}", i, x);
            }))
        }

        for thread in handles {
            thread.join().unwrap();
        }
    }

    #[test]
    fn test() {
        // 创建一个共享的可变变量
        let counter = Arc::new(Mutex::new(0));
        // 创建多个线程
        let mut handles = vec![];

        for _ in 0..5 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                // 在这个线程中，获取互斥锁，对共享变量进行操作
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }

        // 等待所有线程结束
        for handle in handles {
            handle.join().unwrap();
        }

        // 打印最终结果
        println!("Result: {}", *counter.lock().unwrap());
    }

    #[test]
    fn test2() {
        let v = Arc::new(vec![10, 20, 30]);
        let mut handles = Vec::new();
        for i in 1..5 {
            let v = Arc::clone(&v);
            handles.push(thread::spawn(move || {
                let thread_id = thread::current().id();
                println!("{thread_id:?}: {v:?}");
            }));
        }

        handles.into_iter().for_each(|h| h.join().unwrap());
        println!("v: {v:?}");

    }

    #[test]
    fn test3() {
        let v = Mutex::new(vec![10, 20, 30]);
        println!("v: {:?}", v.lock().unwrap());
        {
            let mut guard = v.lock().unwrap();
            guard.push(40);
        }

        println!("v: {:?}", v.lock().unwrap());
    }
}