#include "log.h"
#include "lwip/apps/fs.h"
#include "lwip/apps/httpd.h"
#include "lwip/api.h"
#include "string.h"

void httpd_cgi_handler(struct fs_file *file, const char *uri, int iNumParams,
                       char **pcParam, char **pcValue, void *state) {
    TRACE("httpd_cgi_handler");

    DBG("%s", uri);
    DBG("%d", iNumParams);
    DBG("%s", file->data);
    DBG("%d", file->flags);
    DBG("%d", file->index);
    DBG("%d", file->len);
    DBG("%p", pcParam);
    DBG("%s", pcParam[0]);
    DBG("%p", pcValue);
    DBG("%s", pcValue[0]);
    DBG("%p", state);
}

// Called first for every opened file to allow opening files that are not
// included in fsdata(_custom).c
int fs_open_custom(struct fs_file *file, const char *name) {
    TRACE("fs_open_custom");

    if (strcmp(name, "/index.html")) {
        TRACE("skipping %s", name);
        return false;
    }

    DBG("%s", name);

    DBG("%s", file->data);
    DBG("%d", file->flags);
    DBG("%d", file->index);
    DBG("%d", file->len);

    // static const char http_response[] =
    //     "HTTP/1.1 200 OK\r\n"
    //     // "Content-Length: 21\r\n"
    //     "Content-Type: text/event-stream\r\n"
    //     // "Connection: keep-alive\r\n"
    //     // "Keep-Alive: timeout=5, max=1000\r\n"
    //     "\r\n"
    //     "<h1>hello diana</h1>";
    // file->data = http_response;
    // file->len = sizeof(http_response) - 1;
    // file->index = 1;

    return true;
}

// Called to free resources allocated by fs_open_custom().
void fs_close_custom(struct fs_file *file) {
    TRACE("fs_close_custom");

    DBG("%s", file->data);
    DBG("%d", file->flags);
    DBG("%d", file->index);
    DBG("%d", file->len);
}

/** This user-defined function is called when a file is opened. */
void *fs_state_init(struct fs_file *file, const char *name) {
    TRACE("fs_state_init");

    DBG("%s", file->data);
    DBG("%d", file->flags);
    DBG("%d", file->index);
    DBG("%d", file->len);

    DBG("%s", name);

    return NULL;
}

/** This user-defined function is called when a file is closed. */
void fs_state_free(struct fs_file *file, void *state) {
    TRACE("fs_state_free");

    DBG("%s", file->data);
    DBG("%d", file->flags);
    DBG("%d", file->index);
    DBG("%d", file->len);

    DBG("%p", state);

    free(state);
}

uint8_t fs_canread_custom(__unused struct fs_file *file) {
    TRACE("fs_canread_custom");

    return false;
}
uint8_t fs_wait_read_custom(__unused struct fs_file *file,
                            __unused fs_wait_cb callback_fn,
                            __unused void *callback_arg) {
    TRACE("fs_wait_read_custom");

    return FS_READ_DELAYED;
}

int fs_read_custom(struct fs_file *file, char *buffer, int count) {
    TRACE("fs_read_custom");

    DBG("%s", file->data);
    DBG("%d", file->flags);
    DBG("%d", file->index);
    DBG("%d", file->len);

    DBG("%p", buffer);

    DBG("%d", count);

    return FS_READ_DELAYED;
}
