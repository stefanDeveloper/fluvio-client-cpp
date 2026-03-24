set(VERSION "0.0.2")

vcpkg_download_distfile(ARCHIVE
    URLS "https://github.com/stefanDeveloper/fluvio-client-cpp/releases/download/v${VERSION}/fluvio-client-cpp-linux-x64.tar.gz"
    FILENAME "fluvio-client-cpp-linux-x64-${VERSION}.tar.gz"
    SHA512 0
)

vcpkg_extract_source_archive(
    SOURCE_PATH
    ARCHIVE ${ARCHIVE}
)

# Install includes
file(GLOB_RECURSE HEADERS "${SOURCE_PATH}/include/*")
foreach(HEADER ${HEADERS})
    get_filename_component(HEADER_DIR ${HEADER} DIRECTORY)
    string(REPLACE "${SOURCE_PATH}/include" "" RELATIVE_DIR "${HEADER_DIR}")
    file(INSTALL ${HEADER} DESTINATION "${CURRENT_PACKAGES_DIR}/include${RELATIVE_DIR}")
endforeach()

# Install libs
file(INSTALL "${SOURCE_PATH}/lib/libfluvio_client_cpp.a" DESTINATION "${CURRENT_PACKAGES_DIR}/lib")

# Install CMake config
file(INSTALL "${SOURCE_PATH}/fluvio_client_cppConfig.cmake" DESTINATION "${CURRENT_PACKAGES_DIR}/share/fluvio_client_cpp")

vcpkg_install_copyright(FILE_LIST "${SOURCE_PATH}/LICENSE" OPTIONAL)
