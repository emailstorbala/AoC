#include <memory>
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

