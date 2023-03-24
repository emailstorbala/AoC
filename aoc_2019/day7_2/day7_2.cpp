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

int ReadAndExecuteCode(vector<int> & inpCodes, int & pos, std::stack <long> inpS, std::stack <long> & outS, int phase = 0) {
    int opCode = 0;

    if (phase) {
        inpS.push(phase);
    }

    for (; pos < int(inpCodes.size());) {
        int currCode = inpCodes[pos];
        if (currCode == HALT) {
            opCode = HALT;
            break;
        }

        Utilities utils;

        string insStr = std::to_string(currCode);
        insStr = utils.PrependZeros(insStr, 5);
        string opCodeStr = insStr.substr(insStr.length() - 2);
        opCode = atoi((char *)opCodeStr.c_str());

        if (INPUT == opCode && inpS.empty() && !outS.empty()) {
            break;
        }

        //cout << "Ins string ->" << insStr << endl;
        //cout << "op code ->" << opCode << endl;
        //cout << "Current pos ->" << pos << endl;
        auto && [param1, param2, tgtPos] = GetParameters(opCode, inpCodes, insStr, pos);

        long tmpInp = 0;
        long tmpOut = 0;

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
                tmpInp = inpS.top();
                //cout << "Inp:" << tmpInp << endl;
                inpCodes[tgtPos] = tmpInp;
                inpS.pop();
                pos += 2;
                break;
            case OUTPUT:
                tmpOut = inpCodes[tgtPos];
                //cout << "Out:" << tmpOut << endl;
                outS.push(tmpOut);
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

    return opCode;
}

typedef std::pair<int, vector <int>> SNAPSHOT;
typedef map<int, SNAPSHOT> SNAPSHOTS;

long StartAmps(string phases, vector <int> inpCodes) {
    Utilities utils;
    long output = 0;

    SNAPSHOTS snaps;    

    for (size_t cnt = 0; cnt < phases.length(); cnt++) {
        int phase = utils.ToInteger(phases[cnt]);
        int pos = 0;
        vector <int> tmpCodes = inpCodes;
        std::stack <long> inpS;
        std::stack <long> outS;
        inpS.push(output);
        ReadAndExecuteCode(tmpCodes, pos, inpS, outS, phase);

        while (outS.empty()) {
            std::this_thread::sleep_for(std::chrono::milliseconds(10));
        }
        output = outS.top();
        outS.pop();
        SNAPSHOT snap = std::pair(pos, tmpCodes);
        snaps[cnt] = snap;
    }

    int ret = 0;
    int cnt = 0;

    while (HALT != ret) {
        std::stack <long> inpS;
        std::stack <long> outS;
        inpS.push(output);

        auto && [pos, tmpCodes] = snaps[cnt]; 

        int tmpRet = ReadAndExecuteCode(tmpCodes, pos, inpS, outS);
        cout << "Ret code ->" << ret << endl;

        if ( (cnt == int(phases.length() - 1)) and (HALT == tmpRet)) {
            ret = HALT;
        } 

        while (outS.empty()) {
            std::this_thread::sleep_for(std::chrono::milliseconds(10));
        }
        output = outS.top();
        outS.pop();
        cout << "Output -> " << output << endl;
        SNAPSHOT snap = std::pair(pos, tmpCodes);
        snaps[cnt] = snap;

        if (cnt == int(phases.length() - 1)) {
            cnt = 0;
        } else {
            cnt++;
        }
    }

    return output;
}

int main(int argc, const char * argv[]) {
    auto && [fname] = ParseProgramArguments(argc, argv);
    auto inpCodes = ReadInputFile((const char *)fname.c_str());

    for (auto && loc: inpCodes) {
        cout << loc << ",";
    }
    cout << endl;

    /***/
    string phases = "98765";
    vector <string> permutations;
    vector <long> outputs;
    Utilities utils;
    utils.GetPermutations(phases, string(""), permutations);
    for (auto && ph: permutations) {
        cout << "-------------------" << endl;
        cout << "ph -> " << ph << endl;
        long output = StartAmps(ph, inpCodes);
        cout << "----------------" << endl;
        cout << endl;
        outputs.push_back(output);
    }

    cout << "Max thruster :" << *std::max_element(outputs.begin(), outputs.end()) << endl;
    /***/

    /**** /
    string phases = "97856";
    long output = StartAmps(phases, inpCodes);
    cout << "output :" << output << endl;
    / ***/

    return EXIT_SUCCESS;
}

