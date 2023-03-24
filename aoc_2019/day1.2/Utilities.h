#include <vector>
#include <sstream>
#include <string>

class Utilities {
    public:
        std::vector<std::string> split(std::string _inp, char DELIMITER);
        std::vector<int> ToIntVector(std::vector<std::string> _inp);
};
