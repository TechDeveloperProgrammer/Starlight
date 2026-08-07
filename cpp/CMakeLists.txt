cmake_minimum_required(VERSION 3.25)

project(
    StarlightBedrockAdapter
    VERSION 0.1.0
    LANGUAGES CXX
)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

# Options
option(BUILD_SHARED_LIBS "Build shared libraries" ON)
option(STARLIGHT_ENABLE_TESTS "Enable tests" ON)
option(STARLIGHT_ENABLE_SANITIZERS "Enable sanitizers" OFF)

# Version from git tag or manual override
if(DEFINED ENV{STARLIGHT_VERSION})
    set(PROJECT_VERSION $ENV{STARLIGHT_VERSION})
endif()

# Compiler flags
if(CMAKE_CXX_COMPILER_ID MATCHES "GNU|Clang")
    add_compile_options(-Wall -Wextra -Wpedantic)
    if(STARLIGHT_ENABLE_SANITIZERS)
        add_compile_options(-fsanitize=address,undefined -fno-omit-frame-pointer)
        add_link_options(-fsanitize=address,undefined)
    endif()
endif()

# Main library
add_library(starlight_bedrock_adapter
    src/main.cpp
    # src/protocol.cpp
    # src/packet_handler.cpp
    # src/translator.cpp
)

target_include_directories(starlight_bedrock_adapter
    PUBLIC
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
        $<INSTALL_INTERFACE:include>
)

target_compile_features(starlight_bedrock_adapter PUBLIC cxx_std_20)

# Executable (optional)
add_executable(starlight_bedrock_adapter_bin
    src/main.cpp
)

target_link_libraries(starlight_bedrock_adapter_bin
    PRIVATE starlight_bedrock_adapter
)

set_target_properties(starlight_bedrock_adapter_bin PROPERTIES
    OUTPUT_NAME starlight_bedrock_adapter
)

# Install targets
install(
    TARGETS starlight_bedrock_adapter_bin
    RUNTIME DESTINATION bin
)

install(
    TARGETS starlight_bedrock_adapter
    LIBRARY DESTINATION lib
    ARCHIVE DESTINATION lib
    RUNTIME DESTINATION bin
)

install(
    DIRECTORY ../packaging/bedrock/config/
    DESTINATION config
    OPTIONAL
)

install(
    FILES ../packaging/bedrock/manifest.json
    DESTINATION .
    OPTIONAL
)

# CPack configuration for packaging
set(CPACK_PACKAGE_NAME "starlight-bedrock-adapter")
set(CPACK_PACKAGE_VENDOR "Starlight Core Team")
set(CPACK_PACKAGE_VERSION ${PROJECT_VERSION})
set(CPACK_PACKAGE_DESCRIPTION_SUMMARY "Bridge adapter for Minecraft Bedrock Edition")
set(CPACK_PACKAGE_HOMEPAGE_URL "https://github.com/TU_ORGANIZACION/starlight-core")
set(CPACK_GENERATOR "ZIP;TGZ")
set(CPACK_PACKAGE_FILE_NAME "${CPACK_PACKAGE_NAME}-${CPACK_PACKAGE_VERSION}-linux-x64")
set(CPACK_ARCHIVE_COMPONENT_INSTALL ON)

# Platform-specific settings
if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    set(CPACK_PACKAGE_FILE_NAME "${CPACK_PACKAGE_NAME}-${CPACK_PACKAGE_VERSION}-linux-x64")
elseif(CMAKE_SYSTEM_NAME STREQUAL "Windows")
    set(CPACK_PACKAGE_FILE_NAME "${CPACK_PACKAGE_NAME}-${CPACK_PACKAGE_VERSION}-windows-x64")
    set(CPACK_GENERATOR "ZIP")
endif()

include(CPack)

# Tests
if(STARLIGHT_ENABLE_TESTS)
    enable_testing()
    
    add_executable(starlight_tests
        src/tests/test_main.cpp
    )
    
    target_link_libraries(starlight_tests
        PRIVATE starlight_bedrock_adapter
    )
    
    add_test(
        NAME starlight_unit_tests
        COMMAND starlight_tests
    )
endif()
