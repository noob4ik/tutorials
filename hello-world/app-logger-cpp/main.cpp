#include "sdk.h"
#include <string.h>

const std::string greeting = "Hello world! From ";

char *invoke(const char *str, int length) {
    const std::string request = read_request<std::string>(str, length);
    const std::string response = greeting + request;

    return write_response(std::move(response));
}

