#include "llhttp.h"
#include "http/request.h"

static int on_url(llhttp_t *parser, const char *at, size_t length) {
    http_raw_request_t *request = request_for_parser(parser);

    chunked_str_push(&request->url, str_from_raw(at, length));

    return 0;
}

static int on_url_complete(llhttp_t *parser) {
    http_raw_request_t *request = request_for_parser(parser);

    // Shrink vector to consume just the amount needed
    // TODO: may be using more cycles to realloc than is worth the spaces
    chunked_str_shrink_to_fit(&request->url);

    return 0;
}

static int on_header_field(llhttp_t *parser, const char *at, size_t length) {
    http_raw_request_t *request = request_for_parser(parser);

    TRACE("on_header_field");

    str_t header_field = str_from_raw(at, length);
    DBG_str(header_field);

    header_name_value_t *header_name_value = header_map_last(&request->headers);
    if (header_name_value == NULL ||
        header_name_value->header_name.complete == true) {
        header_name_value =
            header_map_push(&request->headers, header_name_value_new());
    }

    header_text_t *header_text = &header_name_value->header_name;

    chunked_str_push(&header_text->text, header_field);

    return 0;
}

int on_header_field_complete(llhttp_t *parser) {
    http_raw_request_t *request = request_for_parser(parser);

    TRACE("on_header_field_complete");

    header_name_value_t *header_name_value = header_map_last(&request->headers);
    if (header_name_value == NULL) {
        return ERR_ABRT;
    }

    header_name_value->header_name.complete = true;

    // Shrink vector to consume just the amount needed
    chunked_str_shrink_to_fit(&header_name_value->header_name.text);

    return 0;
}

static int on_header_value(llhttp_t *parser, const char *at, size_t length) {
    http_raw_request_t *request = request_for_parser(parser);

    TRACE("on_header_value");

    str_t header_value = str_from_raw(at, length);
    DBG_str(header_value);

    header_name_value_t *header_name_value = header_map_last(&request->headers);
    if (header_name_value == NULL) {
        header_name_value =
            header_map_push(&request->headers, header_name_value_new());
    }

    header_text_t *header_text =
        header_text_vector_last(&header_name_value->header_values);
    if (header_text == NULL) {
        header_text =
            header_text_vector_push(&header_name_value->header_values,
                                    header_text_new());
    }

    chunked_str_push(&header_text->text, header_value);

    return 0;
}

int on_header_value_complete(llhttp_t *parser) {
    http_raw_request_t *request = request_for_parser(parser);

    TRACE("on_header_value_complete");

    header_name_value_t *header_name_value = header_map_last(&request->headers);
    if (header_name_value == NULL) {
        return ERR_ABRT;
    }

    header_text_t *header_text =
        header_text_vector_last(&header_name_value->header_values);

    if (header_text == NULL) {
        return ERR_ABRT;
    }

    header_text->complete = true;

    // Shrink vector to consume just the amount needed
    chunked_str_shrink_to_fit(&header_text->text);
    header_text_vector_shrink_to_fit(&header_name_value->header_values);

    return 0;
}

int on_headers_complete(llhttp_t *parser) {
    http_raw_request_t *request = request_for_parser(parser);

    TRACE("on_headers_complete");

    // Shrink vector to consume just the amount needed
    header_map_shrink_to_fit(&request->headers);

    return 0;
}

int on_chunk_header(llhttp_t *parser) {
    __unused http_raw_request_t *request = request_for_parser(parser);

    TRACE("on_chunk_header");

    return 0;
}

int on_chunk_complete(llhttp_t *parser) {
    __unused http_raw_request_t *request = request_for_parser(parser);

    TRACE("on_chunk_complete");

    return 0;
}

// int __test_3(__unused llhttp_t *parser, const char *at, size_t length) {
//     http_raw_request_t *request = request_for_parser(parser);

//     return 0;
// }

// int __test_1(__unused llhttp_t *parser) {
//     http_raw_request_t *request = request_for_parser(parser);

//     return 0;
// }

// TODO: implement body
const llhttp_settings_t parser_callbacks = {
    // .on_body = __test_3,
    .on_chunk_complete = on_chunk_header,
    .on_chunk_header = on_chunk_header,
    .on_header_field = on_header_field,
    .on_header_field_complete = on_header_field_complete,
    .on_header_value = on_header_value,
    .on_header_value_complete = on_header_value_complete,
    .on_headers_complete = on_headers_complete,
    // .on_message_begin = __test_1,
    // .on_message_complete = on_message_complete,
    // .on_status = __test_3,
    // .on_status_complete = __test_1,
    .on_url = on_url,
    .on_url_complete = on_url_complete,
};
