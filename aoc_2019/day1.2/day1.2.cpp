#include <iostream>
#include <vector>
#include <map>
#include <algorithm>
#include <fstream>
#include <tuple>
#include <memory>
#include <boost/program_options.hpp>
#include "Utilities.h"

using std::string;
using std::cout;
using std::cerr;
using std::endl;
using std::vector;
using std::map;
using std::ifstream;
using std::tuple;

using boost::program_options::notify;
using boost::program_options::parse_command_line;
using boost::program_options::store;
using boost::program_options::variables_map;
using boost::program_options::options_description;
using boost::program_options::value;
using boost::program_options::error;

const int NUM_OF_DIGITS = 6;

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

vector <string> ReadInputFile(const char * inpfile) {
    vector <string> inpCtx;

    if (ifstream myfile(inpfile); myfile.is_open()) {
        string line;
        while (getline(myfile, line)) {
            if (line.rfind('#', 0) == 0) {
                continue;
            }
            inpCtx.push_back(line);
        }
        myfile.close();
    } else {
        std::cerr << "Unable to open file '" << inpfile << "'. Please check!" << endl;
        exit(2);
    }

    return inpCtx;
}

long CalculateFuel(int mass) {
    long totalFuel = 0;
    int reqFuel = 0;

    do {
        reqFuel = int(mass/3) - 2;
        if (reqFuel > 0) {
            totalFuel += reqFuel;
            mass = reqFuel;
        }
    } while (reqFuel > 0);

    return totalFuel;
}

int main(int argc, const char * argv[]) {
    auto && [fname] = ParseProgramArguments(argc, argv);
    vector <string> massList = ReadInputFile((const char *)fname.c_str());

    long totalFuelReq = 0;
    for (auto && mass: massList) {
        long massNum = atol(mass.c_str());
        auto fuelReq = CalculateFuel(massNum);
        totalFuelReq += fuelReq;
    }

    cout << "Fuel required " << totalFuelReq << endl;
    return EXIT_SUCCESS;
}

