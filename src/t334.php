<?php

namespace t334;

class Solution {

    /**
     * @param Integer[] $nums
     * @return Boolean
     */
    function increasingTriplet($nums) {
        $min1 = PHP_INT_MAX - 1;
        $min2 = PHP_INT_MAX;
        foreach ($nums as $num) {
            if ($num <= $min1) {
                $min1 = $num;
            } elseif ($num < $min2) {
                $min2 = $num;
            } elseif ($num > $min2) {
                return true;
            }
        }
        return false;
    }

    // [1 1 2 2 1 1 3 3]
    // 1 2


}
