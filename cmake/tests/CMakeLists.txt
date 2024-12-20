# CMake configuration for the test suite of ShellMorph Pro

# Locate Google Test
include(FetchContent)
FetchContent_Declare(
    googletest
    URL https://github.com/google/googletest/archive/release-1.13.0.zip
)
FetchContent_MakeAvailable(googletest)

# Define test executables
enable_testing()

add_executable(unit_tests
    unit_tests/test_injector.cpp
    unit_tests/test_loader.cpp
)
target_link_libraries(unit_tests PRIVATE gtest_main shellmorph_core)
target_compile_features(unit_tests PRIVATE cxx_std_20)

add_executable(integration_tests
    integration_tests/test_end_to_end.cpp
)
target_link_libraries(integration_tests PRIVATE gtest_main shellmorph_core)
target_compile_features(integration_tests PRIVATE cxx_std_20)

# Register tests
include(GoogleTest)
gtest_discover_tests(unit_tests)
gtest_discover_tests(integration_tests)
