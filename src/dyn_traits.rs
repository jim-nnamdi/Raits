
struct Teacher {
    name: &'static str,
    age: i32 
}

struct Student {
    name: &'static str,
    age: i32
}

pub trait Person {
    fn name(&self)-> &str;
}

impl Teacher {
    fn new(teacher_name: &'static str, age: i32) -> Self {
        Teacher {name:teacher_name, age}
    }
}

impl Student {
    fn new(student_name: &'static str, age:i32) -> Self {
        Student {name: student_name, age}
    }
}

impl Person for Teacher {
    fn name(&self)-> &str {
        self.name
    }   
}

impl Person for Student {
    fn name(&self)-> &str {
        self.name
    }    
}

#[allow(non_snake_case)]
pub fn Return_Trait(pid: u32, 
    generic_name:&'static str,
     generic_age: i32) -> Box<dyn Person> {
    if pid < 5 {
        Box::new(Teacher {name: generic_name, age:generic_age})
    } else {
        Box::new(Student{name:generic_name, age:generic_age})
    }
}