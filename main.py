from src import core1, core0
import uasyncio as asyncio
import _thread as thread

flags: list[asyncio.ThreadSafeFlag] = []

# Start
thread.start_new_thread(core1.main, (flags,))

# loop: asyncio.Loop = asyncio.new_event_loop()

asyncio.run(core0.main())
