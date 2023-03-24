/** Copyright [2021] <balamurugan.r@velocix.com> **/
#include <fmt/format.h>
#include <cstring>
#include <cstdlib>
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
        std::cerr << "Unable to open file '" << inpfile
                  << "'. Please check!" << endl;
        exit(2);
    }

    return inpCtx;
}

int main(int argc, const char * argv[]) {
    auto && [fname] = ParseProgramArguments(argc, argv);
    vector <string> inpList = ReadInputFile((const char *)fname.c_str());
    char * gamma;
    char * epsilon;
    int binLength = inpList.at(0).length();

    gamma = new char[binLength+1];
    epsilon = new char[binLength+1];

    memset(gamma, '\0', binLength+1);
    memset(epsilon, '\0', binLength+1);

    for (int cnt = 0; cnt < binLength; cnt++) {
        int one_cnt = 0;
        int zero_cnt = 0;
        for (auto && inp : inpList) {
            if (inp[cnt] == '0') {
                zero_cnt += 1;
            } else {
                one_cnt += 1;
            }
        }

        //cout << format("cnt, (one_cnt, zero_cnt) -> ({},{})",
        //               cnt, one_cnt, zero_cnt) << endl;

        if (one_cnt > zero_cnt) {
            gamma[cnt] = '1';
            epsilon[cnt] = '0';
        } else {
            gamma[cnt] = '0';
            epsilon[cnt] = '1';
        }
    }

    int64_t gammaNum = strtoul(gamma, nullptr, 2);
    int64_t epsilonNum = strtoul(epsilon, nullptr, 2);

    delete [] gamma;
    delete [] epsilon;

    cout << format("gamma is {}", gammaNum) << endl;
    cout << format("epsilon is {}", epsilonNum) << endl;
    cout << format("Power consumption is {}", gammaNum * epsilonNum) << endl;

    return EXIT_SUCCESS;
}
