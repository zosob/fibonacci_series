fn main() {
    let fibonacci_series = fibonacci(15);
    println!("{:?}", fibonacci_series);
}

fn fibonacci(n:usize)-> Vec<u32> {
    let mut solution = vec![0,1];
    
    for i in 2..n{
        let next_fib = solution[i-1] + solution[i-2];
        solution.push(next_fib);
    }

    solution

}
