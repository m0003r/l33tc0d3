#include <vector>
#include <algorithm>
#include <cassert>
#include <numeric>
#include <iostream>

class Solution {
public:
    int largestSumAfterKNegations(std::vector<int>& nums, int k) {
        std::sort(nums.begin(), nums.end());
        int sum = 0;
        auto left = nums.begin();
        while (k > 0) {
            if (left == nums.end()) {
                if (k % 2 == 0) {
                    return sum;
                } else {
                    auto prev = std::prev(left);
                    return sum + 2 * (*prev);
                }
            } else if (*left < 0) {
                k--;
                sum -= *left;
            } else if (*left == 0 || k % 2 == 0) {
                k = 0;
                break;
            } else {
                // exact 1 flip
                if (left == nums.begin()) {
                    // nothing to do but flip it
                    sum -= *left;
                } else {
                    auto prev = std::prev(left);
                    sum += *prev;
                    sum += std::abs(*prev + *left);
                }
                k = 0;
            }      
            
            left = std::next(left);
        }

        return std::accumulate(left, nums.end(), sum);
    }
};

int main() {
    Solution s;
    std::vector<int> v;
    assert(s.largestSumAfterKNegations(v = {4,2,3}, 1) == 5);
    assert(s.largestSumAfterKNegations(v = {3,-1,0,2}, 3) == 6);
    assert(s.largestSumAfterKNegations(v = {2,-3,-1,5,-4}, 2) == 13);
    assert(s.largestSumAfterKNegations(v = {-2,9,9,8,4}, 5) == 32);
    assert(s.largestSumAfterKNegations(v = {-4,-3,-5}, 3) == 12);
    assert(s.largestSumAfterKNegations(v = {-4,-3,-5}, 4) == 6);
    std::cout << "OK\n"; 
    return 0;
}