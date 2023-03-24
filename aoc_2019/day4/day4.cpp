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

const int NUM_OF_DIGITS = 6;

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

tuple<string, string> ReadInputFile(const char * inpfile) {
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
    return std::make_tuple(tmpV[0], tmpV[1]);
}

void FindPassword(long startRange, long endRange) {
    char * numStr = new char[NUM_OF_DIGITS];
    vector <long> passwords;

    for (long num = startRange; num < endRange; num++) {
        memset(numStr, '\0', NUM_OF_DIGITS);
        sprintf(numStr, "%ld", num);
        
        bool match = false;
        for(int pos = 0; pos < 5; pos++) {
            if (numStr[pos] == numStr[pos+1]) {
                match = true;
            }

            int currNum = int(numStr[pos]) - 48;
            int nextNum = int(numStr[pos+1]) - 48;

            if (currNum > nextNum) {
                match = false;
                break;
            }
        }
        if (match) passwords.push_back(num);
    }

    cout << "Total passwords -> " << passwords.size() << endl; 
}

int main(int argc, const char * argv[]) {
    auto && [fname] = ParseProgramArguments(argc, argv);
    auto && [startRange, endRange] = ReadInputFile((const char *)fname.c_str());
    cout << "Start Range, End Range ->" << startRange << "," << endRange << endl; 
    FindPassword(atol(startRange.c_str()), atol(endRange.c_str()));
    return EXIT_SUCCESS;
}

