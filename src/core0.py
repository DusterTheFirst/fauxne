
from machine import Pin, Timer
from src.access_point import AccessPoint
from src.web_server import serve_client
import uasyncio as asyncio

onboard = Pin("LED", Pin.OUT)

async def main():
    print('Setting up Access Point...')
    AccessPoint().setup()

    print('Setting up web server...')
    asyncio.create_task(asyncio.start_server(serve_client, "0.0.0.0", 80))
    
    while True:
        onboard.on()
        print("web server running...")
        await asyncio.sleep(0.25)
        onboard.off()
        await asyncio.sleep(5)