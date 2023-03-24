#include <memory>
#include <algorithm>
#include "Utilities.h"

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
    for (auto && _tmp: _inp) {
        retV.push_back(atoi(_tmp.c_str()));
    }

    return retV;
}

int Utilities::ToInteger(char _chr) {
    return int(_chr - '0');
}

std::string Utilities::PrependZeros(std::string _tmp, int _length) {
    int diff = _length - _tmp.length();
    std::string newStr = _tmp;

    for ( int cnt = 1; cnt <= diff; cnt++) {
        newStr = "0" + newStr;
    }

    return newStr;
}

void Utilities::GetPermutations(std::string str, std::string out, std::vector <std::string> & permutations) {
    if (str.size() == 0) {
        permutations.push_back(out);
        return;
    }

    for (int i = 0; i < int(str.size()); i++) {
        GetPermutations(str.substr(1), out + str[0], permutations);
        std::rotate(str.begin(), str.begin() + 1, str.end());
    }
}
