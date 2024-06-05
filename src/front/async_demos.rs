use futures::executor::block_on;

async fn count_to(count: i32) {
    for i in 0..count {
        println!("count is: {}", i);
    }
}

async fn async_main(count: i32) {
    count_to(count).await;
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;
    use crate::front::async_demos::async_main;

    #[test]
    fn test1() {
        block_on(async_main(30));
        println!("hello, world!")
    }
}