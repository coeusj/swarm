use std::{cell::UnsafeCell, mem::MaybeUninit, sync::atomic::{AtomicUsize, Ordering}};

pub struct RingBuffer<T, const SIZE: usize> {
    pub value: [UnsafeCell<MaybeUninit<T>>; SIZE],
    reader_idx: AtomicUsize,
    writer_idx: AtomicUsize,
}

// SAFETY: Shared access across threads is safe because producer
// and consumer manipulate distinct indices enforced by atomic semantics.
unsafe impl<T, const SIZE: usize> Sync for RingBuffer<T, SIZE> {}

impl<T, const SIZE: usize> RingBuffer<T, SIZE> {
    pub fn new() -> Self {
        Self {
            value: [const { UnsafeCell::new(MaybeUninit::uninit()) }; SIZE],
            reader_idx: AtomicUsize::new(0),
            writer_idx: AtomicUsize::new(0),
        }
    }

    pub fn push(&self, value: T) -> Result<(), T> {
        let reader_idx = self.reader_idx.load(Ordering::Acquire);
        let writer_idx = self.writer_idx.load(Ordering::Relaxed);

        // To not lose 1 slot of capacity, let the atomic indeces increment infinitely
        // and only apply '% SIZE' whena accessing array slots
        // # Examples
        // SIZE = 4
        // reader_idx = 4 -> 4 % 4 = 0 (array index)
        // writer_idx = 7 -> 7 % 4 = 3 (array index)
        let slot_idx = writer_idx % SIZE;

        if writer_idx.wrapping_sub(reader_idx) >= SIZE {
            unsafe {
                // Safely drop the evicted value in-place before replacing it
                let slot = &mut *self.value[slot_idx].get();
                slot.assume_init_drop();
            }
            // Advance the reader pointer to evict the old element from logical scope
            self.reader_idx.store(reader_idx.wrapping_add(1), Ordering::Release);
        }

        // SAFETY: Only the producer writes to the 'writer_idx' slot.
        unsafe {
            let slot = &mut *self.value[slot_idx].get();
            slot.write(value);
        }

        self.writer_idx.store(writer_idx.wrapping_add(1), Ordering::Release);
        Ok(())
    }

    pub fn pop(&self) -> Option<T> {
        let reader_idx = self.reader_idx.load(Ordering::Relaxed);
        let writer_idx = self.writer_idx.load(Ordering::Acquire);

        if reader_idx == writer_idx {
            return None;
        }

        // To not lose 1 slot of capacity, let the atomic indeces increment infinitely
        // and only apply '% SIZE' whena accessing array slots
        // # Examples
        // SIZE = 4
        // reader_idx = 4 -> 4 % 4 = 0 (array index)
        // writer_idx = 7 -> 7 % 4 = 3 (array index)
        let slot_idx = reader_idx % SIZE;

        let value = unsafe {
            let slot = &*self.value[slot_idx].get();
            slot.assume_init_read()
        };

        // Attempt CAS update to claim this read step safely in case of race
        match self.reader_idx.compare_exchange_weak(
            reader_idx,
            reader_idx.wrapping_add(1),
            Ordering::Release,
            Ordering::Relaxed,
        ) {
            Ok(_) => return Some(value),
            Err(_) => {
                // Reader index changed (e.g. evicted concurrently by writer).
                // Forget `value` so we don't duplicate ownership, then retry.
                std::mem::forget(value);
                return None;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::{sync::Arc, thread};
    use super::RingBuffer;

    #[test]
    fn drop_oldest() {
        const BUFFER_SIZE: usize = 5;
        let buffer: RingBuffer<Vec<u8>, BUFFER_SIZE> = RingBuffer::new();

        let mut count: u32 = 0;
        while count <= 5 {
            let paylod = count.to_be_bytes().to_vec();
            buffer.push(paylod).unwrap();
            count += 1;
        }

        let expected: u32 = 1;
        assert_eq!(buffer.pop().is_some_and(|x| x == expected.to_be_bytes().to_vec()), true);
    }

    #[test]
    fn reader_thread_reads_all_buffer_items() {
        const BUFFER_SIZE: usize = 5;
        let buffer: Arc<RingBuffer<Vec<u8>, BUFFER_SIZE>> = Arc::new(RingBuffer::new());

        let buffer1 = Arc::clone(&buffer);
        let t1 = thread::spawn(move || {
            let mut count: u32 = 0;
            while count <= 5 {
                let payload = count.to_be_bytes().to_vec();
                buffer1.push(payload).unwrap();
                count += 1;
            }
        });

        let buffer2 = Arc::clone(&buffer);
        let t2 = thread::spawn(move || {
            let mut res = 0;
            while res < 5 {
                if buffer2.pop().is_some() {
                    res += 1;
                } else {
                    // Yield thread execution to avoid burning CPU while waiting
                    thread::yield_now();
                }
            }

            res
        });

        t1.join().unwrap();
        let res = t2.join().unwrap();

        let expected = 5;
        assert_eq!(expected, res);
    }
}