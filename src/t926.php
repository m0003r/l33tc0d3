<?php

namespace t926;

class Solution {

    /**
     * @param String $s
     * @return Integer
     */
    function minFlipsMonoIncr($str) {
        $minZero = 0;
        $minOne = 0;
        $str = ltrim($str, '0');
        $str = rtrim($str, '1');
        if (strlen($str) === 0) {
            return 0;
        }

        for ($i = 0; $i < strlen($str); $i++) {
            $c = $str[$i];
            if ($c === '0') {
                $minOne++;
            } else {
                $minOne = min($minOne, $minZero);
                $minZero++;
            }
        }
    }
}

// we have sequence that can accept 0's and give 0:
// we have sequence2 that cant accept 0's and give 0: s2++;
// we have sequence that can accept