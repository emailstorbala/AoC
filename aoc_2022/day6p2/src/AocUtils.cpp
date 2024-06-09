#include "Utilities.h"
#include "AocUtils.h"

size_t AocUtils::GetMarkerCharacter(const string &inpStr) {
  const int cmpStrLen = 14;
  size_t uniqCharPos = string::npos;

  for (size_t pos = 0; (pos + cmpStrLen) < inpStr.size(); pos++) {
    string locStr = {inpStr.begin() + pos, inpStr.begin() + pos + cmpStrLen};
    // fmt::print("locStr -> {}\n", locStr);
    if (Utilities utils; utils.ContainsUniqueCharacters(locStr)) {
      // fmt::print("New char pos->{}\n", pos + cmpStrLen);
      uniqCharPos = pos + cmpStrLen;
      break;
    }
  }

  return uniqCharPos;
}
