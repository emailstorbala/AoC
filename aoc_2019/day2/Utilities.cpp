#include <memory>
#include "Utilities.h"

std::vector<std::string> Utilities::split(std::string _inp, char DELIMITER) {
    std::string temp;
    std::istringstream ss(_inp);
    std::vector<std::string> retV;

    while (std::getline(ss, temp, DELIMITER)) {
        retV.push_back(temp);
    }

    return retV;
}

std::vector<int> Utilities::ConvertVectorToIntType(std::vector<std::string> _inp) {
    std::vector <int> retV;
    for (auto && _tmp: _inp) {
        retV.push_back(atoi(_tmp.c_str()));
    }
    
    return retV;
}
