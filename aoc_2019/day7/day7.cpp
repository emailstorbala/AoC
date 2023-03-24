#include <iostream>
#include <cstdio>
#include <unistd.h>
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

int ReadOutput(const char * inpfile) {
    int output;
    if (ifstream myfile(inpfile); myfile.is_open()) {
        string line;
        if (getline(myfile, line)) {
            sscanf(line.c_str(), "Output: %d\n", &output);
            cout << "Output -->" << output << "<--" << endl;
        }
        myfile.close();
    } else {
        std::cerr << "Unable to open file '" << inpfile << "'. Please check!" << endl;
        exit(2);
    }

    return output;
}

int RunIntCode(string phase, string inputFile) {
    int output = 0;

    for (size_t pos = 0; pos < phase.length(); pos++) {
        string command = "../day5_2/day5_2 --i " + inputFile + " |tail -n 1 >out.txt";
        cout << "Command executed -> " << command << endl;
        FILE* pipe = popen(command.c_str(), "w");

        if (!pipe) {
            cerr << "Couldn't start command." << endl;
            return 5;
        }

        char ins[10];
        memset(ins, '\0', 10);
        sprintf(ins, "%c\n", phase[pos]);
        fwrite(ins, 1, 2, pipe);
        char * outStr = new char[10];
        memset(outStr, '\0', 10);
        sprintf(outStr, "%d\n", output);
        fwrite(outStr, 1, strlen(outStr), pipe);
        pclose(pipe);
        output = ReadOutput("out.txt");
    }

    cout << "Final Output:" << output << endl;
    return output;
}

int main(int argc, const char * argv[]) {
    auto && [fname] = ParseProgramArguments(argc, argv);

    vector <int> outputs;
    Utilities utils;
    string phase = "43210";
    vector <string> permutations;
    utils.GetPermutations(phase, string(""), permutations);
    for (auto && ph: permutations) {
        int out = RunIntCode(ph, fname);
        outputs.push_back(out);
    }

    for (int out : outputs) {
        cout << "OUT:" << out << endl;
    }
    return EXIT_SUCCESS;
}

