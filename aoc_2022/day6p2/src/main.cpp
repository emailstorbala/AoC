/* Copyright [2022-2023] Balamurugan R<emailstorbala@gmail.com> */
#include "Utilities.h"
#include <algorithm>
#include <boost/program_options.hpp>
#include <chrono> // NOLINT [build/c++11]
#include <cstdio>
#include <exception>
#include <fmt/core.h>
#include <fmt/format.h>
#include <functional>
#include <iostream>
#include <stdexcept>
#include <tuple>
#include <utility>

using boost::program_options::error;
using boost::program_options::notify;
using boost::program_options::options_description;
using boost::program_options::parse_command_line;
using boost::program_options::store;
using boost::program_options::value;
using boost::program_options::variables_map;

using std::cout;
using std::endl;
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
            cout << prgDesc << endl;
            exit(0);
        }

        notify(vm);
    } catch (const error &ex) {
        fmt::print(stderr, "Exception: {}\n", ex.what());
        cout << prgDesc << endl;
        exit(3);
    }

    return filename;
}

std::list<string> ReadInputFile(const std::string &inpfile) {
    Utilities utils;
    return utils.SimpleFileRead(inpfile);
}

int GetMarkerCharacter(const string &inpStr) {
    const int cmpStrLen = 14;
    int uniqCharPos = 0;

    for (size_t pos = 0; (pos + cmpStrLen) < inpStr.size(); pos++) {
        string locStr = {inpStr.begin() + pos, inpStr.begin() + pos + cmpStrLen};
        // fmt::print("locStr -> {}\n", locStr);
        Utilities utils;
        if (utils.ContainsUniqueCharacters(locStr)) {
            // fmt::print("New char pos->{}\n", pos + cmpStrLen);
            uniqCharPos = pos + cmpStrLen;
            break;
        }
    }

    return uniqCharPos;
}

int main(int argc, const char *argv[]) {
    auto start = chrono::system_clock::now();
    auto &&fname = ParseProgramArguments(argc, argv); // NOLINT [-Wc++17-extensions]
    for (const string &line : ReadInputFile(fname)) {
        int res = GetMarkerCharacter(line);
        fmt::print("The result is {}\n", res);
    }

    auto end = chrono::system_clock::now();
    auto dur = chrono::duration_cast<chrono::nanoseconds>(end - start).count();
    cout << "Time taken: " << float(dur / 1000.0) << " mu.secs" << endl;

    return EXIT_SUCCESS;
}
