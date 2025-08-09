
#include "rust_cpp_demo/hello.h"
#include <string>

rust::String greet(rust::Str name) {
    std::string cpp_name(name.data(), name.size());
    return rust::String("Hello from C++, " + cpp_name + "!");
}
