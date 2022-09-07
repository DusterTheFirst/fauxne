/**
 * Subscribe to the SSE endpoint
 */
export function events_subscribe() {
    const event_source = new EventSource("/api/events");

    event_source.addEventListener("open", (event) => {
        console.info("event subscriptions connection open");
    });
    event_source.addEventListener("error", (event) => {
        console.error("event subscription encountered error");
    });
    event_source.addEventListener("message", (event) => {
        console.log("received a message");
        console.dir(event.data, event.lastEventId, event.origin, event);
    });
    event_source.addEventListener("test", (event) => {});
}
