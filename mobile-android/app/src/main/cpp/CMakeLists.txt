cmake_minimum_required(VERSION 3.22.1)

project("pocketmic_engine" CXX C)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Find Google Oboe via Prefab
find_package(oboe REQUIRED CONFIG)

add_library(pocketmic_engine SHARED
    jni_bridge.cpp
    audio_engine.cpp
    opus_encoder.cpp
    udp_client.cpp
)

target_include_directories(pocketmic_engine PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}
)

target_link_libraries(pocketmic_engine PRIVATE
    oboe::oboe
    log
    android
)
