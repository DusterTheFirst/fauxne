import machine
import src.broadcast.channel as broadcast_channel
import src.access_point as access_point
import src.web_server as web_server
import uasyncio as asyncio

onboard = machine.Pin("LED", machine.Pin.OUT)

async def main(channel: broadcast_channel.BroadcastChannel):
    print('Setting up Access Point...')
    access_point.AccessPoint().setup()

    print('Setting up web server...')
    asyncio.create_task(asyncio.start_server(lambda reader, writer: web_server.serve_client(reader, writer, channel), "0.0.0.0", 80))
    
    while True:
        onboard.on()
        print("web server running...")
        await asyncio.sleep(0.25)
        onboard.off()
        await asyncio.sleep(5)