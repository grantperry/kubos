IF(TARGET_MSP430_GCC_TOOLCHAIN_INCLUDED)
    return()
endif()
set(TARGET_MSP430_GCC_TOOLCHAIN_INCLUDED 1)

set(_CPU_COMPILATION_OPTIONS "-mmcu=msp430f5529")
set(_CPU_DEFINES "")


set(CMAKE_C_FLAGS_INIT             "${CMAKE_C_FLAGS_INIT} ${_CPU_COMPILATION_OPTIONS} ${_CPU_DEFINES}")
set(CMAKE_ASM_FLAGS_INIT           "${CMAKE_ASM_FLAGS_INIT} ${_CPU_COMPILATION_OPTIONS} ${_CPU_DEFINES}")
set(CMAKE_CXX_FLAGS_INIT           "${CMAKE_CXX_FLAGS_INIT} ${_CPU_COMPILATION_OPTIONS} ${_CPU_DEFINES}")
set(CMAKE_MODULE_LINKER_FLAGS_INIT "${CMAKE_MODULE_LINKER_FLAGS_INIT} ${_CPU_COMPILATION_OPTIONS}")
set(CMAKE_C_LINK_FLAGS             "${CMAKE_C_LINK_FLAGS} ${_CPU_COMPILATION_OPTIONS}")
set(CMAKE_EXE_LINKER_FLAGS_INIT    "${CMAKE_EXE_LINKER_FLAGS_INIT} -T\"${CMAKE_CURRENT_LIST_DIR}/../ld/memory.x\" -T\"${CMAKE_CURRENT_LIST_DIR}/../ld/periph.x\" -T\"${CMAKE_CURRENT_LIST_DIR}/../ld/msp430.x\"")