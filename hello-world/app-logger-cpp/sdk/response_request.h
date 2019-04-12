#ifndef FLUENCE_C_SDK_RR_H
#define FLUENCE_C_SDK_RR_H
#include "allocator.h
#include <string>
#include <cstdint>

template <typename T>
T read_request(void *ptr, size_t)

template<>
std::string read_request(void *ptr, size_t) {

}

template<>
std::vector<uint8_t> read_request(void *ptr, size_t) {

}

template <typename T>
uintptr_t write_response(T arg);

template <typename T>
uintptr_t write_response(std::string &arg) {

}

template <typename T>
uintptr_t write_response(std::string &&arg) {

}

template <typename T>
uintptr_t write_response(std::vector &arg) {

}

template <typename T>
uintptr_t write_response(std::vector &&arg) {

}

#endif //FLUENCE_C_SDK_RR_H
