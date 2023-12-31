cmake_minimum_required(VERSION 3.26)
project(crosu_pp C)

set(CMAKE_C_STANDARD 11)

include_directories(bindings)
link_directories(target/release)
link_libraries(crosu_pp)

SET(SOURCE_FILES tests/test_perf.c)

add_executable(test_crosu_pp ${SOURCE_FILES})
add_custom_command(
        TARGET test_crosu_pp PRE_BUILD
        COMMENT "CREATING DLL"
        COMMAND cargo build --release
        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
        VERBATIM
)

IF (LINUX)
    add_custom_command(
            TARGET test_crosu_pp POST_BUILD
            COMMENT "COPYING DLL TO TO ROOT"
            COMMAND cp -up ./target/release/libcrosu_pp.so .
            WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
            VERBATIM
    )
ELSE ()
    add_custom_command(
            TARGET test_crosu_pp POST_BUILD
            COMMENT "COPYING DLL TO TO ROOT"
            COMMAND cp -up ./target/release/crosu_pp.dll .
            WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
            VERBATIM
    )
ENDIF ()
