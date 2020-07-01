use num_cpus;
mod sum_of_inputs;

fn main() {
    println!("Number of logical cores is {}", sum_of_inputs::sum(32,0));
}
