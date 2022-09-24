import src.web_server as web_server
import uasyncio as asyncio

try:
    asyncio.run(web_server.main())
finally:
    asyncio.new_event_loop()
