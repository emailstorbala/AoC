#include "Utilities.h"
#include <gtest/gtest.h>
#include <string_view>

using std::string;

TEST(AocUtilsTest, Split) {
  Utilities utils;
  string tmpStr = "Hello:World";
  auto splitData = utils.Split(tmpStr, ':');
  EXPECT_EQ(2, splitData.size());
  EXPECT_STREQ(splitData.at(0).c_str(), "Hello");
  EXPECT_STREQ(splitData.at(1).c_str(), "World");
}

TEST(AocUtilsTest, StringContainsString) {
  Utilities utils;
  string mainStr = "India is my country";
  string searchStr = "India";

  EXPECT_TRUE(utils.StringContainsString(mainStr, searchStr));
  EXPECT_FALSE(utils.StringContainsString(mainStr, string("Britain")));
}

int main(int argc, char **argv) {
  testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
