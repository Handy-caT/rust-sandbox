use std::cell::RefCell;
use std::rc::Rc;
use std::sync::MutexGuard;

struct OnlySend<T>
where T: Send {
    value: RefCell<T>
}

struct NotSyncNotSend<T> {
    value: Rc<T>
}

impl <T> NotSyncNotSend<T> {
    fn new(value: T) -> Self {
        Self {
            value: Rc::new(value)
        }
    }

    fn get(&self) -> &T {
        &self.value
    }
}

struct SyncAndSend {
    value: u32
}


#[derive(Debug)]
struct SyncAndNotSend {
    value: u32,
    phantom: std::marker::PhantomData<MutexGuard<'static, u32>>
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::sync::Arc;
    use std::thread;
    use crate::types::{OnlySend, SyncAndNotSend, SyncAndSend};

    #[test]
    fn test_sync_and_send() {
        let sync_and_send = SyncAndSend { value: 1 };

        let handle = thread::spawn(move || {
            println!("sync_and_send send: {}", sync_and_send.value);
        });
        handle.join().unwrap();

        let reference = Arc::new(sync_and_send);

        let handle = thread::spawn(move || {
            println!("sync_and_send sync: {}", reference.value);
        });
        handle.join().unwrap();
    }

    #[test]
    fn test_only_send() {
        let only_send = OnlySend { value: RefCell::new(1) };

        let handle = thread::spawn(move || {
            println!("only_send send: {:?}", only_send.value);
        });
        handle.join().unwrap();
    }
}