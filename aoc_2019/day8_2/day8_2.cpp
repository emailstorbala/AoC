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

typedef vector <int> ROW;
typedef map <int, ROW> LAYER;
typedef vector <LAYER> LAYERS;

const int BLACK = 0;
const int WHITE = 1;
const int TRANSPARENT = 2;

int GetMinimumPixel(LAYERS layers, int layersCnt, int rowPos, int colPos) {
    int retPixel = -1;

    for (int cnt = 1; cnt < layersCnt; cnt++) {
        LAYER layer = layers[cnt];
        ROW row = layer[rowPos];
        int pixel = row[colPos];
        if (TRANSPARENT != pixel) {
            retPixel = pixel;
            break;
        }
    }

    return retPixel;
}

void GetFinalImage(LAYERS layers, int layersCnt, int rows, int cols) {

    const int FRONT_LAYER = 0;
    LAYER layer = layers[FRONT_LAYER];

    for (auto && [rowCnt, row]: layer) {
        for (int col = 0; col < cols; col++) {
            int pixel = row[col];
            cout << pixel << " ";
            if (TRANSPARENT == pixel) {
                row[col] = GetMinimumPixel(layers, layersCnt, rowCnt, col);
            }
        }
        cout << endl;
    }

    cout << endl;
    cout << endl;

    for (auto && [rowCnt, row]: layer) {
        for (int col = 0; col < cols; col++) {
            int pixel = row[col];

            if (pixel == WHITE) {
                cout << "0 ";
            } else {
                cout << "  ";
            }

            // For legible display
            if ((col+1)%5 == 0) {
                cout << " ";
            }
        }
        cout << endl;
    }

}

void CreateLayers(string inp, const int rows, const int cols) {
    int layersCnt = inp.length() / (rows * cols);
    cout << "layersCount ->" << layersCnt << endl;

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
    LAYERS _layers;

    for (int cnt = 0; cnt < layersCnt; cnt++) {
        LAYER _layer;
        for (int row = 0; row < rows; row++) {
            ROW _row;
            for (int col = 0; col < cols; col++) {
                int _val = utils.ToInteger(inp[pos]);
                _row.push_back(_val);
                pos++;
            }
            _layer[row] = _row;
        }
        _layers.push_back(_layer);
    }

    GetFinalImage(_layers, layersCnt, rows, cols);
}

int main(int argc, const char * argv[]) {
    auto && [fname] = ParseProgramArguments(argc, argv);
    string pixelDtls = ReadInputFile((const char *)fname.c_str());
    
    CreateLayers(pixelDtls, 6, 25);
    return EXIT_SUCCESS;
}

