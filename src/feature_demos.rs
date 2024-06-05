use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use std::time::Duration;


#[derive(Debug)]
struct SomePosition {
    current_position: i32,
}

impl Future for SomePosition {
    type Output = ();

    /**
   [Link](https://medium.com/@mikecode/rust-asynchronous-04-implement-future-964d09e77969)
    If we set current_position value is 1 , poll method will return Pending ,
    so this future will never complete. If we set current_position is 10 ,
    it will return Ready, this future will complete.
    **/
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            if (self.current_position == 10) {
                println!("Ready");
                return Poll::Ready(());
            } else {
                println!("Poll, sleep 1s");
                thread::sleep(Duration::from_secs(1));
                self.current_position += 1;
            }
        }
    }
}

async fn get_position() {
    let sp = SomePosition { current_position: 0 };
    sp.await;
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;
    use crate::feature_demos::get_position;

    #[test]
    fn test1() {
        let f1 = get_position();
        block_on(f1);
    }
}