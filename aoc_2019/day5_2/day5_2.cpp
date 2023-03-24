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

const int ADD = 1;
const int MUL = 2;
const int INPUT = 3;
const int OUTPUT = 4;
const int JUMP_IF_TRUE = 5;
const int JUMP_IF_FALSE = 6;
const int LESS_THAN = 7;
const int EQUALS = 8;
const int HALT = 99;

const int INVALID_OPCODE = 10;

const int POSITION_MODE = 0;
const int IMMEDIATE_MODE = 1;

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
    vector<string> tmpV = utils->Split(prg_inp, ',') ;
    vector<int> && inpCodes = utils->ToIntegers(tmpV);

    return inpCodes;
}

tuple<int,int,int> GetParameters(const int & opCode, const vector<int> & inpCodes, string _ins, int pos) {
    Utilities utils;
    int param1Mode = utils.ToInteger(_ins[2]);
    int param2Mode = utils.ToInteger(_ins[1]);
    int tgtMode = utils.ToInteger(_ins[0]);

    int param1;
    int param2;
    int tgtPos;

    if ( opCode == INPUT || opCode == OUTPUT) {
        if (param1Mode == IMMEDIATE_MODE) {
            tgtPos = pos + 1;
        } else {
            tgtPos = inpCodes[pos+1];
        }
    } else {
        if (param1Mode == IMMEDIATE_MODE) {
            param1 = inpCodes[pos+1];
        } else {
            int _pos = inpCodes[pos+1];
            param1 = inpCodes[_pos];
        }

        if (param2Mode == IMMEDIATE_MODE) {
            param2 = inpCodes[pos+2];
        } else {
            int _pos = inpCodes[pos+2];
            param2 = inpCodes[_pos];
        }

        if (tgtMode == IMMEDIATE_MODE) {
            tgtPos = pos+3;
        } else {
            tgtPos = inpCodes[pos+3];
        }
    }

    return std::make_tuple(param1, param2, tgtPos);
}

void ReadAndExecuteCode(vector<int> & inpCodes) {
    for (int pos = 0; pos < int(inpCodes.size());) {
        int currCode = inpCodes[pos];
        if (currCode == HALT) break;

        Utilities utils;

        string insStr = std::to_string(currCode);
        insStr = utils.PrependZeros(insStr, 5);
        string opCodeStr = insStr.substr(insStr.length() - 2);
        int opCode = atoi((char *)opCodeStr.c_str());

        //cout << "Ins string ->" << insStr << endl;
        //cout << "op code ->" << opCode << endl;
        //cout << "Current pos ->" << pos << endl;
        auto && [param1, param2, tgtPos] = GetParameters(opCode, inpCodes, insStr, pos);

        switch(opCode) {
            case ADD:
                inpCodes[tgtPos] = param1 + param2;
                pos += 4;
                break;
            case MUL:
                inpCodes[tgtPos] = param1 * param2;
                pos += 4;
                break;
            case INPUT:
                cout << endl << "Input: ";
                cin >> inpCodes[tgtPos];
                pos += 2;
                break;
            case OUTPUT:
                cout << endl << "Output: " << inpCodes[tgtPos] << endl;
                pos += 2;
                break;
            case JUMP_IF_TRUE:
                if (param1) {
                    pos = param2;
                } else {
                    pos += 3;
                }
                break;
            case JUMP_IF_FALSE:
                if (!param1) {
                    pos = param2;
                } else {
                    pos += 3;
                }
                break;
            case LESS_THAN:
                if (param1 < param2) {
                    inpCodes[tgtPos] = 1;
                } else {
                    inpCodes[tgtPos] = 0;
                }
                pos += 4;
                break;
            case EQUALS:
                if (param1 == param2) {
                    inpCodes[tgtPos] = 1;
                } else {
                    inpCodes[tgtPos] = 0;
                }
                pos += 4;
                break;
            default:
                cout << "Invalid opcode. Exiting ..." << endl;
                exit(INVALID_OPCODE);
        }
    }
}

int main(int argc, const char * argv[]) {
    auto && [fname] = ParseProgramArguments(argc, argv);
    auto inpCodes = ReadInputFile((const char *)fname.c_str());

    for (auto && loc: inpCodes) {
        cout << loc << ",";
    }
    cout << endl;

    ReadAndExecuteCode(inpCodes);

    return EXIT_SUCCESS;
}

