use crate::fact::Fact;

mod fact;

fn main() {
    let fact = Fact::<Vec<u32>>::new();
    println!("{}", fact.fact());
    println!("{}", fact.fact());
    println!("{}", fact.fact());

    let fact = Fact::<String>::new();
    println!("{}", fact.fact());
    println!("{}", fact.fact());
    println!("{}", fact.fact());

    let fact = Fact::<u32>::new();
    println!("{}", fact.fact());
    println!("{}", fact.fact());
    println!("{}", fact.fact());

    let fact = Fact::<()>::new();
    println!("{}", fact.fact());
    println!("{}", fact.fact());
    println!("{}", fact.fact());
}
