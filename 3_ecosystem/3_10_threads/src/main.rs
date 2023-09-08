use rand::Rng;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

fn producer() -> Box<[[u8; 64]; 64]> {
    let mut rng = rand::thread_rng();
    let mut result: Box<[[u8; 64]; 64]> = Box::new([[0; 64]; 64]);

    for j in 0..64 {
        let mut row: [u8; 64] = [0; 64];
        for i in 0..64 {
            row[i] = rng.gen();
        }
        result[j] = row;
    };

    result
}

fn consumer(data: Box<[[u8; 64]; 64]>) {
    let sum: u32 = data
        .into_par_iter()
        .map(|row| row.into_par_iter().map(|x| x as u32).sum::<u32>())
        .sum();

    println!("Sum: {}", sum);
}


fn main() {
    let prod = producer();
    consumer(prod);

}
