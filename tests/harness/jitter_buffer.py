import numpy as np
from collections import deque
import threading

class JitterBuffer:
    def __init__(self, capacity_samples: int = 4800): # 100 ms at 48kHz
        self.capacity = capacity_samples
        self.queue = deque()
        self.lock = threading.Lock()
        self.total_samples = 0

    def push(self, samples: np.ndarray):
        with self.lock:
            if self.total_samples + len(samples) > self.capacity:
                # Drop oldest to avoid latency buildup
                overflow = (self.total_samples + len(samples)) - self.capacity
                self._drop_samples(overflow)
            self.queue.append(samples.astype(np.float32))
            self.total_samples += len(samples)

    def pop_or_silence(self, count: int) -> np.ndarray:
        out = np.zeros(count, dtype=np.float32)
        filled = 0
        with self.lock:
            while filled < count and self.queue:
                chunk = self.queue[0]
                needed = count - filled
                if len(chunk) <= needed:
                    out[filled:filled + len(chunk)] = chunk
                    filled += len(chunk)
                    self.total_samples -= len(chunk)
                    self.queue.popleft()
                else:
                    out[filled:count] = chunk[:needed]
                    self.queue[0] = chunk[needed:]
                    self.total_samples -= needed
                    filled = count
        return out

    def _drop_samples(self, count: int):
        dropped = 0
        while dropped < count and self.queue:
            chunk = self.queue[0]
            needed = count - dropped
            if len(chunk) <= needed:
                dropped += len(chunk)
                self.total_samples -= len(chunk)
                self.queue.popleft()
            else:
                self.queue[0] = chunk[needed:]
                self.total_samples -= needed
                dropped = count
