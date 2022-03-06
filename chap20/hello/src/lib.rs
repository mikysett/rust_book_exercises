use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;

pub struct ThreadPool {
	workers: Vec<Worker>,
	sender: mpsc::Sender<Message>,
}

enum Message {
	NewJob(Job),
	Terminate,
}

impl ThreadPool {
	/// Create a new ThreadPool.
	/// 
	/// The size is the number of workers in the pool.
	/// If the size is zero new return a PoolCreationError.
	pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
		if size == 0 {
			Err(PoolCreationError)
		} else {
			let mut workers = Vec::with_capacity(size);
			let (sender, receiver) = mpsc::channel();
			let receiver = Arc::new(Mutex::new(receiver));

			for id in 0..size {
				workers.push(Worker::new(id, Arc::clone(&receiver)));
			}
			Ok(ThreadPool {
				workers,
				sender,
			})
		}
	}

	pub fn execute<F>(&self, f: F)
		where F: FnOnce() + Send + 'static
	{
		let job = Box::new(f);

		self.sender.send(Message::NewJob(job)).unwrap();
	}
}

impl Drop for ThreadPool {
	fn drop(&mut self) {
		println!("Send message terminate to all workers");

		for _ in &self.workers {
			self.sender.send(Message::Terminate).unwrap();
		}

		println!("Shutting down all workers");

		for worker in &mut self.workers {
			println!("Shutting down worker {}", worker.id);

			if let Some(thread) = worker.thread.take() {
				thread.join().unwrap();
			}
		}
	}
}

pub struct PoolCreationError;

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
	id: usize,
	thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
	fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
		Worker {
			id,
			thread: Some(thread::spawn(move || loop {
				let job = receiver.lock().unwrap().recv().unwrap();

				match job {
					Message::NewJob(job) => {
						println!("Worker {} got a job, executing...", id);
						
						job();
					},
					Message::Terminate => {
						println!("Worker {} terminating...", id);
						
						break;
					}
				}
			})),
		}
	}
}
