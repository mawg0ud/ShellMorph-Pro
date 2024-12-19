# CMake configuration for the CLI component of ShellMorph Pro

add_executable(shellmorph-cli main.cpp arguments_parser.cpp)
target_include_directories(shellmorph-cli PRIVATE "${CMAKE_SOURCE_DIR}/src/core/include")
target_link_libraries(shellmorph-cli PRIVATE shellmorph_core)

# Apply modern tooling
target_compile_options(shellmorph-cli PRIVATE -Wall -Wextra -Wpedantic)
target_compile_features(shellmorph-cli PRIVATE cxx_std_20)

# Install the CLI executable
install(TARGETS shellmorph-cli RUNTIME DESTINATION bin)
