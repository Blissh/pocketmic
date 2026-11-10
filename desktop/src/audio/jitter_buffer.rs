use ringbuf::{
    traits::{Consumer, Producer, Split},
    HeapRb,
};

pub struct JitterBuffer {
    capacity_samples: usize,
}

pub struct JitterBufferProducer {
    producer: <HeapRb<f32> as Split>::Prod,
}

pub struct JitterBufferConsumer {
    consumer: <HeapRb<f32> as Split>::Cons,
}

impl JitterBuffer {
    pub fn new(capacity_samples: usize) -> (JitterBufferProducer, JitterBufferConsumer) {
        let rb = HeapRb::<f32>::new(capacity_samples);
        let (producer, consumer) = rb.split();
        (
            JitterBufferProducer { producer },
            JitterBufferConsumer { consumer },
        )
    }
}

impl JitterBufferProducer {
    /// Pushes samples into the ring buffer.
    /// Non-blocking, returns the number of samples actually written.
    pub fn push_slice(&mut self, samples: &[f32]) -> usize {
        self.producer.push_slice(samples)
    }

    /// Number of free sample slots available in the buffer.
    pub fn free_len(&self) -> usize {
        self.producer.free_len()
    }
}

impl JitterBufferConsumer {
    /// Pops samples into the destination buffer.
    /// Non-blocking, lock-free, zero heap allocation.
    /// If fewer samples than requested are available (underrun), the remainder is filled with silence (0.0).
    pub fn pop_or_silence(&mut self, dest: &mut [f32]) -> usize {
        let read = self.consumer.pop_slice(dest);
        if read < dest.len() {
            // Zero-fill underrun to prevent static or clicks
            dest[read..].fill(0.0);
        }
        read
    }

    /// Number of samples currently queued and ready to be read.
    pub fn occupied_len(&self) -> usize {
        self.consumer.occupied_len()
    }
}
