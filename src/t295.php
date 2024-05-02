<?php

namespace t295;

class MedianFinder {
    protected $lower;
    protected $upper;
    function __construct() {
        $this->lower = new \SplMaxHeap();
        $this->upper = new \SplMinHeap();
    }

    /**
     * @param Integer $num
     * @return NULL
     */
    function addNum($num) {
        if ($this->upper->isEmpty()) {
            $this->upper->insert($num);
            return;
        }

        if ($num >= $this->upper->top()) {
            $this->upper->insert($num);
            if ($this->upper->count() > $this->lower->count() + 1) {
                $el = $this->upper->extract();
                $this->lower->insert($el);
            }
        } else {
            $this->lower->insert($num);
            if ($this->lower->count() > $this->upper->count() + 1) {
                $el = $this->lower->extract();
                $this->upper->insert($el);
            }
        }
    }

    /**
     * @return Float
     */
    function findMedian() {
        $cu = $this->upper->count();
        $cl = $this->lower->count();
        if ($cu === $cl) {
            return ($this->upper->top() + $this->lower->top())/2;
        } elseif ($cu > $cl) {
            return $this->upper->top();
        } else {
            return $this->lower->top();
        }
    }
}

