<?php

class Solution {

    /**
     * @param String $s
     * @return String
     */
    function longestPalindrome($s)
    {
        $l = strlen($s);
        $data_odd = array_fill(0, $l, 1);
        $data_even = array_fill(0, $l, 0);
        $maxlen = 1;
        $maxpos = 0;

        //       aaaa
        // len=1 1111
        // len=2 2221
        // len=3 3321
        // len=4

        for ($len = 2; $len <= $l; $len++) {
            if ($len % 2) {
                $data = &$data_odd;
            } else {
                $data = &$data_even;
            }
            for ($i = 0; $i <= ($l - $len); $i++) {
                if ($s[$i] === $s[$i + $len - 1] && ($data[$i+1] === $len - 2)) {
                    $data[$i] = $len;
                    if ($len > $maxlen) {
                        $maxlen = $len;
                        $maxpos = $i;
                    }
                }
            }
        }

        return substr($s, $maxpos, $maxlen);
    }
}