<?php

class Solution {

    function inner($path, $from, $c, $t) {
        if (count($c) === $from) {
            return [];
        }
        $p = $c[$from];
        if ($p > $t) {
            return [];
        }
        if ($p === $t) {
            return [[...$path, $p]];
        }

        return [
            ...$this->inner([...$path, $p], $from, $c, $t - $p),
            ...$this->inner([...$path], $from + 1, $c, $t),
        ];
    }

    /// [], 0, [2 3 5], 8
    ///     [2], 0, [2 3 5], 6
    ///         [2 2], 0, [2 3 5], 4
    ///             [2 2 2], 0, [2 3 5], 2 => [2 2 2 2]
    ///

    /**
     * @param Integer[] $candidates
     * @param Integer $target
     * @return Integer[][]
     */
    function combinationSum($candidates, $target) {
        sort($candidates);
        return $this->inner([], 0, $candidates, $target);
    }
}