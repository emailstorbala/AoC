/* Copyright [2022-2023] Balamurugan R<emailstorbala@gmail.com> */
#include <fmt/core.h>
#include <iostream>
#include <algorithm>
#include <chrono> // NOLINT [build/c++11]
#include <fstream>
#include <tuple>
#include <boost/program_options.hpp>
#include "Utilities.h"

using boost::program_options::notify;
using boost::program_options::parse_command_line;
using boost::program_options::store;
using boost::program_options::variables_map;
using boost::program_options::options_description;
using boost::program_options::value;
using boost::program_options::error;

using std::cout;
using std::endl;
using std::string;
using std::tuple;
using std::vector;
using std::map;
using std::ifstream;

namespace chrono = std::chrono;

const int ROCK      = 1;
const int PAPER     = 2;
const int SCISSORS  = 3;

const int WIN_BONUS = 6;
const int DRAW_BONUS = 3;

map <char, int> PLAYER1;
map <char, int> PLAYER2;


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
        fmt::print(stderr, "Exception: {}\n", ex.what());
        cout << prgDesc << endl;
        exit(3);
    }

    return make_tuple(filename);
}

vector <std::pair<char, char>> ReadInputFile(string inpfile) {
    vector <std::pair<char, char>> inpCtx;

    if (ifstream myfile(inpfile.c_str()); myfile.is_open()) { // NOLINT [-Wc++17-extensions]
        string line;
        while (getline(myfile, line)) {
            if (line.size() != 3) {
                throw std::runtime_error("Input file contains unexpected content");
            }
            char p1 = line[0];
            char p2 = line[2];
            inpCtx.push_back(std::make_pair(p1, p2));
        }
        myfile.close();
    } else {
        throw std::runtime_error(fmt::format("Unable to open file '{}'!",
                                 inpfile));
        exit(2);
    }

    return inpCtx;
}

int GetScore(int chosen, int otherChosen) {
    int score = 0;

    if (chosen == otherChosen) {
        score = chosen + DRAW_BONUS;
    } else if ( (chosen == ROCK && otherChosen == SCISSORS) ||
                (chosen == SCISSORS && otherChosen == PAPER) ||
                (chosen == PAPER && otherChosen == ROCK)) {
        score = chosen + WIN_BONUS;
    } else if ( (otherChosen == ROCK && chosen == SCISSORS) ||
                (otherChosen == SCISSORS && chosen == PAPER) ||
                (otherChosen == PAPER && chosen == ROCK)) {
        score = chosen;
    }

    return score;
}

int main(int argc, const char * argv[]) {
    auto start = chrono::system_clock::now();
    auto && [fname] = ParseProgramArguments(argc, argv); // NOLINT [-Wc++17-extensions]
    vector <std::pair<char, char>> inpList = ReadInputFile(fname);

    PLAYER1['A'] = ROCK;
    PLAYER1['B'] = PAPER;
    PLAYER1['C'] = SCISSORS;

    PLAYER2['X'] = ROCK;
    PLAYER2['Y'] = PAPER;
    PLAYER2['Z'] = SCISSORS;

    int totalScore = 0;
    for (auto && [p1, p2] : inpList) { // NOLINT [-Wc++17-extensions]
        totalScore += GetScore(PLAYER2[p2], PLAYER1[p1]);
    }

    fmt::print("The total score is {}\n", totalScore);
    auto end = chrono::system_clock::now();
    auto dur = chrono::duration_cast<chrono::nanoseconds>(end - start).count();
    cout << "Time taken: " << float(dur / 1000.0) << " mu.secs" << endl;

    return EXIT_SUCCESS;
}
