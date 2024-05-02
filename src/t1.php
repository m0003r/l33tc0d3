<?php

namespace t1;

class Solution {

    /**
     * @param Integer[] $nums
     * @param Integer $target
     * @return Integer[]
     */
    function twoSum($nums, $target) {
        $indices = [];

        foreach ($nums as $idx => $num) {
            $rem = $target - $num;
            if (isset($indices[$rem])) {
                return [$idx, $indices[$rem]];
            }
            $indices[$num] = $idx;
        }

        return [];
    }
}
