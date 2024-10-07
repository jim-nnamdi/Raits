#[allow(dead_code)]
struct Sheep { naked: bool, name: &'static str}

#[allow(dead_code)]
trait Animal {
    fn new(name:&'static str) -> Self;
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    fn talk(&self){
        println!("{} says {}", self.name(), self.noise());
    }
}
#[allow(dead_code)]
impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }
    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is naked", self.name());
        } else {
            println!("{} gets a haircut", self.name);
        }
    }
}

impl Animal for Sheep {
    fn name(&self) -> &'static str {
        self.name
    }
    fn new(name:&'static str) -> Self {
        Sheep {naked: false, name}
    }
    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "blaaah"
        } else {
            "meowww"
        }
    }
}
