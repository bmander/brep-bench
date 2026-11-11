# The brep-bench runner for SMLib through its C++ API. Built by build.py, which fetches the
# headers and libraries this names first.
cmake_minimum_required(VERSION 3.16)
project(brep-bench-smlib-cpp CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(USD_BREP_VERSION "unknown" CACHE STRING "the usd-brep release the runner is built against")

set(UPSTREAM ${CMAKE_CURRENT_SOURCE_DIR}/upstream)
# every implementation registers itself; the runner looks the case up by its id
file(GLOB_RECURSE CASES CONFIGURE_DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/src/cases/*.cpp)
add_executable(brep-bench-smlib-cpp src/runner.cpp src/bench.cpp ${CASES})
target_include_directories(brep-bench-smlib-cpp PRIVATE
    src ${UPSTREAM}/usd-brep/source/SM_API/inc ${UPSTREAM}/usd-brep/source/SMLib/inc)
target_compile_definitions(brep-bench-smlib-cpp PRIVATE USD_BREP_VERSION="${USD_BREP_VERSION}")
# SMLib's headers are old and loud; the runner's own code is held to -Wall.
target_compile_options(brep-bench-smlib-cpp PRIVATE -Wall -Wno-unused-variable
    -Wno-unknown-pragmas -Wno-deprecated-declarations -Wno-class-memaccess -Wno-overloaded-virtual
    -Wno-reorder -Wno-sign-compare -Wno-unused-but-set-variable)
target_link_directories(brep-bench-smlib-cpp PRIVATE ${UPSTREAM}/lib)
target_link_libraries(brep-bench-smlib-cpp PRIVATE SM_API SMLib)
set_target_properties(brep-bench-smlib-cpp PROPERTIES
    BUILD_RPATH "\$ORIGIN/../upstream/lib")
