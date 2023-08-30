#include "Utilities.h"
#include <gtest/gtest.h>

int GetMarkerCharacter(const std::string &inpStr);

TEST(AocTest, GetMarkerCharacter) {
  std::string mystr = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
  Utilities utils;
  EXPECT_EQ(19, utils.GetMarkerCharacter(mystr));
}

int main(int argc, char **argv) {
  testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
