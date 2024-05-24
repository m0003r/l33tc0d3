#include <iostream>
#include <limits>
#include <cassert>

constexpr const int int_min = std::numeric_limits<int>::min();
constexpr const int int_max = std::numeric_limits<int>::max();

class Solution {
public:
    int divide(int x, int y) {
        bool neg = true;
        if (x == y) {
            return 1;
        }
        if (x == 0) {
            return 0;
        }
        assert(y != 0);
        if (y == int_min) {
            // nothing can fill it except itself
            return 0;
        }

        // so there is:
        // abs(x), abs(y) <= int_max

        if (x > 0) {
            x = -x;
            neg = !neg;
        }
        if (y < 0) {
            y = -y;
            neg = !neg;
        }

        if (x + y == 0) {
            return neg ? -1 : 1;
        }

        // now x < 0, y > 0; neg is sign of the answer
        int answer = 0;
        int rem = x;

        while (rem + y < 0) {
            int shifted = y;
            int shift = 0;
            do {
                if (rem + shifted >= 0) {
                    break;
                }
                shift++;
                shifted <<= 1;
            } while (shifted > 0);

            answer += (1 << shift);
            rem += (y << shift);
            
        }

        return neg ? -answer : answer;
    }
};

int main() {
    Solution s;
    assert(s.divide(1, 1) == 1);
    assert(s.divide(int_max, (int_max >> 1) + 1) == 1);
    assert(s.divide(int_max, int_min) == 0);
    assert(s.divide(int_min, int_max) == -1);
    assert(s.divide(1, -1) == -1);
    assert(s.divide(5, 5) == 1);
    assert(s.divide(10, 5) == 2);
    assert(s.divide(100, 7) == 14);
    assert(s.divide(-50, 6) == -8);
    return 0;
}