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

string CalculateO2Rating(vector <string> inpList) {
    int binLength = inpList.at(0).length();
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

        vector <string> tmpList;
        if (one_cnt > zero_cnt) {
            for (auto && inp : inpList) {
                if (inp[cnt] == '1') {
                    tmpList.push_back(inp);
                }
            }
        } else if (one_cnt == zero_cnt) {
            for (auto && inp : inpList) {
                if (inp[cnt] == '1') {
                    tmpList.push_back(inp);
                }
            }
        } else {
            for (auto && inp : inpList) {
                if (inp[cnt] == '0') {
                    tmpList.push_back(inp);
                }
            }
        }

        inpList = tmpList;

        /***/
        cout << endl << "----------" << endl;
        for (auto && loc : inpList) {
            cout << loc << endl;
        }
        cout << "----------" << endl;
        /***/
        if (inpList.size() == 1) break;
    }

    return inpList[0];
}

string CalculateCO2Rating(vector <string> inpList) {
    int binLength = inpList.at(0).length();
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

        vector <string> tmpList;
        if (one_cnt < zero_cnt) {
            for (auto && inp : inpList) {
                if (inp[cnt] == '1') {
                    tmpList.push_back(inp);
                }
            }
        } else if (one_cnt == zero_cnt) {
            for (auto && inp : inpList) {
                if (inp[cnt] == '0') {
                    tmpList.push_back(inp);
                }
            }
        } else {
            for (auto && inp : inpList) {
                if (inp[cnt] == '0') {
                    tmpList.push_back(inp);
                }
            }
        }

        inpList = tmpList;

        /*** /
        cout << endl << "----------" << endl;
        for (auto && loc : inpList) {
            cout << loc << endl;
        }
        cout << "----------" << endl;
        / ***/
        if (inpList.size() == 1) break;
    }

    return inpList[0];
}

int main(int argc, const char * argv[]) {
    auto && [fname] = ParseProgramArguments(argc, argv);
    vector <string> inpList = ReadInputFile((const char *)fname.c_str());

    string o2Rating = CalculateO2Rating(inpList);
    cout << "o2Rating is " << o2Rating << endl;

    string co2Rating = CalculateCO2Rating(inpList);
    cout << "co2Rating is " << co2Rating << endl;

    int64_t o2RatingNum = strtoul(o2Rating.c_str(), nullptr, 2);
    int64_t co2RatingNum = strtoul(co2Rating.c_str(), nullptr, 2);

    cout << format("o2RatingNum, co2RatingNum -> {},{}",
                   o2RatingNum, co2RatingNum) << endl;
    cout << format("The product of o2RatingNum and co2RatingNum is {}",
                   o2RatingNum * co2RatingNum) << endl;

    return EXIT_SUCCESS;
}
