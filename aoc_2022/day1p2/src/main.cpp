/* Copyright 2022-2023 Balamurugan R <emailstorbala@gmail.com> */
#include <fmt/format.h>
#include <iostream>
#include <fstream>
#include <algorithm>
#include <chrono> // NOLINT [build/c++11]
#include <tuple>
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
using std::ifstream;

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

vector <int> ReadInputFile(const char * inpfile) {
    vector <int> inpCtx;

    if (ifstream myfile(inpfile); myfile.is_open()) { // NOLINT [-Wc++17-extensions]
        string line;
        int tmpSum = 0;
        while (getline(myfile, line)) {
            tmpSum += atoi(line.c_str());
            if (line.empty()) {
                inpCtx.push_back(tmpSum);
                tmpSum = 0;
            }
        }
        inpCtx.push_back(tmpSum);
        myfile.close();
    } else {
        std::cerr << "Unable to open file '" << inpfile
                  << "'. Please check!" << endl;
        exit(2);
    }

    return inpCtx;
}

int main(int argc, const char * argv[]) {
    auto start = chrono::system_clock::now();
    auto && [fname] = ParseProgramArguments(argc, argv); // NOLINT [-Wc++17-extensions]
    vector <int> inpList = ReadInputFile((const char *)fname.c_str());
    std::sort(inpList.begin(), inpList.end(), std::greater<>());
    int maxCalorie = inpList[0] + inpList[1] + inpList[2];
    fmt::print("Maximum calore is ->{}\n", maxCalorie);
    auto end = chrono::system_clock::now();
    auto dur = chrono::duration_cast<chrono::nanoseconds>(end - start).count();
    cout << "Time taken: " << float(dur / 1000.0) << " mu.secs" << endl;

    return EXIT_SUCCESS;
}
