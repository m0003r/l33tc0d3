<?php

namespace t46;

class Solution {

    /**
     * @param Integer[] $nums
     * @return Integer[][]
     */
    function permute($nums) {
        $c = count($nums);
        if ($c === 1) {
            return [$nums];
        }

        $tmp = array_slice($nums, 1);
        $output = [];
        for ($i = 0; $i < $c; $i++) {
            foreach ($this->permute($tmp) as $pt) {
                $output[] = [$nums[$i], ...$pt];
            }

            if ($i !== $c - 1) {
                $tmp[$i] = $nums[$i];
            }
        }

        return $output;
    }
}