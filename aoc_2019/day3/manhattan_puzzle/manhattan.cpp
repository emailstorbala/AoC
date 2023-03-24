#include <iostream>
#include <cstdlib>
#include <vector>
#include <map>
#include <algorithm>
#include <fstream>
#include <sstream>
#include <tuple>
#include <memory>
#include <boost/program_options.hpp>

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

typedef std::pair <int, int> POINT;
 
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

void ReadInputFile(string inpfile, vector <string> && wireV1, vector <string> && wireV2) {
    vector <string> inpCtx;

    if (ifstream myfile(inpfile.c_str()); myfile.is_open()) {
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

    auto wire1 = inpCtx[0];
    auto wire2 = inpCtx[1];
    string temp;

    std::istringstream ss1(wire1);
    while (std::getline(ss1, temp, ',')) {
        wireV1.push_back(temp);
    }

    std::istringstream ss2(wire2);
    while (std::getline(ss2, temp, ',')) {
        wireV2.push_back(temp);
    }
}

// Here wire 0->first wire, 1->second wire
void FillWireArray(vector <string> wireV, int wire, map <POINT, long> && wireMap) {
    POINT wirePt(0, 0);

    if ((wire < 0) || (wire >1)) {
        cerr << "Invalid wire call. Only 0 (for first) and 1 (for second) is allowed!" << endl;
        exit(10);
    }

    auto && [x, y] = wirePt;
    long wire_dist = 0;

    for (auto && pos: wireV) {
        auto dist = atoi(pos.substr(1).c_str());

        for (int cnt = 1; cnt <= dist; cnt++) {
            if (pos.rfind('R', 0) == 0) {
                x++;
            } else if (pos.rfind('L', 0) == 0) {
                x--;
            } else if (pos.rfind('U', 0) == 0) {
                y++;
            } else if (pos.rfind('D', 0) == 0) {
                y--;
            }

            wirePt.first = x;
            wirePt.second = y;

            wire_dist++;
            wireMap[wirePt] = wire_dist;
        }
    }
}

void FindShortestIntersectionDistanceFromOrigin(map <POINT, long> && wireMap1, map <POINT, long> &&wireMap2) {
    map<POINT, long> intersections;
    for (auto && [pt, dist] : wireMap1) {
        if (auto itr = wireMap2.find(pt); itr != wireMap2.end()) {
            auto ipt = itr->first;
            auto idist = itr->second;
            intersections[ipt] = idist; 
        }
    }

    long shortDist = -1;

    for (auto && [pt, dist] : intersections) {
        auto tempDist = std::abs(pt.first) + std::abs(pt.second);

        if (shortDist == -1) {
            shortDist = tempDist;
        } else if (tempDist < shortDist) {
            shortDist = tempDist;
        }
    }

    cout << "The shortest distance is " << shortDist << endl;
}

int main(int argc, const char * argv[]) {
    auto && [fname] = ParseProgramArguments(argc, argv);
    vector<string> wireV1;
    vector<string> wireV2;
    ReadInputFile(fname, std::move(wireV1), std::move(wireV2));
    
    map <POINT, long> wireMap1;
    FillWireArray(std::move(wireV1), 0, std::move(wireMap1));
    
    map <POINT, long> wireMap2;
    FillWireArray(std::move(wireV2), 1, std::move(wireMap2));
    
    FindShortestIntersectionDistanceFromOrigin(std::move(wireMap1), std::move(wireMap2));
    return EXIT_SUCCESS;
}

