import machine
import time
import src.broadcast.channel as broadcast_channel

def main(channel: broadcast_channel.BroadcastChannel):
    while True:
        time.sleep(10)
        print(machine.unique_id())
        channel.broadcast({"piss": "poos"})
