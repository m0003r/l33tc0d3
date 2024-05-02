<?php

namespace t174;

class Solution {

    /**
     * @param Integer[][] $dungeon
     * @return Integer
     */
    function calculateMinimumHP($dungeon) {
        $w = count($dungeon[0]);
        $h = count($dungeon);

        $d = array_fill(0, $h, []);
        $d[$h-1][$w-1] = max(1, 1 - $dungeon[$h-1][$w-1]);

        // -3 -5
        //  4  9

        // fill last row
        for ($x = $w-2; $x >= 0; $x--) {
            $prev = $d[$h-1][$x+1];
            $curr = $dungeon[$h-1][$x];
            $d[$h-1][$x] = max(1, $prev - $curr);
        }
        for ($y = $h-2; $y >= 0; $y--) {
            $prev = $d[$y+1][$w-1];
            $curr = $dungeon[$y][$w-1];
            $d[$y][$w-1] = max(1, $prev - $curr);
        }


        for ($x = $w - 2; $x >= 0; $x--) {
            for ($y = $h - 2; $y >= 0; $y--) {
                $curr = $dungeon[$y][$x];
                $prev1 = $d[$y+1][$x];
                $prev2 = $d[$y][$x+1];
                $d[$y][$x] = max(1, min($prev1 - $curr, $prev2 - $curr));
            }
        }

        /*

        echo "\nSRC\n";
        // output formatted grid
        for ($y = 0; $y < $h; $y++) {
            for ($x = 0; $x < $w; $x++) {
                $s = (string)$dungeon[$y][$x];
                echo str_pad($s, 5);
            }
            echo "\n";
        }

        echo "\nDST\n";
        // output formatted grid
        for ($y = 0; $y < $h; $y++) {
            for ($x = 0; $x < $w; $x++) {
                $s = (string)$d[$y][$x];
                echo str_pad($s, 5);
            }
            echo "\n";
        } */

        return $d[0][0];
    }
}



$s = new Solution();
echo $s->calculateMinimumHP([[1,-3,3],[0,-2,0],[-3,-3,-3]]). "\n";
echo $s->calculateMinimumHP([[0, -5, 20],[-5,0,0],[15,0,0]]). "\n";
echo $s->calculateMinimumHP([[-2,-3,3],[-5,-10,1],[10,30,-5]]). "\n";
echo $s->calculateMinimumHP([[0]]) . "\n";