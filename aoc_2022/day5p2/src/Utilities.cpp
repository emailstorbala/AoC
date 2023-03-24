/* Copyright [2022-2023] Balamurugan R<emailstorbala@gmail.com> */
#include <memory>
#include <ranges>
#include <algorithm>
#include <fmt/format.h>
#include "Utilities.h"

using std::string;
using std::vector;

vector<std::string> Utilities::Split(std::string _inp, char DELIMITER) const {
    std::string temp;
    std::istringstream ss(_inp);
    vector<std::string> retV;

    while (std::getline(ss, temp, DELIMITER)) {
        retV.emplace_back(temp);
    }

    return retV;
}

vector<int> Utilities::ToIntegers(vector<std::string> _inp) {
    vector <int> retV;
    for (auto && _tmp : _inp) {
        retV.push_back(atoi(_tmp.c_str()));
    }

    return retV;
}

vector<int64_t> Utilities::ToLongV(vector<std::string> _inp) {
    vector <int64_t> retV;
    for (auto && _tmp : _inp) {
        retV.push_back(atol(_tmp.c_str()));
    }

    return retV;
}

std::set<int> Utilities::GetNumberRange(int begin, int end) const {
    std::set <int> numRange;
    for (int i : std::views::iota(begin, end+1)) {
        numRange.emplace(i);
    }

    return numRange;
}

bool Utilities::StringContainsString(const std::string & inpStr, const std::string & tmp) const {
    return (inpStr.find(tmp) != string::npos);
}

bool Utilities::SetContainsSet(std::set<int> set1,  std::set<int> set2) const {
    // This checks and returns whether set1 contains set2
    return std::includes(set1.begin(), set1.end(), set2.begin(), set2.end());
}

bool Utilities::DoSetsHaveOverlapItems(std::set<int> set1, std::set<int> set2) const {
    bool hasOverlapItems = false;
    for (const int & set1_item : set1) {
        for (const int & set2_item : set2) {
            if (set1_item == set2_item) {
                hasOverlapItems = true;
            }
        }

        if (hasOverlapItems) break;
    }

    return hasOverlapItems;
}

vector<char> Utilities::ConcatenateVectors(const vector<char> & vec1,
                                           const vector<char> & vec2) const {
    vector <char> retVec;
    retVec.insert(retVec.end(), vec1.begin(), vec1.end());
    retVec.insert(retVec.end(), vec2.begin(), vec2.end());

    return retVec;
}

int Utilities::ToInteger(char _chr) const {
    int ret = _chr - '0';
    return ret;
}

int Utilities::ToAscii(int _inp) const {
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
                                vector <std::string> & permutations) {
    if (str.size() == 0) {
        permutations.push_back(out);
        return;
    }

    for (int i = 0; i < static_cast<int>(str.size()); i++) {
        GetPermutations(str.substr(1), out + str[0], permutations);
        std::rotate(str.begin(), str.begin() + 1, str.end());
    }
}
