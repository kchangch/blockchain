use std::sync::mpsc;
use std::thread;

pub trait Task {
    type Output: Send;
    fn run(&self) -> Option<Self::Output>;
}

pub struct WorkQueue<TaskType: 'static + Task + Send> {
    send_tasks: Option<spmc::Sender<TaskType>>, // Option because it will be set to None to close the queue
    recv_tasks: spmc::Receiver<TaskType>,
    //send_output: mpsc::Sender<TaskType::Output>, // not need in the struct: each worker will have its own clone.
    recv_output: mpsc::Receiver<TaskType::Output>,
    workers: Vec<thread::JoinHandle<()>>,
}

impl<TaskType: 'static + Task + Send> WorkQueue<TaskType> {
    pub fn new(n_workers: usize) -> WorkQueue<TaskType> {
        // TODO: create the channels; start the worker threads; record their JoinHandles
        let (spmc_send, spmc_recv) = spmc::channel();
        let (mpsc_send, mpsc_recv) = mpsc::channel();

        let mut workers_vec: Vec<thread::JoinHandle<()>> = Vec::new();
        for _ in 0 .. n_workers {
            let sender = mpsc_send.clone();
            let receiver = spmc_recv.clone();
            workers_vec.push(thread::spawn(move || {
                WorkQueue::run(receiver, sender);
            }));
        }

        return WorkQueue {
            send_tasks: Some(spmc_send),
            recv_tasks: spmc_recv,
            recv_output: mpsc_recv,
            workers: workers_vec
        }
    }

    fn run(recv_tasks: spmc::Receiver<TaskType>, send_output: mpsc::Sender<TaskType::Output>) {
        // TODO: the main logic for a worker thread
        loop {
            let task_result = recv_tasks.recv();
            match task_result {
                Ok(task_result) => {
                    send_output.send(task_result.run().unwrap());
                }
                Err(e) => {
                    return;
                }
            }
            // NOTE: task_result will be Err() if the spmc::Sender has been destroyed and no more messages can be received here
        }
    }

    pub fn enqueue(&mut self, t: TaskType) -> Result<(), spmc::SendError<TaskType>> {
        // TODO: send this task to a worker
        self.send_tasks.as_mut().unwrap().send(t)
    }

    // Helper methods that let you receive results in various ways
    pub fn iter(&mut self) -> mpsc::Iter<TaskType::Output> {
        self.recv_output.iter()
    }
    pub fn recv(&mut self) -> TaskType::Output {
        self.recv_output
            .recv()
            .expect("I have been shutdown incorrectly")
    }
    pub fn try_recv(&mut self) -> Result<TaskType::Output, mpsc::TryRecvError> {
        self.recv_output.try_recv()
    }
    pub fn recv_timeout(&self, timeout: std::time::Duration) -> Result<TaskType::Output, mpsc::RecvTimeoutError> {
        self.recv_output.recv_timeout(timeout)
    }

    pub fn shutdown(&mut self) {
        // TODO: destroy the spmc::Sender so everybody knows no more tasks are incoming;
        // drain any pending tasks in the queue; wait for each worker thread to finish.
        // HINT: Vec.drain(..)
        self.send_tasks = None;
        loop {
            let left_over = self.recv_tasks.recv();
            match left_over {
                Ok(task_result) => {
                }
                Err(e) => {
                    break;
                }
            }
        }
        let elements = self.workers.drain(..);
        for x in elements {
            x.join().unwrap()
        }
    }
}

impl<TaskType: 'static + Task + Send> Drop for WorkQueue<TaskType> {
    fn drop(&mut self) {
        // "Finalisation in destructors" pattern: https://rust-unofficial.github.io/patterns/idioms/dtor-finally.html
        match self.send_tasks {
            None => {} // already shut down
            Some(_) => self.shutdown(),
        }
    }
}
