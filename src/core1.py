import machine
from time import sleep
from uasyncio import ThreadSafeFlag

def main(flags: list[ThreadSafeFlag]):
    sleep(10)
    print(machine.unique_id())
