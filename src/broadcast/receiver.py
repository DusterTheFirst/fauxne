import src.broadcast.channel as broadcast_channel
import uasyncio as asyncio

class BroadcastReceiver:
    id: int
    flag: asyncio.ThreadSafeFlag
    channel: broadcast_channel.BroadcastChannel

    def __init__(self, id: int, flag: asyncio.ThreadSafeFlag, channel: broadcast_channel.BroadcastChannel) -> None:
        self.id = id
        self.flag = flag
        self.channel = channel

    def __del__(self) -> None:
        self.channel.remove_receiver(self.id)

    async def wait(self) -> dict:
        await self.flag.wait()
        return self.channel.message