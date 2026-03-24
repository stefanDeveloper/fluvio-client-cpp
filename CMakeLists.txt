cmake_minimum_required(VERSION 3.20)
project(fluvio_client_cpp_sys CXX C)

set(CMAKE_CXX_STANDARD 17)

# Enable tasting
enable_testing()

# Normally you would build Rust via FetchContent / Corrosion,
# But since 'cargo build' generates the static library, we'll assume it exists or run it.
execute_process(
    COMMAND cargo build
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

set(RUST_TARGET_DIR "target/debug")
file(GLOB_RECURSE CXXBRIDGE_HEADERS "${RUST_TARGET_DIR}/build/*lib.rs.h")
file(GLOB_RECURSE CXXBRIDGE_SOURCES "${RUST_TARGET_DIR}/build/*lib.rs.cc")
file(GLOB_RECURSE CXX_CORE_HEADER "${RUST_TARGET_DIR}/build/*/cxx.h")
file(GLOB RUST_STATIC_LIB "${RUST_TARGET_DIR}/libfluvio*.a")
list(GET RUST_STATIC_LIB 0 FLUVIO_STATIC_LIB)

list(GET CXXBRIDGE_HEADERS 0 CXXBRIDGE_HEADER)
get_filename_component(CXXBRIDGE_HDR_DIR ${CXXBRIDGE_HEADER} DIRECTORY)
list(GET CXX_CORE_HEADER 0 CXX_CORE_HDR)
get_filename_component(CXX_CORE_DIR ${CXX_CORE_HDR} DIRECTORY)
get_filename_component(CXX_ROOT_DIR ${CXX_CORE_DIR} DIRECTORY)

# Producer Test
add_executable(test_producer tests/test_producer.cpp ${CXXBRIDGE_SOURCES})
target_include_directories(test_producer PRIVATE 
    ${CXXBRIDGE_HDR_DIR}/../.. 
    ${CXX_ROOT_DIR}
)
target_link_libraries(test_producer PRIVATE
    ${FLUVIO_STATIC_LIB}
    pthread dl m
)
add_test(NAME fluvio_producer_test COMMAND test_producer)

# Consumer Test
add_executable(test_consumer tests/test_consumer.cpp ${CXXBRIDGE_SOURCES})
target_include_directories(test_consumer PRIVATE 
    ${CXXBRIDGE_HDR_DIR}/../.. 
    ${CXX_ROOT_DIR}
)
target_link_libraries(test_consumer PRIVATE
    ${FLUVIO_STATIC_LIB}
    pthread dl m
)
add_test(NAME fluvio_consumer_test COMMAND test_consumer)

# Admin Test
add_executable(test_admin tests/test_admin.cpp ${CXXBRIDGE_SOURCES})
target_include_directories(test_admin PRIVATE ${CXXBRIDGE_HDR_DIR}/../.. ${CXX_ROOT_DIR})
target_link_libraries(test_admin PRIVATE
    ${FLUVIO_STATIC_LIB}
    pthread dl m
)
add_test(NAME fluvio_admin_test COMMAND test_admin)

# Config Test
add_executable(test_config tests/test_config.cpp ${CXXBRIDGE_SOURCES})
target_include_directories(test_config PRIVATE ${CXXBRIDGE_HDR_DIR}/../.. ${CXX_ROOT_DIR})
target_link_libraries(test_config PRIVATE
    ${FLUVIO_STATIC_LIB}
    pthread dl m
)
add_test(NAME fluvio_config_test COMMAND test_config)

# Pure C Native Test
add_executable(test_c tests/test_c.c)
target_include_directories(test_c PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/include)
target_link_libraries(test_c PRIVATE
    ${FLUVIO_STATIC_LIB}
    stdc++ pthread dl m
)
add_test(NAME fluvio_c_native_test COMMAND test_c)

# Auth E2E CXX Test
add_executable(test_auth tests/test_auth.cpp ${CXXBRIDGE_SOURCES})
target_include_directories(test_auth PRIVATE ${CXXBRIDGE_HDR_DIR}/../.. ${CXX_ROOT_DIR})
target_link_libraries(test_auth PRIVATE
    ${FLUVIO_STATIC_LIB}
    pthread dl m
)
add_test(NAME fluvio_auth_test COMMAND test_auth)

# Auth E2E C Native Test
add_executable(test_auth_c tests/test_auth_c.c)
target_include_directories(test_auth_c PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/include)
target_link_libraries(test_auth_c PRIVATE
    ${FLUVIO_STATIC_LIB}
    stdc++ pthread dl m
)
add_test(NAME fluvio_auth_c_test COMMAND test_auth_c)
