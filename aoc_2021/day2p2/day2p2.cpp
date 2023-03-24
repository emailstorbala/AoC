/** Copyright [2021] <balamurugan.r@velocix.com> **/
#include <fmt/format.h>
#include <stdexcept>
#include <iostream>
#include <fstream>
#include <tuple>
#include <boost/program_options.hpp>
#include "Utilities.h"

using std::cout;
using std::cerr;
using std::endl;
using std::map;
using std::ifstream;
using std::string;
using std::tuple;
using fmt::format;

using boost::program_options::notify;
using boost::program_options::parse_command_line;
using boost::program_options::store;
using boost::program_options::variables_map;
using boost::program_options::options_description;
using boost::program_options::value;
using boost::program_options::error;

struct SubmarineCmd {
        string action;  // action
        int value;  // action value
};

struct SubmarinePosition {
        int hp;  // Horizontal position
        int dp;  // Depth position
        int aim;  // aim position
};

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
        cerr << "Exception: '" << ex.what() << endl;
        cout << prgDesc << endl;
        exit(3);
    }

    return make_tuple(filename);
}

map<int, SubmarineCmd> ReadInputFile(const char * inpfile) {
    map <int, SubmarineCmd> inpCtx;

    if (ifstream myfile(inpfile); myfile.is_open()) {
        string line;
        int cnt = 0;
        while (getline(myfile, line)) {
            if (line.rfind('#', 0) == 0) {
                continue;
            }
            char action[10];
            int value = 0;

            sscanf(line.c_str(), "%s %d", static_cast<char *>(action), &value);
            inpCtx[++cnt] = SubmarineCmd{action, value};
            cout << format("action, value -> {},{}", action, value) << endl;
        }
        myfile.close();
    } else {
        std::cerr << "Unable to open file '" << inpfile
                  << "'. Please check!" << endl;
        exit(2);
    }

    return inpCtx;
}

int main(int argc, const char * argv[]) {
    auto && [fname] = ParseProgramArguments(argc, argv);
    map<int, SubmarineCmd> inpList = ReadInputFile((const char *)fname.c_str());

    for (auto && [idx, cmd] : inpList) {
        cout << format("11 action, value -> {}, {}", cmd.action, cmd.value)
             << endl;
    }

    SubmarinePosition subPos = {0, 0, 0};
    for (auto && [idx, cmd] : inpList) {
        if (cmd.action == "forward") {
            subPos.hp += cmd.value;
            subPos.dp += cmd.value * subPos.aim;
        } else if (cmd.action == "down") {
            //subPos.dp += cmd.value;
            subPos.aim += cmd.value;
        } else if (cmd.action == "up") {
            //subPos.dp -= cmd.value;
            subPos.aim -= cmd.value;
        } else {
            throw std::invalid_argument("Invalid command action!");
        }
        cout << format("pos(hp,dp,aim) -> ({},{},{})", subPos.hp, subPos.dp, subPos.aim) << endl;
    }

    cout << format("SubPosition -> ({}, {})", subPos.hp, subPos.dp) << endl;
    cout << format("SubPosition prod -> {}", subPos.hp * subPos.dp) << endl;

    return EXIT_SUCCESS;
}
