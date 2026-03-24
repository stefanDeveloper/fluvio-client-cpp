include(CMakeFindDependencyMacro)

add_library(fluvio_client_cpp::fluvio_client_cpp STATIC IMPORTED)

set_target_properties(fluvio_client_cpp::fluvio_client_cpp PROPERTIES
    IMPORTED_LOCATION "${CMAKE_CURRENT_LIST_DIR}/lib/libfluvio_client_cpp_sys.a"
    INTERFACE_INCLUDE_DIRECTORIES "${CMAKE_CURRENT_LIST_DIR}/include"
)

set_property(TARGET fluvio_client_cpp::fluvio_client_cpp APPEND PROPERTY INTERFACE_LINK_LIBRARIES pthread dl m)
