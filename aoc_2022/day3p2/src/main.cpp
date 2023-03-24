/* Copyright [2022-2023] Balamurugan R<emailstorbala@gmail.com> */
#include <fmt/core.h>
#include <iostream>
#include <map>
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

std::map<int, std::vector<string>> ReadInputFile(const string &inpfile) {
    Utilities utils;

    std::map<int, std::vector<string>> groupDetails;
    std::vector <string> tmpGroup;

    int grpId = 1;
    for (auto && line : utils.SimpleFileRead(inpfile)) {
        tmpGroup.emplace_back(line);

        if (tmpGroup.size() == 3) {
            groupDetails[grpId] = tmpGroup;
            tmpGroup.clear();
            grpId++;
        }
    }

    return groupDetails;
}

std::set <char> GetCommonChars(const std::vector <string> &tmpGrp) {
    if (tmpGrp.size() != 3) {
        std::runtime_error("Group contains invalid number of strings");
    }

    Utilities utils;
    auto set1 = utils.getCommonCharacters(tmpGrp[0], tmpGrp[1]);
    auto set2 = utils.getCommonCharacters(tmpGrp[1], tmpGrp[2]);

    std::set<char> commonSet;
    for (const char & com1 : set1) {
        if (std::find(set2.begin(), set2.end(), com1) != set2.end()) {
            commonSet.insert(com1);
        }
    }

    return commonSet;
}

int GetPriority(const int &_inp) {
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

int64_t getPrioritySumOfItems(const vector <char> &_items) {
    int64_t prioritySum = 0;

    for (const char & chr : _items) {
        int priority = GetPriority(std::cref(chr));
        prioritySum += priority;
    }

    return prioritySum;
}

int main(int argc, const char * argv[]) {
    auto start = chrono::system_clock::now();
    auto && [fname] = ParseProgramArguments(argc, argv); // NOLINT [-Wc++17-extensions]
    auto inpList = ReadInputFile(std::cref(fname));

    vector <char> allCommonChars;
    for (auto && [p1, p2] : inpList) { // NOLINT [-Wc++17-extensions]
        for (const char & cchr : GetCommonChars(std::cref(p2))) {
            allCommonChars.push_back(cchr);
        }
    }

    auto prioritySum = getPrioritySumOfItems(std::cref(allCommonChars));
    fmt::print("The priority sum is {}\n", prioritySum);

    auto end = chrono::system_clock::now();
    auto dur = chrono::duration_cast<chrono::nanoseconds>(end - start).count();
    cout << "Time taken: " << float(dur / 1000.0) << " mu.secs" << endl;

    return EXIT_SUCCESS;
}
