use std::cell::RefCell;
use std::rc::Rc;

struct OnlySend<T>
where T: Send {
    value: RefCell<T>
}

unsafe impl<T> Send for OnlySend<T>
where T: Send {
}

struct OnlySync<T>
{
    value: Rc<T>
}

impl <T> OnlySync<T>
where T: Sync {
    fn new(value: T) -> Self {
        Self {
            value: Rc::new(value)
        }
    }

    fn clone(&mut self) -> Self {
        Self {
            value: self.value.clone()
        }
    }

    fn get(&self) -> &T {
        &self.value
    }

    fn get_mut(&mut self) -> &mut T {
        Rc::get_mut(&mut self.value).unwrap()
    }
}

unsafe impl<T> Sync for OnlySync<T>
where T: Sync {
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


unsafe impl Send for SyncAndSend {
}

unsafe impl Sync for SyncAndSend {
}