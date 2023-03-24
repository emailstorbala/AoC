/* Copyright [2022-2023] Balamurugan R<emailstorbala@gmail.com> */
#include <fmt/format.h>
#include <memory>
#include <algorithm>
#include "Utilities.h"

using std::string;

std::vector<std::string> Utilities::Split(std::string _inp, char DELIMITER) {
    std::string temp;
    std::istringstream ss(_inp);
    std::vector<std::string> retV;

    while (std::getline(ss, temp, DELIMITER)) {
        retV.push_back(temp);
    }

    return retV;
}

std::vector<int> Utilities::ToIntegers(std::vector<std::string> _inp) {
    std::vector <int> retV;
    for (auto && _tmp : _inp) {
        retV.push_back(atoi(_tmp.c_str()));
    }

    return retV;
}

std::vector<int64_t> Utilities::ToLongV(std::vector<std::string> _inp) {
    std::vector <int64_t> retV;
    for (auto && _tmp : _inp) {
        retV.push_back(atol(_tmp.c_str()));
    }

    return retV;
}

int Utilities::ToInteger(char _chr) {
    return static_cast<int>(_chr - '0');
}

int Utilities::ToAscii(int _inp) {
    return '0' + _inp;
}

std::tuple<std::string, std::string> Utilities::SplitStringExactHalf(const std::string & inp) {
    size_t inpLength = inp.size();
    auto part1 = inp.substr(0, inpLength/2);
    auto part2 = inp.substr(inpLength/2);

    return std::make_tuple(part1, part2);
}

std::list <string> Utilities::SimpleFileRead(const std::string &_fname) {
    std::list <string> lines;

    if (std::ifstream myfile(_fname.c_str()); myfile.is_open()) { // NOLINT [-Wc++17-extensions]
        string line;
        while (getline(myfile, line)) {
            lines.push_back(line);
        }
        myfile.close();
    } else {
        throw std::runtime_error(fmt::format("Unable to open file '{}'!", _fname));
        exit(2);
    }

    return lines;
}

std::set <char> Utilities::getCommonCharacters(const std::string &_str1, const std::string &_str2) {
    std::set <char> result;

    for (const char &chr : _str1) {
        if (_str2.find(chr) != string::npos) {
            result.insert(chr);
        }
    }

    return result;
}

std::string Utilities::PrependZeros(std::string _tmp, int _length) {
    int diff = _length - static_cast<int>(_tmp.length());
    std::string newStr = _tmp;

    for (int cnt = 1; cnt <= diff; cnt++) {
        newStr = "0" + newStr;
    }

    return newStr;
}

void Utilities::GetPermutations(std::string str, std::string out,
                                std::vector <std::string> & permutations) {
    if (str.size() == 0) {
        permutations.push_back(out);
        return;
    }

    for (int i = 0; i < static_cast<int>(str.size()); i++) {
        GetPermutations(str.substr(1), out + str[0], permutations);
        std::rotate(str.begin(), str.begin() + 1, str.end());
    }
}
