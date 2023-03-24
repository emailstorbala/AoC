/* Copyright [2022-2023] Balamurugan R<emailstorbala@gmail.com> */
#include <fmt/core.h>
#include <fmt/format.h>
#include <cstdio>
#include <iostream>
#include <algorithm>
#include <chrono> // NOLINT [build/c++11]
#include <boost/program_options.hpp>
#include <memory>
#include <exception>
#include <stdexcept>
#include <tuple>
#include <utility>
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
using std::map;

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

struct CrateStack {
    map<int, std::list<char>> stackMap;
    vector<tuple<int, int, int>> stackInst;

    void ReadInputFile(string inpfile);
    void PrintStackMap(void);
    void PrintInstructions(void);
    bool ApplyInstruction(tuple<int, int, int> ins);
    void ApplyAllInstructionsInOrder(void);
};

void CrateStack::ReadInputFile(string inpfile) {
    Utilities utils;
    std::list <string> lines = utils.SimpleFileRead(inpfile);

    for (auto && line : lines) {
        if (line.empty() || line[1] == '1') break;

        int crateId = 1;
        for (size_t idx = 0; idx < line.size(); idx+=4) {
            char locChr = line[idx+1];  // crate position is idx + 1
            if (locChr && locChr != ' ') {
                std::list <char> tmpCrates;
                if (this->stackMap.find(crateId) != this->stackMap.end()) {
                    tmpCrates = this->stackMap[crateId];
                }
                tmpCrates.emplace_back(locChr);
                this->stackMap[crateId] = tmpCrates;
            }

            crateId++;
        }
    }

    for (auto && line : lines) {
        if (utils.StringContainsString(line, "move")) {
            int numCrates;
            int fromCrateId;
            int toCrateId;

            sscanf(line.c_str(), "move %d from %d to %d", &numCrates, &fromCrateId, &toCrateId);
            this->stackInst.emplace_back(std::make_tuple(numCrates, fromCrateId, toCrateId));
        }
    }
}

void CrateStack::PrintStackMap(void) {
    for (auto && [idx, stackList] : this->stackMap) { // NOLINT [-Wc++17-extensions]
        fmt::print("Crates for crate id '{}' are: [", idx);
        for (auto && crateChr : stackList) {
            fmt::print("{} ", crateChr);
        }
        fmt::print("]\n");
    }
}

void CrateStack::PrintInstructions(void) {
    for(auto && [numCrates, fromCrateId, toCrateId] : this->stackInst) { // NOLINT [-Wc++17-extensions]
        fmt::print("{}, {}, {}\n", numCrates, fromCrateId, toCrateId);
    }
}

bool CrateStack::ApplyInstruction(tuple<int, int, int> ins) {
    bool applied = false;
    auto && [numCrates, fromCrateId, toCrateId] = ins; // NOLINT [-Wc++17-extensions]

    try {
        for (int cnt = 0; cnt < numCrates; cnt++) {
            char loc = this->stackMap[fromCrateId].front();
            this->stackMap[fromCrateId].pop_front();
            this->stackMap[toCrateId].push_front(loc);
        }
    } catch (std::exception & excp) {
        string customExcp = fmt::format("{} {}", "Runtime exception occurred! ", excp.what());
        std::runtime_error(customExcp.c_str());
    }

    return applied;
}

void CrateStack::ApplyAllInstructionsInOrder(void) {
    for (auto && crateIns : this->stackInst) {
        this->ApplyInstruction(crateIns);
    }
}

int main(int argc, const char * argv[]) {
    auto start = chrono::system_clock::now();
    auto && [fname] = ParseProgramArguments(argc, argv); // NOLINT [-Wc++17-extensions]

    auto crateStackPtr = std::make_unique<CrateStack>();
    crateStackPtr->ReadInputFile(fname);

    crateStackPtr->ApplyAllInstructionsInOrder();
    // fmt::print("After instructions apply\n");
    // crateStackPtr->PrintStackMap();

    string res;

    for(auto && [crateId, crateList] : crateStackPtr->stackMap) { // NOLINT [-Wc++17-extensions]
        char topChr = crateList.front();
        res.push_back(topChr);
    }
    fmt::print("Result is {}\n", res);

    auto end = chrono::system_clock::now();
    auto dur = chrono::duration_cast<chrono::nanoseconds>(end - start).count();
    cout << "Time taken: " << float(dur / 1000.0) << " mu.secs" << endl;

    return EXIT_SUCCESS;
}
