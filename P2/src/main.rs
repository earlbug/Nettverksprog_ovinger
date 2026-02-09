use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;


type Task = Box<dyn FnOnce() + Send + 'static>;

struct SharedState {
    tasks: Vec<Task>,
    stopped: bool,
}

// Taskpool only for each instance of the Worker thread,
// to be able choose what type of task is best suited for each thread
struct Workers {
    num_of_worker_threads: i32,
    shared_state: Arc<(Mutex<SharedState>, Condvar)>,
    threads: Vec<JoinHandle<()>>,
}

impl Workers {
    fn new(num_of_worker_threads: i32) -> Self {
        let shared_state = SharedState {
            tasks: Vec::new(),
            stopped: false,
        };
        Self {
            num_of_worker_threads,
            shared_state: Arc::new((Mutex::new(shared_state), Condvar::new())),
            threads: Vec::new(),
        }
    }


    fn start(mut self) -> Self {
        let mut threads = Vec::new();

        for _ in 0..self.num_of_worker_threads {
            // new arc instance, which pints to the same place on the heap as the parenthesis
            let shared_state = Arc::clone(&self.shared_state);

            let handle = thread::spawn(move || {
                let (lock, cvar) = &*shared_state;

                loop {
                    // wait for a task
                    let mut guard = lock.lock().unwrap();
                    while guard.tasks.is_empty() && !guard.stopped {
                        guard = cvar.wait(guard).unwrap();
                    }

                    // if stopped and no tasks left, exit
                    if guard.stopped && guard.tasks.is_empty() {
                        break;
                    }


                    // we know there is at least one task
                    let task = guard.tasks.pop().unwrap();
                    drop(guard); // release lock while running the task
                    task();
                }
            });

            threads.push(handle);
        }

        self.threads = threads;
        self
    }

    fn post<F>(&self, f: F)
    where
      F: FnOnce() + Send + 'static,
    {
        let (lock, cvar) = &*self.shared_state;
        let mut shared_state = lock.lock().unwrap();

        if shared_state.stopped {
            return;
        }

        shared_state.tasks.push(Box::new(f));
        // wake one sleeping worker
        cvar.notify_one();
    }

    fn post_timeout<F>(&self, f: F, delay_ms: i32)
    where
      F: FnOnce() + Send + 'static,
    {
        let (lock, cvar) = &*self.shared_state;
        let mut shared_state = lock.lock().unwrap();

        if shared_state.stopped {
            return;
        }

        // wrap the original task so the delay is part of the task itself
        let delay = delay_ms.max(0) as u64;
        let wrapped_task = Box::new(move || {
            thread::sleep(Duration::from_millis(delay));
            f();
        }) as Task;

        shared_state.tasks.push(wrapped_task);
        cvar.notify_one();
    }

    fn stop(self) {
        let (lock, cvar) = &*self.shared_state;

        // set stopped flag and wake all workers
        {
            let mut shared_state = lock.lock().unwrap();
            shared_state.stopped = true;
            cvar.notify_all();
        }

        // join all worker threads
        for handle in self.threads {
            let _ = handle.join();
        }
    }
}



fn main() {
    let worker = Workers::new(2).start();
    let worker2 = Workers::new(1).start();

    worker.post(|| {
        println!("starting worker task 1");

        for i in 0..100000 {
            for j in 0..10000 {
                let temp:i32 = i*j;
            }
        }
        println!("wirker task1 finished");
    });


    worker.post( || {
        println!("starting worker task 2");

        for i in 0..100000 {
            for j in 0..10000 {
                let temp:i32 = i*j;
            }
        }
        println!("worker task 2 finished");

    });

    worker2.post_timeout(|| println!("loop task 1"), 1000);
    let x = 42;
    worker2.post_timeout(move || println!("loop task 2. x = {}", x), 1000);

    worker.stop();
    worker2.stop();
}