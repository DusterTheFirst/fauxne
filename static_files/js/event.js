/**
 *
 * @param {EventSource} event_source
 *
 * @returns {string}
 */
export function event_source_state_name(event_source) {
    switch (event_source.readyState) {
        case EventSource.CONNECTING:
            return "CONNECTING";

        case EventSource.OPEN:
            return "OPEN";

        case EventSource.CLOSED:
            return "CLOSED";
    }
}

/**
 * Subscribe to the SSE endpoint
 *
 * @param {string} source event source url
 */
export function events_subscribe(source = "/api/events") {
    const event_source = new EventSource(source);

    event_source.addEventListener("open", (event) => {
        console.info(
            "event subscriptions connection open",
            event_source_state_name(event_source)
        );
    });
    event_source.addEventListener("error", (event) => {
        console.error(
            "event subscription encountered error",
            event_source_state_name(event_source),
            event
        );
    });
    event_source.addEventListener("message", (event) => {
        console.log("received a message");

        let data;
        try {
            data = JSON.parse(event.data);
        } catch (e) {
            console.warn("non-json data received");
            data = event.data;
        }
        console.dir(data);
        console.dir(event.lastEventId);
        console.dir(event.origin);
    });
    event_source.addEventListener("test", (event) => {
        console.log("TEST!!!", event.data, event.lastEventId);
    });
}
