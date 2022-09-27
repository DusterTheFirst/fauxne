import src.broadcast.receiver as broadcast_receiver
import uasyncio as asyncio
import json

async def handle_events(writer: asyncio.StreamWriter, receiver: broadcast_receiver.BroadcastReceiver):
    writer.write(": welcome\n\n")
    await writer.drain()

    while True:
        message = await receiver.wait()
        print(writer.get_extra_info("peername"), "evt", message)

        try:
            writer.write("data: ")
            writer.write(json.dumps(message))
            writer.write("\n\n")
            await writer.drain()
        except OSError as e:
            print(e)
            break
