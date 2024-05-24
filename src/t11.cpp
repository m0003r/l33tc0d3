#include <vector>
#include <iostream>
#include <cassert>

class Solution {
public:
    int maxArea(std::vector<int>& height) {
        auto left = height.cbegin();
        auto right = height.cend() - 1;

        int max = 0;
        while (right > left) {
            max = std::max(max, static_cast<int>(std::min(*left, *right) * (right - left)));
            if (*right > *left) {
                left++;
            } else {
                right--;
            }
        }
        
        return max;
    }

    int _maxArea(std::vector<int> height) {
        return maxArea(height);
    }
};

int main() {
    Solution s{};

    assert(s._maxArea({1,8,6,2,5,4,8,3,7}) == 49);
    assert(s._maxArea({1,1}) == 1);

    return 0;
}