#include <iostream>
#include <cmath>
#include <vector>
#include <map>
#include <algorithm>
#include <fstream>
#include <tuple>
#include <memory>
#include <numeric>
#include <stack>
#include <future>
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

typedef std::pair<int, int> POINT;
typedef map<POINT, char> ASTEROID_MAP;

ASTEROID_MAP ReadInputFile(const char * inpfile, int &rows, int &cols) {
    vector <string> inpCtx;
    ASTEROID_MAP astMap;

    if (ifstream myfile(inpfile); myfile.is_open()) {
        string line;
        while (getline(myfile, line)) {
            inpCtx.push_back(line);
        }
        myfile.close();
    } else {
        std::cerr << "Unable to open file '" << inpfile << "'. Please check!" << endl;
        exit(2);
    }

    string line;
    for (int lpos = 0; lpos < int(inpCtx.size()); lpos++) {
        line = inpCtx[lpos];

        for (int pos = 0; pos < int(line.length()); pos++) {
            char currChar = line[pos];
            POINT currPt(pos, lpos); 
            astMap[currPt] = currChar;
        } 
    }

    rows = int(inpCtx.size());
    cols = int(line.length());

    return astMap;
}

vector <POINT> GetAstroidPoints(ASTEROID_MAP astMap) {
    vector <POINT> astPoints;
    for (auto && [pt, ch]: astMap) {
        if (ch == '#') {
            astPoints.push_back(pt);
        }
    }

    return astPoints;
}

int distance(POINT a, POINT b) {
    return nearbyint(std::sqrt(std::pow((a.first - b.first), 2) + std::pow((a.second - b.second), 2)));
}

// Checking whether point a lies between b and c
bool LiesInBetween(POINT a, POINT b, POINT c) {
    return (distance(b, a) + distance(a, c) == distance(b, c));
}

void GetAsteroidsLookupCount(ASTEROID_MAP astMap, int rows, int cols) {
    auto pts = GetAstroidPoints(astMap);
    map <POINT, int> pointCntMap;

    for (auto pt: pts) {
        for (auto lpt: pts) {
            if (pt == lpt) continue;

            bool condMet = true;
            
            for (auto mpt: pts) {
                if (mpt == pt || mpt == lpt) continue;

                if (LiesInBetween(mpt, pt, lpt)) {
                    condMet = false;
                    break;
                }
            }

            if (condMet){
                //cout << "Condition met for points (" << pt.first << ", " << pt.second << ") : (" << lpt.first << ", " << lpt.second << ")" << endl;
                if (pointCntMap.find(pt) != pointCntMap.end()) {
                    int cnt = pointCntMap[pt];
                    pointCntMap[pt] = cnt + 1;
                } else {
                    pointCntMap[pt] = 1;
                }
            }
        }
    }

    int maxCnt = 0;
    POINT maxPoint;
    for (auto && [pt, cnt]: pointCntMap) {
        if (maxCnt < cnt) {
            maxCnt = cnt;
            maxPoint = pt;
        }
    }

    cout << "(" << maxPoint.first << ", " << maxPoint.second << ") -> " << maxCnt << endl;
    /***/
    for (auto && [pt, cnt]: pointCntMap) {
        cout << "(" << pt.first << ", " << pt.second << ") -> " << cnt << endl;
    }
    /***/
}

int main(int argc, const char * argv[]) {
    /**** /
    POINT a(1, 0);
    POINT b(2, 2);
    POINT c(3, 4);

    bool ret = LiesInBetween(a, b, c);
    if (ret) {
        cout << "Point a lies between b and c" << endl;
    }
    / ****/

    /****/
    auto && [fname] = ParseProgramArguments(argc, argv);

    int rows = -1;
    int cols = -1;

    auto && astMap = ReadInputFile((const char *)fname.c_str(), rows, cols);

    //cout << "rows -> " << rows << endl;
    //cout << "cols -> " << cols << endl;
    /*****/

    /***/
    for (int row = 0; row < rows; row++) {
        for (int col = 0; col < cols; col++) {
            POINT pt(col, row);
            cout << astMap[pt] << "   ";
        }
        cout << endl;
        cout << endl;
    }
    /***/

    GetAsteroidsLookupCount(astMap, rows, cols);

    return EXIT_SUCCESS;
}

