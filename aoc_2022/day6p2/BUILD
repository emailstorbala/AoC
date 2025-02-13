cc_library( 
  name = "aoc_utils",
  srcs = ["src/AocUtils.cpp"],
  hdrs = ["inc/AocUtils.h", "inc/Utilities.h"],
  cxxopts = [
    "-std=c++20",
    "-s",
    "-O3",
    "-Wall",
    "-fPIC"
  ],
  includes = ["inc"],
  visibility = ["//visibility:private"],
)

cc_library( 
  name = "utils",
  srcs = ["src/Utilities.cpp"],
  hdrs = ["inc/Utilities.h"],
  cxxopts = [
    "-std=c++20",
    "-s",
    "-O3",
    "-Wall",
    "-fPIC"
  ],
  includes = ["inc"],
  visibility = ["//visibility:private"],
)

cc_binary( 
  name = "aoc",
  srcs = ["src/main.cpp"],
  cxxopts = [
    "-std=c++20",
    "-s",
    "-O3",
    "-Wall",
  ],
  includes = ["inc"],
  deps = [
    ":utils",
    ":aoc_utils"
  ],
  linkopts = [
    "-s",
    "-lboost_program_options",
    "-lfmt"
  ],
)
