#include <vector>
#include <bitset>
#include <cassert>
#include <iostream>

class Solution {
    std::bitset<9> validator;

    bool canAccept(char c) {
        if (c == '.') {
            return true;
        }
        c -= '0';
        if (validator[c]) {
            return false;
        }
        validator[c] = true;
        return true;
    }
public:
    bool isValidSudoku(std::vector<std::vector<char>>& board) {
        
        for (int i = 0; i < 9; i++) {
            validator.reset();
            for (int j = 0; j < 9; j++) {
                if (!canAccept(board[i][j])) {
                    return false;
                }
            }

            validator.reset();
            for (int j = 0; j < 9; j++) {
                if (!canAccept(board[j][i])) {
                    return false;
                }
            }

            validator.reset();
            int start_row = (i / 3) * 3;
            int start_col = (i % 3) * 3;
            for (int j = 0; j < 9; j++) {
                int row = start_row + j / 3;
                int col = start_col + j % 3;
                if (!canAccept(board[row][col])) {
                    return false;
                }
            }
        }
        return true;
    }
};

int main() {
    std::vector<std::vector<char>> board = {{'5','3','.','.','7','.','.','.','.'}
,{'6','.','.','1','9','5','.','.','.'}
,{'.','9','8','.','.','.','.','6','.'}
,{'8','.','.','.','6','.','.','.','3'}
,{'4','.','.','8','.','3','.','.','1'}
,{'7','.','.','.','2','.','.','.','6'}
,{'.','6','.','.','.','.','2','8','.'}
,{'.','.','.','4','1','9','.','.','5'}
,{'.','.','.','.','8','.','.','7','9'}};

    Solution s;
    assert(s.isValidSudoku(board));

    board[0][0] = '8';
    assert(!s.isValidSudoku(board));

    return 0;
}