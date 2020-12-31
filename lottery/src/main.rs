use rand::Rng;

fn main() {
    let mut loop_number = 0;

    while loop_number < 6 {
        let secret_number = rand::thread_rng().gen_range(0..10);
        println!("Secret number is: {}", secret_number);

        loop_number += 1;
    }
}
