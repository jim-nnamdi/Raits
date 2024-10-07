

#[derive(PartialEq)]
struct A <'a> {
    val: &'a str
}

#[derive(PartialEq)]
struct B <'a> {
    val: &'a str
}

impl <'a> A <'a> {
    fn new(new_item : &'a str) -> Self {
        A {val: &new_item}
    }
    fn print_val(&self) -> &'a str {
        self.val
    }
}

impl <'a> B <'a> {
    fn new(new_item : &'a str) -> Self {
        B {val: &new_item}
    }
    fn print_val_b(&self) -> &'a str {
        self.val
    }
}

pub fn cmp_vals(word_one:&'static str, word_two:&'static str) {
    let x = A::new(&word_one).print_val();
    let y =  B::new(&word_two).print_val_b();
    if x.eq(y) {
        println!("{} is equal to  {}", x , y);
    } else {
        print!("{} is not equal to {}",x, y );
    }
}