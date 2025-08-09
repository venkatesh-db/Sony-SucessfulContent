#pragma once
#include "rust/cxx.h"

// rust::Str is a borrowed UTF-8 string from Rust
// rust::String is an owned UTF-8 string returned to Rust
rust::String greet(rust::Str name);

