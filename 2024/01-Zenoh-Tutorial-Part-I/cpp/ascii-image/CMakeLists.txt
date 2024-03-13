cmake_minimum_required(VERSION 3.27)
project(ascii_image)


set(CMAKE_CXX_STANDARD 20)

#include(FindProtobuf)
find_package(absl CONFIG REQUIRED)
find_package(Protobuf REQUIRED)
include_directories(${Protobuf_INCLUDE_DIRS})
include_directories(${CMAKE_CURRENT_BINARY_DIR})
protobuf_generate_cpp(P ROTO_SRCS PROTO_HDRS Image.proto)
#protobuf_generate_cpp(PROTO_SRCS PROTO_HDRS EXPORT_MACRO DLL_EXPORT Image.proto)
#protobuf_generate_cpp(PROTO_SRCS PROTO_HDRS DESCRIPTORS PROTO_DESCS Image.proto)


message(Protobuf_LIBRARIES="${Protobuf_LIBRARIES}")
message(CMAKE_LINK_DIRECTORIES="${CMAKE_LINK_DIRECTORIES}")

set(absl_LIBRARIES
        absl::algorithm
        absl::base
        absl::debugging
        absl::flat_hash_map
        absl::flags
        absl::memory
        absl::meta
        absl::numeric
        absl::random_random
        absl::strings
        absl::synchronization
        absl::time
        absl::utility
        absl::log
        absl::log_internal_proto
        absl::log_internal_globals
        absl::log_internal_check_op
)

include(FetchContent)

FetchContent_declare(c_backend GIT_REPOSITORY "https://github.com/eclipse-zenoh/zenoh-c" GIT_TAG release-0.10.1-rc)
FetchContent_MakeAvailable(c_backend)
FetchContent_declare(cpp_wrapper GIT_REPOSITORY "https://github.com/eclipse-zenoh/zenoh-cpp" GIT_TAG release-0.10.1-rc)
FetchContent_MakeAvailable(cpp_wrapper)

add_executable(AsciiImageSub image_sub.cpp Image.pb.cc)
target_link_libraries(AsciiImageSub PRIVATE zenohcxx::zenohc::lib ${Protobuf_LIBRARIES} ${absl_LIBRARIES})

add_executable(AsciiImagePub image_pub.cpp Image.pb.cc images.cpp)
target_link_libraries(AsciiImagePub PRIVATE zenohcxx::zenohc::lib ${Protobuf_LIBRARIES} ${absl_LIBRARIES})
