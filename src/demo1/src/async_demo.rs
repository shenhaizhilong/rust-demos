// https://rust-lang.github.io/async-book/01_getting_started/02_why_async.html

#[cfg(test)]
mod async_demo {
    use std::future::Future;
    use std::os::unix::raw::mode_t;
    use futures::executor::block_on;

    pub struct Song {
        pub name: String,
        pub publish_date: String,
    }

    async fn sing_song(song: Song) {
        let song_name = song.name;
        println!("sing_song: {song_name:?}");
    }

    async fn learn_song(name: String) -> Song {
        Song {
            name: name.to_string(),
            publish_date: "2020-03".to_string(),
        }
    }

    async fn dance() {
        println!("dancing");
    }


    async fn learn_and_sing(song_name: String) {
        let song = learn_song(song_name).await;
        sing_song(song).await;
    }

    async fn test() {
        let f1 = learn_and_sing("爱你一万年".to_string());
        let f2 = dance();
        futures::join!(f1, f2);
    }

    async fn async_hello() {
        println!("hello, rust async");
    }


    async fn fib(n: i32) -> i32 {
        if (n == 0 || n == 1) {
            return n;
        }
        return Box::pin(fib(n - 1)).await + Box::pin(fib(n - 2)).await;
    }

    async fn blocks() {
        let my_string = "foo".to_string();

        let future_one = async {
            // ...
            println!("{my_string}");
        };

        let future_two = async {
            // ...
            println!("{my_string}");
        };

        // Run both futures to completion, printing "foo" twice:
        let ((), ()) = futures::join!(future_one, future_two);
    }

    fn move_block() -> impl Future<Output = ()> {
        let my_string = "foo".to_string();
        async move {
            // ...
            println!("{my_string}");
        }
    }

    #[test]
    fn test1() {
        block_on(test());
        block_on(async_hello());
    }

    #[test]
    fn test2() {
        let n = 6;
        let v1 = async { fib(4).await };
        let v2 = async { fib(5).await };
        let v3 = async { fib(n).await };

        println!("{}", block_on(v1));
        println!("{}", block_on(v2));
        println!("{}, {}", n, block_on(v3));
    }
}