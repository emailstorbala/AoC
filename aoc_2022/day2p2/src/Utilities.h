/* Copyright [2022-2023] Balamurugan R<emailstorbala@gmail.com> */
#ifndef __SRC_UTILITIES_H__
#define __SRC_UTILITIES_H__
#include <vector>
#include <fstream>
#include <sstream>
#include <string>
#include <list>

class Utilities {
 public:
        std::vector<std::string> Split(std::string _inp, char DELIMITER);
        std::vector<int> ToIntegers(std::vector<std::string> _inp);
        std::vector<int64_t> ToLongV(std::vector<std::string> _inp);
        int ToInteger(char _chr);
        int ToAscii(int _inp);
        std::list <std::string> SimpleFileRead(std::string _fname);
        std::string PrependZeros(std::string _tmp, int _length);
        void GetPermutations(std::string str, std::string out,
                             std::vector <std::string> & permutations);
};
#endif  // __SRC_UTILITIES_H__
