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

fn consumer(data: Box<[[u8; 64]; 64]>, id: usize) {
    let sum: u32 = data
        .into_par_iter()
        .map(|row| row.into_par_iter().map(|x| x as u32).sum::<u32>())
        .sum();

    println!("Sum from {}: {}",id, sum);
}


fn main() {
    // let prod = producer();
    // consumer(prod);

    let (sender, receiver) = crossbeam_channel::bounded(1);

    let handles = (0..2).map(|i| {
        let receiver = receiver.clone();
        std::thread::spawn(move || {
            while let Ok(data) = receiver.recv() {
                consumer(data, i);
            }
        })
    }).collect::<Vec<_>>();

    loop {
        let data = producer();
        sender.send(data).unwrap();
    }
}
