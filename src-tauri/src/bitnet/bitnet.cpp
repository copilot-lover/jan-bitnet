#include "ggml-bitnet.h"

// Integration unit to build BitNet kernels within Jan's Tauri core.
// This compilation unit simply pulls in the official BitNet sources
// from the BitNet project to make them available to the build system.
// The actual kernels live in the accompanying ggml-bitnet-*.cpp files.

#include "ggml-bitnet-lut.cpp"
#include "ggml-bitnet-mad.cpp"
