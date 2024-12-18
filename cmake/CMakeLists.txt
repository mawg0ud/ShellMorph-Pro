# Top-level CMake configuration for ShellMorph Pro

cmake_minimum_required(VERSION 3.26)
project(ShellMorphPro LANGUAGES CXX C)

# Enforce modern C++ standards
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Enable advanced warnings and optimizations
if(CMAKE_BUILD_TYPE MATCHES Release)
    add_compile_options(-O3 -march=native -flto)
elseif(CMAKE_BUILD_TYPE MATCHES Debug)
    add_compile_options(-g -fsanitize=address,undefined)
endif()

# Project-wide settings
set_property(GLOBAL PROPERTY USE_FOLDERS ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Include subdirectories
add_subdirectory(src/core)
add_subdirectory(src/cli)
add_subdirectory(src/api)
add_subdirectory(tests)

# Install configuration
install(TARGETS ShellMorphPro
        RUNTIME DESTINATION bin
        LIBRARY DESTINATION lib
        ARCHIVE DESTINATION lib)

# Generate a documentation target
find_package(Doxygen)
if(DOXYGEN_FOUND)
    add_custom_target(doc
        COMMAND ${DOXYGEN_EXECUTABLE} ${CMAKE_SOURCE_DIR}/docs/Doxyfile
        WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}/docs
        COMMENT "Generating API documentation with Doxygen"
        VERBATIM
    )
endif()
