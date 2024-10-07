use std::ops;

struct FirstFrequency {val:u32}
struct SecondFrequency {val: u32}

#[derive(Debug)]
struct BaseFreq {val: u32}

#[derive(Debug)]
struct BaseFreq2 {val: u32}

impl ops::Add<FirstFrequency> for BaseFreq {
    type Output = BaseFreq;
    fn add(self, rhs: FirstFrequency) -> Self::Output {
        let base_addition = self.val + rhs.val;
        BaseFreq {val:base_addition}
    }
}

impl ops::Add<SecondFrequency> for BaseFreq2 {
    type Output = BaseFreq2;
    fn add(self, rhs: SecondFrequency) -> Self::Output {
        let base2_addition = self.val + rhs.val;
        BaseFreq2{val:base2_addition}
    }
}

pub fn add_double_structs() {
    let first = FirstFrequency {val: 10};
    let second = SecondFrequency {val: 20};

    let base = BaseFreq {val: 30};
    let base2 = BaseFreq2 {val: 40};

    let result1 = base + first;
    let result2 = base2 + second;

    println!("BaseFreq + FirstFrequency = {:?}", result1);
    println!("BaseFreq2 + SecondFrequency = {:?}", result2);
}