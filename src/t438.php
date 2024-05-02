<?php

namespace t438;

class Solution
{
    var $lack;
    var $dist;

    function add_letter($c, $d)
    {
        $ol = ($this->lack[$c] ?? 0);
        $nl = $this->lack[$c] = $ol + $d;

        if ($ol === 0) {
            $this->dist--;
        } elseif ($nl === 0) {
            $this->dist++;
        }
    }

    /**
     * @param String $s
     * @param String $p
     * @return Integer[]
     */
    function findAnagrams($s, $p)
    {
        $output = [];
        $sl = strlen($s);
        $pl = strlen($p);

        if ($sl < $pl) {
            return [];
        }

        $this->lack = [];
        $this->dist = 0;

        // analyze string $p
        for ($i = 0; $i < $pl; $i++) {
            $this->add_letter($p[$i], 1);
        }

        // for each sliding window of $s check anagrammity
        $pos = 0;
        // build first
        for ($i = 0; $i < $pl; $i++) {
            $c = $s[$i];
            $this->add_letter($c, -1);
        }

        if ($this->dist === 0) {
            $output[] = $pos;
        }

        for ($pos = 1; $pos <= $sl - $pl; $pos++) {
            $this->add_letter($s[$pos - 1], 1);
            $this->add_letter($s[$pos + $pl - 1], -1);
            if ($this->dist === 0) {
                $output[] = $pos;
            }
        }

        return $output;
    }
}
