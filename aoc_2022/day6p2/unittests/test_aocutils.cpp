#include "AocUtils.h"
#include <gtest/gtest.h>

TEST(AocUtilsTest, GetMarkerCharacter) {
  std::string mystr = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
  AocUtils utils;
  EXPECT_EQ(19, utils.GetMarkerCharacter(mystr));
}

int main(int argc, char **argv) {
  testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
