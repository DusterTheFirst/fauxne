import network
import rp2

from machine import Pin, Timer
import uasyncio as asyncio

onboard = Pin("LED", Pin.OUT)

html = """<!DOCTYPE html>
<html>
<head> <title>Pico W</title> </head>
<body> <h1>Pico W</h1>
<p>%s</p>
</body>
</html>"""

rp2.country("NL")

class AccessPoint:
    wlan = network.WLAN(network.AP_IF)

    def setup(self) -> None:
        self.wlan.config(ssid="fauxne", key="fake phone")
        self.wlan.active(True)
        # self.wlan.ifconfig(('192.168.0.1', '255.255.255.0', '192.168.0.1', '8.8.8.8'))

        status = self.wlan.ifconfig()
        print('ip = ' + status[0])
        print(status)
    

async def serve_client(reader: asyncio.StreamReader, writer: asyncio.StreamWriter) -> None:
    print("Client connected")

    request_line: bytes = await reader.readline()
    request = str(request_line)

    print("Request:", request)
    # We are not interested in HTTP request headers, skip them
    while await reader.readline() != b"\r\n":
        pass

    led_on = request.find('/light/on')
    led_off = request.find('/light/off')
    print( 'led on = ' + str(led_on))
    print( 'led off = ' + str(led_off))

    stateis = ""
    if led_on == 6:
        print("led on")
        stateis = "LED is ON"

    if led_off == 6:
        print("led off")
        stateis = "LED is OFF"

    response = html % stateis
    writer.write('HTTP/1.0 200 OK\r\nContent-type: text/html\r\n\r\n')
    writer.write(response)

    await writer.drain()
    await writer.wait_closed()
    print("Client disconnected")

async def main():
    print('Connecting to Network...')
    AccessPoint().setup()

    print('Setting up web server...')
    asyncio.create_task(asyncio.start_server(serve_client, "0.0.0.0", 80))
    
    while True:
        onboard.on()
        print("heartbeat")
        await asyncio.sleep(0.25)
        onboard.off()
        await asyncio.sleep(5)
