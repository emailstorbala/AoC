/* Copyright [2022-2023] Balamurugan R<emailstorbala@gmail.com> */
#ifndef __SRC_UTILITIES_H__
#define __SRC_UTILITIES_H__
#include <map>
#include <vector>
#include <set>
#include <string>
#include <string_view>
#include <list>
#include <tuple>

class Utilities {
 public:
        std::vector<std::string> Split(std::string _inp, char DELIMITER) const;
        std::vector<int> ToIntegers(std::vector<std::string> _inp);
        std::vector<int64_t> ToLongV(std::vector<std::string> _inp);
        std::set<int> GetNumberRange(int begin, int end) const;
        bool StringContainsString(const std::string & inpStr, const std::string & tmp) const;
        int ToInteger(char _chr) const;
        int ToAscii(int _inp) const;
        bool SetContainsSet(std::set<int> set1,  std::set<int> set2) const;
        bool DoSetsHaveOverlapItems(std::set<int> set1, std::set<int> set2) const;
        std::vector<char> ConcatenateVectors(const std::vector<char> & vec1,
                                             const std::vector<char> & vec2) const;
        std::tuple<std::string, std::string> SplitStringExactHalf(const std::string & inp);
        std::list <std::string> SimpleFileRead(std::string_view _fname);
        std::set <char> getCommonCharacters(std::string_view str1, std::string_view _str2);
        std::string PrependZeros(std::string_view _tmp, int _length);
        void GetPermutations(std::string str, std::string out,
                             std::vector <std::string> & permutations);
        bool ContainsUniqueCharacters(std::string_view tmp);
};
#endif  // __SRC_UTILITIES_H__
