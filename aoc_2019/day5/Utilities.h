#include <vector>
#include <sstream>
#include <string>

class Utilities {
    public:
        std::vector<std::string> split(std::string _inp, char DELIMITER);
        std::vector<int> ConvertVectorToIntType(std::vector<std::string> _inp);
        int ToInteger(char c);
        std::string PrependZeros(std::string _tmp, int _length);
};
