#include <vector>
#include <sstream>
#include <string>

class Utilities {
    public:
        std::vector<std::string> Split(std::string _inp, char DELIMITER);
        std::vector<int> ToIntegers(std::vector<std::string> _inp);
        std::vector<long> ToLongV(std::vector<std::string> _inp);
        int ToInteger(char _chr);
        std::string PrependZeros(std::string _tmp, int _length);
        void GetPermutations(std::string str, std::string out, std::vector <std::string> & permutations);
};
