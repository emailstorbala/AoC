#include <iostream>
#include <algorithm>
#include <chrono>
#include <fstream>
#include <tuple>
#include <fmt/format.h>
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
using fmt::print;

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

    if (ifstream myfile(inpfile); myfile.is_open()) {
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
    auto && [fname] = ParseProgramArguments(argc, argv);
    vector <int> inpList = ReadInputFile((const char *)fname.c_str());
    int maxCalorie = *max_element(inpList.begin(), inpList.end());
    fmt::print("Maximum calore is ->{}\n", maxCalorie);
    auto end = chrono::system_clock::now();
    auto dur = chrono::duration_cast<chrono::nanoseconds>(end - start).count();
    cout << "Time taken: " << float(dur / 1000) << " mu.secs" << endl;

    return EXIT_SUCCESS;
}
