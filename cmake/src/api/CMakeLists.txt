# CMake configuration for the API component of ShellMorph Pro

find_program(NPM npm REQUIRED)

# Define API targets
add_custom_target(api ALL
    COMMAND ${NPM} install
    COMMAND ${NPM} run build
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}/src/api
    COMMENT "Building the API with npm"
)

# Install the API
install(DIRECTORY ${CMAKE_SOURCE_DIR}/src/api/build/
        DESTINATION ${CMAKE_INSTALL_PREFIX}/api)
