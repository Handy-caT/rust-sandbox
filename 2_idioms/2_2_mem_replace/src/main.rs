use std::mem;

fn main() {
    let mut s = Solver {
        expected: Trinity { a: 1, b: 2, c: 3 },
        unsolved: vec![
            Trinity { a: 1, b: 2, c: 3 },
            Trinity { a: 2, b: 1, c: 3 },
            Trinity { a: 2, b: 3, c: 1 },
            Trinity { a: 3, b: 1, c: 2 },
        ],
    };
    s.resolve();
    println!("{:?}", s)
}

#[derive(Clone, Debug, PartialEq)]
struct Trinity<T> {
    a: T,
    b: T,
    c: T,
}

impl<T: Clone> Trinity<T> {
    fn rotate(&mut self) {
        mem::swap(&mut self.a, &mut self.b);
        mem::swap(&mut self.c, &mut self.a);
    }
}

#[derive(Debug)]
struct Solver<T> {
    expected: Trinity<T>,
    unsolved: Vec<Trinity<T>>,
}

impl<T: Clone + PartialEq> Solver<T> {

    fn resolve(&mut self) {
        let mut k = 0;
        for i in 0..self.unsolved.len() {
            let mut ind = false;
            let mut j = 0;
            while j < 3 && !ind {
                if self.unsolved[i-k] == self.expected {
                    self.unsolved.swap_remove(i-k);
                    k += 1;
                    ind = true;
                } else {
                    self.unsolved[i-k].rotate();
                    j+=1;
                }
            }
        }
    }
}
