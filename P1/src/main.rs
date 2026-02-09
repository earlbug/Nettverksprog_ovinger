use std::thread;



fn find_primes(from: i32, to: i32, num_threads: i32) -> Vec<i32>{

  let mut primes: Vec<i32> = vec![];


  let mut threads = Vec::new();
  for thread_index in 0..num_threads {
    // Make thread
    threads.push(thread::spawn(move || {
      let mut thread_primes: Vec<i32> = vec![];
      // Repeat for all numbers
      for j in from..=to {
        // Split workload between threads
        if j % num_threads == thread_index {
          if check_if_prime(j) {
            thread_primes.push(j);
          };
        }
      }
      return thread_primes;
    }))
  }

  // Join threads and collect all primes.
  for thread in threads {
    let thread_primes = thread.join().unwrap();
    primes.append(&mut thread_primes.clone());
  }

  return primes;

}


fn check_if_prime(number: i32) -> bool {
  if number < 2 {return false}
  for i in 2..number {
    if number % i == 0 {
      return false;
    }
  }
  return true;
}

fn print_primes (primes: Vec<i32>, from: i32, to: i32) {
  println!("There are {} primes between {} and {}:", primes.len(), from, to);
  for i in 0..primes.len() {
    println!("{}", primes[i]);
  }
}

fn main() {
  let num_threads = 2;
  let from = -5;
  let to = 23;
  let mut primes: Vec<i32> = vec![];
  primes = find_primes(from, to, num_threads);
  primes.sort();
  print_primes(primes, from, to);
}


