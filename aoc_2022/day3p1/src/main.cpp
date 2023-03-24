/* Copyright [2022-2023] Balamurugan R<emailstorbala@gmail.com> */
#include <fmt/core.h>
#include <iostream>
#include <algorithm>
#include <chrono> // NOLINT [build/c++11]
#include <boost/program_options.hpp>
#include "Utilities.h"

using boost::program_options::notify;
using boost::program_options::parse_command_line;
using boost::program_options::store;
using boost::program_options::variables_map;
using boost::program_options::options_description;
using boost::program_options::value;
using boost::program_options::error;

using std::cout;
using std::endl;
using std::string;
using std::tuple;
using std::vector;

namespace chrono = std::chrono;

tuple<string> ParseProgramArguments(const int argc, const char * argv[]) {
    string filename;
    variables_map vm;
    options_description prgDesc{"Usage"};

    try {
        prgDesc.add_options()
            ("help, h", "Help screen")
            ("i", value<string>(&filename)->required(), "Input File");

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

    return make_tuple(filename);
}

vector <std::pair<string, string>> ReadInputFile(string inpfile) {
    vector <std::pair<string, string>> inpCtx;
    Utilities utils;

    for (auto && line : utils.SimpleFileRead(inpfile)) {
        auto && [part1, part2] = utils.SplitStringExactHalf(line); // NOLINT [-Wc++17-extensions]
        inpCtx.push_back(std::make_pair(part1, part2));
    }

    return inpCtx;
}

int GetPriority(int _inp) {
    int priority = 0;

    /* Priority of a-z goes through as 1-26
     * Priority of A-Z goes through as 27-52
     */

    if (_inp >= 'a') {
        priority = _inp - 'a' + 1;
    } else if (_inp >= 'A') {
        priority = (_inp - 'A') + 27;
    } else {
        std::runtime_error("Invalid character came in input file");
    }

    return priority;
}

int64_t getPrioritySumOfItems(vector <char> _items) {
    int64_t prioritySum = 0;

    for (const char & chr : _items) {
        int priority = GetPriority(chr);
        prioritySum += priority;
    }

    return prioritySum;
}

int main(int argc, const char * argv[]) {
    auto start = chrono::system_clock::now();
    auto && [fname] = ParseProgramArguments(argc, argv); // NOLINT [-Wc++17-extensions]
    auto inpList = ReadInputFile(fname);
    Utilities utils;

    vector <char> allCommonChars;
    for (auto && [p1, p2] : inpList) { // NOLINT [-Wc++17-extensions]
        for (const char & cchr : utils.getCommonCharacters(p1, p2)) {
            allCommonChars.push_back(cchr);
        }
    }

    auto prioritySum = getPrioritySumOfItems(allCommonChars);
    fmt::print("The priority sum is {}\n", prioritySum);

    auto end = chrono::system_clock::now();
    auto dur = chrono::duration_cast<chrono::nanoseconds>(end - start).count();
    cout << "Time taken: " << float(dur / 1000.0) << " mu.secs" << endl;

    return EXIT_SUCCESS;
}
