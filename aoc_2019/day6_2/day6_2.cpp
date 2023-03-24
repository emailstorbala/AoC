#include <iostream>
#include <vector>
#include <map>
#include <algorithm>
#include <fstream>
#include <tuple>
#include <memory>
#include <numeric>
#include <boost/program_options.hpp>
#include "Utilities.h"

using std::string;
using std::cout;
using std::cin;
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

map <string, string> ReadInputFile(const char * inpfile) {
    map <string, string> orbitMap;
    if (ifstream myfile(inpfile); myfile.is_open()) {
        string line;
        Utilities utils;
        while (getline(myfile, line)) {
            if (line.rfind('#', 0) == 0) {
                continue;
            }
            vector<string> tmpV = utils.Split(line, ')');
            orbitMap[tmpV.at(1)] = tmpV[0];
        }
        myfile.close();
    } else {
        std::cerr << "Unable to open file '" << inpfile << "'. Please check!" << endl;
        exit(2);
    }

    return orbitMap;
}

map<int, string> FindDirectAndIndirectOrbits(string oObj, map<string, string> & orbitMap) {
    int orbits = 0;
    map<int, string> routeDtls;

    for (string obj = oObj; orbitMap.find(obj) != orbitMap.end();
         obj = orbitMap[obj]) {
        orbits++;
        routeDtls[orbits] = orbitMap[obj];
    }

    return routeDtls;
}

int main(int argc, const char * argv[]) {
    auto && [fname] = ParseProgramArguments(argc, argv);
    auto && orbitMap = ReadInputFile((const char *)fname.c_str());

    auto youDtls = FindDirectAndIndirectOrbits(orbitMap["YOU"], orbitMap);
    auto sanDtls = FindDirectAndIndirectOrbits(orbitMap["SAN"], orbitMap);

    int shortDist = -1;

    for (int dist = 1; dist <= int(youDtls.size()); dist++) {
        string obj = youDtls[dist];
        bool found = false;

        for (auto && [sDist, sObj]: sanDtls) {
            if (sObj == obj) {
                cout << "sObj,obj -> " << sObj << "," << obj << endl;
                cout << "dist,sDist -> " << dist << "," << sDist << endl;
                shortDist = dist + sDist;
                found = true;
                break;
            }
        }

        if (found) break;
    }
    
    cout << "Short Distance is " << shortDist << endl;

    return EXIT_SUCCESS;
}

