/* Copyright [2022-2023] Balamurugan R<emailstorbala@gmail.com> */
#include "AocUtils.h"
#include "Utilities.h"
#include <boost/program_options.hpp>
#include <chrono>
#include <fmt/format.h>
#include <iostream>
#include <string_view>

using boost::program_options::error;
using boost::program_options::notify;
using boost::program_options::options_description;
using boost::program_options::parse_command_line;
using boost::program_options::store;
using boost::program_options::value;
using boost::program_options::variables_map;

using std::string;

namespace chrono = std::chrono;

string ParseProgramArguments(const int argc, const char *argv[]) {
    string filename;
    variables_map vm;
    options_description prgDesc{"Usage"};

    try {
        prgDesc.add_options()("help, h", "Help screen")(
            "i", value<string>(&filename)->required(), "Input File");

        store(parse_command_line(argc, argv, prgDesc), vm);

        if (vm.count("help") || vm.count("h")) {
            prgDesc.print(std::cout);
            exit(0);
        }

        notify(vm);
    } catch (const error &ex) {
        fmt::println(stderr, "Exception: {}", ex.what());
        prgDesc.print(std::cout);
        exit(3);
    }

    return filename;
}

std::list<string> ReadInputFile(std::string_view inpfile) {
    Utilities utils;
    return utils.SimpleFileRead(inpfile);
}

int main(int argc, const char *argv[]) {
    auto start = chrono::steady_clock::now();
    auto &&fname = ParseProgramArguments(argc, argv);
    for (const string &line : ReadInputFile(fname)) {
        AocUtils aocUtils;
        if (auto res = aocUtils.GetMarkerCharacter(line); res != string::npos) {
            fmt::println("The result is {}", res);
        }
    }

    auto end = chrono::steady_clock::now();
    auto dur = chrono::duration_cast<chrono::microseconds>(end - start).count();
    fmt::println("Time taken: {} µ.sec", dur);

    return EXIT_SUCCESS;
}
