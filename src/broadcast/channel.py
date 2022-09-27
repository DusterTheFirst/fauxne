import src.broadcast.receiver as broadcast_receiver
import uasyncio as asyncio
import random
import sys 
import _thread as thread

class BroadcastChannel:
    flags: dict[int, asyncio.ThreadSafeFlag]
    message: dict

    flags_lock: thread.LockType

    def __init__(self) -> None:
        self.flags = {}
        self.flags_lock = thread.allocate_lock()

    def broadcast(self, message: dict) -> None:
        with self.flags_lock:
            self.message = message

            for id, flag in self.flags.items():
                flag.set()

    def create_receiver(self) -> broadcast_receiver.BroadcastReceiver:
        flag = asyncio.ThreadSafeFlag()
        id = random.randint(0, sys.maxsize)

        with self.flags_lock:
            self.flags[id] = flag

        return broadcast_receiver.BroadcastReceiver(id, flag, self)

    def remove_receiver(self, id: int):
        with self.flags_lock:
            del self.flags[id]