<?php

class Solution {

    function adj($w1, $w2) {
        $d = 0;
        for ($i = 0; $i < strlen($w1); $i++) {
            if ($w2[$i] !== $w1[$i]) {
                $d++;
            }
            if ($d > 1) {
                return false;
            }
        }
        return $d === 1;
    }

    /**
     * @param String $beginWord
     * @param String $endWord
     * @param String[] $wordList
     * @return Integer
     */
    function ladderLength($beginWord, $endWord, $wordList) {
        if (!in_array($endWord, $wordList)) {
            return 0;
        }

        $dists = [$beginWord => 1];
        $queue = [$beginWord];

        while ($w = array_shift($queue)) {
            foreach ($wordList as $pn) {
                if (isset($dists[$pn])) {
                    continue;
                }

                if ($this->adj($w, $pn)) {
                    if ($pn === $endWord) {
                        return $dists[$w] + 1;
                    }

                    $dists[$pn] = $dists[$w] + 1;
                    $queue[] = $pn;
                }
            }
        }

        return 0;
    }
}

$s = new Solution();
$r = $s->ladderLength("hit","cog",
    ["hot","dot","dog","lot","log","cog"]);
print_r($r);