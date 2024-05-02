<?php

class Solution
{

    /**
     * @param Integer[][] $matrix
     * @return Integer[]
     */
    function spiralOrder($matrix)
    {
        $x0 = 0;
        $m = $x1 = count($matrix[0]);
        $y0 = 0;
        $n = $y1 = count($matrix);

        $size = $n * $m;

        ///   0 1 2 3
        /// 0 t t t t
        /// 1 l     r
        /// 2 b b b r

        $output = [];
        while ($size > 0) {
            // top side
            for ($x = $x0; $x < $x1; $x++) {
                $output[] = $matrix[$y0][$x];
                $size--;
            }
            $y0++;
            if ($size === 0) { break; }

            // right side
            for ($y = $y0; $y < $y1; $y++) {
                $output[] = $matrix[$y][$x1-1];
                $size--;
            }
            $x1--;
            if ($size === 0) { break; }

            // bottom side
            for ($x = $x1 - 1; $x >= $x0; $x--) {
                $output[] = $matrix[$y1-1][$x];
                $size--;
            }
            $y1--;
            if ($size === 0) { break; }

            // left side
            for ($y = $y1 - 1; $y >= $y0; $y--) {
                $output[] = $matrix[$y][$x0];
                $size--;
            }
            $x0++;
            if ($size === 0) { break; }
        }

        return $output;
    }
}