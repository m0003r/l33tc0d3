<?php

namespace t63;

class Solution {

    /**
     * @param Integer[][] $obstacleGrid
     * @return Integer
     */
    function uniquePathsWithObstacles($obstacleGrid) {
        $h = count($obstacleGrid);
        $w = count($obstacleGrid[0]);
        $prev = array_fill(0, $w, 0);
        $cur = $prev;
        $prev[0] = 1;
        for ($y = 0; $y < $h; $y++) {
            // fill next line
            for ($x = 0; $x < $w; $x++) {
                if ($obstacleGrid[$y][$x]) {
                    $cur[$x] = 0;
                } else {
                    $cur[$x] = $prev[$x] + ($cur[$x-1] ?? 0);
                }
            }
            $prev = $cur;
        }
        return $cur[$w-1];
    }
}