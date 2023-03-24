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

const float PI = 3.14159265;

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
typedef vector <POINT> POINTS;
typedef vector <float> ANGLES;

typedef map<POINT, char> ASTEROID_MAP;
typedef map<float, POINT> DISTANCE_POINT_MAP;
typedef map<float, DISTANCE_POINT_MAP> TOTAL_PROJECTION_MAP;

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
    POINTS astPoints;
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

ANGLES GetShootableProjects(TOTAL_PROJECTION_MAP projPoints) {
    ANGLES projs;

    for (auto && [proj, distPointMap]: projPoints) {
        projs.push_back(proj);
    }

    //std::sort(projs.begin(), projs.end(), std::greater<>());
    std::sort(projs.begin(), projs.end());

    ANGLES retProjs;
    for (auto && proj: projs) {
        if (proj >= 90) {
            retProjs.push_back(proj);
        }
    }

    for (auto && proj: projs) {
        if (proj < 0) {
            retProjs.push_back(proj);
        }
    }

    for (auto && proj: projs) {
        if ( (proj < 90) and (proj >= 0)) {
            retProjs.push_back(proj);
        }
    }

    /*** /
      for (auto && proj: retProjs) {
      cout << "proj ->" << proj << endl;
      }
      / ***/

    return retProjs;
}

TOTAL_PROJECTION_MAP GetTotalProjectionMap(POINT pt, POINTS pts) {
    TOTAL_PROJECTION_MAP projPoints;

    for (auto lpt: pts) {
        if (pt == lpt) continue;

        float degrees = atan2(pt.second-lpt.second, pt.first-lpt.first) * 180 / PI;
        //cout << "degrees ->" << degrees << endl;

        float dist = distance(pt, lpt);

        if (projPoints.find(degrees) != projPoints.end()) {
            auto distPointMap = projPoints[degrees];
            distPointMap[dist] = lpt;
            projPoints[degrees] = distPointMap;
        } else {
            DISTANCE_POINT_MAP distPointMap;
            distPointMap[dist] = lpt;
            projPoints[degrees] = distPointMap;
        }
    }

    return projPoints;
}

std::pair<float, POINT> GetMinDistanceFromMap(DISTANCE_POINT_MAP distMap) {
    float tmpDist = -1;
    POINT lpt;

    for (auto && [dist, pt]: distMap) {
        if (tmpDist == -1) {
            tmpDist = dist;
            lpt = pt;
        } else if (dist < tmpDist) {
            tmpDist = dist;
            lpt = pt;
        }
    }

    return std::pair(tmpDist, lpt);
}

void GetAsteroidsLookupCount(ASTEROID_MAP astMap) {
    auto pts = GetAstroidPoints(astMap);

    //POINT pt(3, 4); // Got the ideal/best position from part 1 program 
    POINT pt(23, 19); // Got the ideal/best position from part 1 program 
    //POINT pt(11, 13); // Got the ideal/best position from part 1 program 

    TOTAL_PROJECTION_MAP projPoints = GetTotalProjectionMap(pt, pts);
    ANGLES projs = GetShootableProjects(projPoints);

    /*** /
      for (auto && [proj, distPointMap]: projPoints) {
      cout << proj << ":" << endl;
      for ( auto && [dist, pt]: distPointMap) {
      cout << "\t " << dist << ", (" << pt.first << ", " << pt.second << ")" << endl;
      }
      }
      / ***/

    int cnt = 1;
    while (projPoints.size()) {
        ANGLES remProjs;
        for (auto && proj: projs) {
            if (projPoints[proj].empty()) continue;

            DISTANCE_POINT_MAP distPointMap = projPoints[proj]; 
            auto && [minDist, minPt] = GetMinDistanceFromMap(distPointMap);

            // Destroying the asteroid from map
            distPointMap.erase(minDist);
            if (distPointMap.empty()) {
                remProjs.push_back(proj);
            }
            projPoints[proj] = distPointMap;

            if (cnt == 200) {
                cout << cnt << "th projection is " << proj << endl;
                cout << cnt << "th destroyed point is (" << minPt.first << ", " << minPt.second << ")" << endl;
            }
            cnt++;
        }

        for (auto && proj: remProjs) {
            projPoints.erase(proj);
            auto idx = std::find(projs.begin(), projs.end(), proj);
            if (idx != projs.end()) {
                projs.erase(idx);
            }
        }
    }
}

int main(int argc, const char * argv[]) {
    auto && [fname] = ParseProgramArguments(argc, argv);

    int rows = -1;
    int cols = -1;

    auto && astMap = ReadInputFile((const char *)fname.c_str(), rows, cols);

    /*** /
      for (int row = 0; row < rows; row++) {
      for (int col = 0; col < cols; col++) {
      POINT pt(col, row);
      cout << astMap[pt] << "   ";
      }
      cout << endl;
      cout << endl;
      }
      / ***/

    GetAsteroidsLookupCount(astMap);

    return EXIT_SUCCESS;
}

