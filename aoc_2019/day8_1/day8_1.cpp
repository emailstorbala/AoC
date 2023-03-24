#include <iostream>
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

string ReadInputFile(const char * inpfile) {
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

    if (inpCtx.size() != 1) {
        cerr << "Number of input lines are wrong. Please provide one line of input. Exiting" << endl;
        exit(2);
    }

    return inpCtx[0];
}

void CreateLayers(string inp, const int rows, const int cols) {
    int layersCnt = inp.length() / (rows * cols);
    cout << "layersCount ->" << layersCnt << endl;

    int layers[layersCnt][rows][cols];
    int residues = 0;

    residues = inp.length() % (rows * cols);
    if (residues != 0) {
        cout << "The input won't fit into the layer (row, col). Exiting ..." << endl;
        exit(5);
    }

    int pos = 0;
    cout << "Rows ->" << rows << endl;
    cout << "Cols ->" << cols << endl;

    Utilities utils;

    typedef vector <int> LAYER;
    map<int, LAYER> layerMap;
    map<int, int> layerZeroCntMap;
    map<int, int> layerOneCntMap;
    map<int, int> layerTwoCntMap;

    for (int cnt = 0; cnt < layersCnt; cnt++) {
        vector <int> layerV;
        int zeroCnt = 0;
        int oneCnt = 0;
        int twoCnt = 0;
        for (int col = 0; col < cols; col++) {
            for (int row = 0; row < rows; row++) {
                int _val = utils.ToInteger(inp[pos]);
                layers[cnt][row][col] = _val;
                layerV.push_back(_val);

                if (!_val) zeroCnt++;
                if (_val == 1) oneCnt++;
                if (_val == 2) twoCnt++;
                pos++;
            }
        }

        layerMap[cnt] = layerV;
        layerZeroCntMap[cnt] = zeroCnt;
        layerOneCntMap[cnt] = oneCnt;
        layerTwoCntMap[cnt] = twoCnt;
    }


    // Get layers having minimum zero digits
    int lyId = -1;
    int lyZeroCnt = 0;
 
    auto g = [layerZeroCntMap, &lyId, &lyZeroCnt](){
        for (auto && [k, v]: layerZeroCntMap) {
            if (lyId == -1) {
                lyId = k;
                lyZeroCnt = v;
                continue;
            }
            
            if (v < lyZeroCnt) {
                lyId = k;
                lyZeroCnt = v;
            }
        }
    };

    g();

    cout << "Layer having minimum zero's is " << lyId << endl;

    int ans = layerOneCntMap[lyId] * layerTwoCntMap[lyId];
    cout << "ans : " << ans << endl;

    /*
    for (int cnt = 0; cnt < layersCnt; cnt++) {
        cout << "Layer [" << cnt << "] values" << endl;
        vector <int> layerV;
        int zeroCnt = 0;
        for (int col = 0; col < cols; col++) {
            for (int row = 0; row < rows; row++) {
                cout << layers[cnt][row][col] << ", ";
                layerV.push_back(layers[cnt][row][col]);
            }
            cout << endl;
        }

        layerMap[cnt] = layerV;
    }*/

}

int main(int argc, const char * argv[]) {
    auto && [fname] = ParseProgramArguments(argc, argv);
    string pixelDtls = ReadInputFile((const char *)fname.c_str());
    
    CreateLayers(pixelDtls, 25, 6);
    return EXIT_SUCCESS;
}

