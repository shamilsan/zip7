#ifndef LOG_H
#define LOG_H

#include <iostream>
#include <chrono>
#include <iomanip>

#define LOG(message)                                                                                              \
    {                                                                                                             \
        std::chrono::time_point<std::chrono::system_clock> now = std::chrono::system_clock::now();                \
        std::time_t now_time = std::chrono::system_clock::to_time_t(now);                                         \
        std::cout << "[" << std::put_time(std::localtime(&now_time), "%H:%M:%S") << "] " << message << std::endl; \
    }

#endif // LOG_H
