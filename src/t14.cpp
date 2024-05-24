#include <vector>
#include <string>
#include <cassert>
#include <iostream>

class Solution {
public:
    std::string longestCommonPrefix(std::vector<std::string>& strs) {
        assert(strs.size() > 0);
        std::string res(strs[0]);
        size_t i;
        for (i = 0; i < res.size(); i++) {
            for (size_t j = 1; j < strs.size(); j++) {
                auto& s = strs[j];
                if (s.size() == i) {
                    goto fin;
                } 
                if (s[i] != res[i]) {
                    goto fin;
                }
            }
        }
fin:

        res.resize(i);
        return res;
    }
};

int main() {
    Solution s;
    std::vector<std::string> v;
    assert(s.longestCommonPrefix(v = {"flower","flow","flight"}) == "fl");
    assert(s.longestCommonPrefix(v = {"dog","racecar","car"}) == "");
    assert(s.longestCommonPrefix(v = {"float","float",""}) == "");
    assert(s.longestCommonPrefix(v = {"abc","abc","abc"}) == "abc");
    std::cout << "OK\n";
    return 0;
}