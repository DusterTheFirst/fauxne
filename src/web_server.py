import src.broadcast.channel as broadcast_channel
import src.events as events
import uasyncio as asyncio

async def serve_client(reader: asyncio.StreamReader, writer: asyncio.StreamWriter, channel: broadcast_channel.BroadcastChannel) -> None:
    print(reader.get_extra_info("peername"), "Client connected")

    request_line: bytes = await reader.readline()
    request: str = request_line.decode("utf-8").strip()

    # We are not interested in HTTP request headers, skip them
    while await reader.readline() != b"\r\n":
        pass

    [method, url, http] = request.split(" ", 3)
    print(reader.get_extra_info("peername"), "method", method)
    print(reader.get_extra_info("peername"), "url", url)
    print(reader.get_extra_info("peername"), "http", http)

    if method == "GET":
        if url == "/index.html" or url == "/":
            await send_static(writer, "text/html", "index.html")
        elif url == "/js/index.js":
            await send_static(writer, "text/javascript", "js/index.js")
        elif url == "/js/event.js":
            await send_static(writer, "text/javascript", "js/event.js")
        elif url == "/api/events":
            writer.write(http_response("text/event-stream"))
            await writer.drain()
            await events.handle_events(writer, channel.create_receiver())
        else:
            await send_static(writer, "text/html", "404.html", status=(404, "Not Found"))
    else:
        await send_static(writer, "text/html", "405.html", status=(405, "Method Not Allowed"))

    print(reader.get_extra_info("peername"), method, url, "Client disconnected")

def http_response(content_type: str, status: tuple[int, str] = (200, "OK")):
    return f"HTTP/1.0 {status[0]} {status[1]}\r\nContent-type: {content_type}\r\n\r\n"

async def send_static(writer: asyncio.StreamWriter, content_type: str, filename: str, status: tuple[int, str] = (200, "OK")):
    writer.write(http_response(content_type, status))
    with open(f"static_files/{filename}", "r") as file:
        writer.write(file.read())

    await writer.drain()
    await writer.wait_closed()
