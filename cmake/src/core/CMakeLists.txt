# CMake configuration for the Rust core of ShellMorph Pro

# Add the Rust integration
find_program(CARGO cargo REQUIRED)

# Define core targets
add_custom_target(core ALL
    COMMAND ${CARGO} build --release
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}/src/core
    COMMENT "Building the Rust core with Cargo"
)

# Link the compiled Rust library
set(CORE_LIB_PATH "${CMAKE_SOURCE_DIR}/src/core/target/release")
add_library(shellmorph_core STATIC IMPORTED)
set_target_properties(shellmorph_core PROPERTIES
    IMPORTED_LOCATION "${CORE_LIB_PATH}/libshellmorph_core.a"
)

# Link dependencies
target_include_directories(shellmorph_core INTERFACE "${CMAKE_SOURCE_DIR}/src/core/include")
target_link_libraries(shellmorph_core INTERFACE pthread dl)
