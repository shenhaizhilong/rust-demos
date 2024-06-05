// https://medium.com/@mbugraavci38/mastering-iterators-in-rust-enhancing-performance-and-productivity-09dca73f17d2
struct Fib {
    curr: u64,
    next: u64,
}

impl Iterator for Fib {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

#[cfg(test)]
mod tests {
    use crate::iter_demo::Fib;

    #[test]
    fn test1() {
        let fib = Fib {curr: 1, next: 1};
        for n in fib.take(10) {
            println!("n = {}", n);
        }
    }
}