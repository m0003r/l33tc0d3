<?php

namespace t8;

class Solution {

    /**
     * @param String $s
     * @return Integer
     */
    function myAtoi($s) {
        $int32_max = (1 << 31) - 1;
        $int32_min = -(1 << 31);
        $clamp = (1 << 31);

        $neg = false;
        $i = 0;
        $len = strlen($s);
        while ($s[$i] === ' ') {
            $i++;
        }

        if ($s[$i] === '+') {
            $i++;
        } elseif ($s[$i] === '-') {
            $neg = true;
            $i++;
        }

        $num = 0;
        while ($i < $len && $s[$i] >= '0' && $s[$i] <= '9') {
            $num = $num * 10 + (int)($s[$i]);
            if ($num >= $clamp) {
                break;
            }
            $i++;
        }

        if ($neg) {
            $num *= -1;
            $num = max($num, $int32_min);
        } else {
            $num = min($num, $int32_max);
        }

        return $num;
    }
}