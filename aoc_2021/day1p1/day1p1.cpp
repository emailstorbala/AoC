/** Copyright [2021] <balamurugan.r@velocix.com> **/
#include <fmt/format.h>
#include <iostream>
#include <fstream>
#include <vector>
#include <tuple>
#include <boost/program_options.hpp>
#include "Utilities.h"

using std::cout;
using std::cerr;
using std::endl;
using std::vector;
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

vector <int> ReadInputFile(const char * inpfile) {
    vector <int> inpCtx;

    if (ifstream myfile(inpfile); myfile.is_open()) {
        string line;
        while (getline(myfile, line)) {
            if (line.rfind('#', 0) == 0) {
                continue;
            }
            inpCtx.push_back(atoi(line.c_str()));
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
    vector <int> inpList = ReadInputFile((const char *)fname.c_str());

    int loc = -1;
    int cnt = 0;
    for (auto && inp : inpList) {
        if (loc != -1 && inp > loc) {
            cnt += 1;
        }
        loc = inp;
    }

    cout << format("Number of time increases: {}", cnt) << endl;

    return EXIT_SUCCESS;
}
