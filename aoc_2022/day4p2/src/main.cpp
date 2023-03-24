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

typedef std::vector<std::set<int>> LINE_DTLS;

std::vector <LINE_DTLS> ReadInputFile(string inpfile) {
    Utilities utils;
    std::vector <LINE_DTLS> allLines;

    for (auto && line : utils.SimpleFileRead(inpfile)) {
        auto && splitDtls = utils.Split(line, ',');
        LINE_DTLS lineDtls;
        for (auto && tmpRange : splitDtls) {
            auto && tmp = utils.Split(tmpRange, '-');
            int start = std::stoi(tmp[0].c_str());
            int end = std::stoi(tmp[1].c_str());
            auto tmpSet = utils.GetNumberRange(start, end);
            lineDtls.emplace_back(tmpSet);
        }
        allLines.emplace_back(lineDtls);
    }

    return allLines;
}


int main(int argc, const char * argv[]) {
    auto start = chrono::system_clock::now();
    auto && [fname] = ParseProgramArguments(argc, argv); // NOLINT [-Wc++17-extensions]
    auto inpList = ReadInputFile(fname);

    int overlapSets = 0;
    Utilities utils;

    for (auto && lineDtls : inpList) { // NOLINT [-Wc++17-extensions]
        /* Checking any set 1 items are present in set 2 */
        if(utils.DoSetsHaveOverlapItems(lineDtls[0], lineDtls[1])) overlapSets++;
    }

    fmt::print("containedSets : {}\n", overlapSets);
    auto end = chrono::system_clock::now();
    auto dur = chrono::duration_cast<chrono::nanoseconds>(end - start).count();
    cout << "Time taken: " << float(dur / 1000.0) << " mu.secs" << endl;

    return EXIT_SUCCESS;
}
