

pub struct Fibonacci {
    curr: u32,
    next: u32
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        self.curr = self.next;
        self.next = current +  self.next;
        Some(current)
    }
}

#[allow(dead_code)]
pub fn fib() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}