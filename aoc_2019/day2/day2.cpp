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

const int ADD = 1;
const int MUL = 2;
const int HALT = 99;

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

    if (inpCtx.size() > 1) {
        cerr << "More than 1 inputs fine. Exiting" << endl;
        exit(2);
    }

    string prg_inp = inpCtx[0];
    std::unique_ptr<Utilities> utils = std::make_unique<Utilities>();
    vector<string> tmpV = utils->split(prg_inp, ',') ;
    vector<int> && inpCodes = utils->ConvertVectorToIntType(tmpV);

    return inpCodes;
}

void ReadAndExecuteCode(vector<int> & inpCodes) {
    for (int pos = 0; pos < inpCodes.size(); pos = pos + 4) {
        int opCode = inpCodes[pos];
        if (opCode == HALT) {
            break;
        }

        int pos1;
        int pos2;
        int tgtPos;

        switch (opCode) {
            case ADD:
                pos1 = inpCodes[pos+1];
                pos2 = inpCodes[pos+2];
                tgtPos = inpCodes[pos+3];
                inpCodes[tgtPos] = inpCodes[pos1] + inpCodes[pos2];
                break;
            case MUL:
                pos1 = inpCodes[pos+1];
                pos2 = inpCodes[pos+2];
                tgtPos = inpCodes[pos+3];
                inpCodes[tgtPos] = inpCodes[pos1] * inpCodes[pos2];
                break;
            default:
                cerr << "Invalid opcode. Exiting ..." << endl;
                exit(3);
        }
    }
}

int main(int argc, const char * argv[]) {
    auto && [fname] = ParseProgramArguments(argc, argv);
    auto inpCodes = ReadInputFile((const char *)fname.c_str());

    for (auto && loc: inpCodes) {
        cout << loc << ",";
    }
    
    ReadAndExecuteCode(inpCodes);

    cout << endl;
    for (auto && loc: inpCodes) {
        cout << loc << ",";
    }
    cout << endl;

    return EXIT_SUCCESS;
}

