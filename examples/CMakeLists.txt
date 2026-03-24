cmake_minimum_required(VERSION 3.20)
project(fluvio_examples CXX)

set(CMAKE_CXX_STANDARD 17)

# VCPKG dependencies
find_package(nlohmann_json CONFIG REQUIRED)
find_package(fmt CONFIG REQUIRED)

# The downloaded release via FetchContent
include(FetchContent)
FetchContent_Declare(
  fluvio_client_cpp
  URL https://github.com/stefanDeveloper/fluvio-client-cpp/releases/download/v0.0.2/fluvio-client-cpp-linux-x64.tar.gz
)
FetchContent_MakeAvailable(fluvio_client_cpp)

find_package(fluvio_client_cpp CONFIG REQUIRED PATHS ${fluvio_client_cpp_SOURCE_DIR})

add_executable(producer_example producer.cpp)
target_link_libraries(producer_example PRIVATE nlohmann_json::nlohmann_json fmt::fmt fluvio_client_cpp::fluvio_client_cpp)

add_executable(consumer_example consumer.cpp)
target_link_libraries(consumer_example PRIVATE nlohmann_json::nlohmann_json fmt::fmt fluvio_client_cpp::fluvio_client_cpp)
