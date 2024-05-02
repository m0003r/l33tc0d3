<?php

namespace t218;

class Building {
    public $left;
    public $right;
    public $height;

    public function __construct(array $building)
    {
        $this->left = $building[0];
        $this->right = $building[1];
        $this->height = $building[2];
    }
}

class BuildingRightHeap extends \SplMinHeap {
    protected function compare($value1, $value2)
    {
        return $value2->right <=> $value1->right;
    }
}

class BuildingHeightHeap extends \SplMaxHeap {
    protected function compare($value1, $value2)
    {
        return $value1->height <=> $value2->height;
    }
}

class Solution {

    /**
     * @param Integer[][] $buildings
     * @return Integer[][]
     */
    function getSkyline($buildings) {
        $h = new BuildingHeightHeap();
        $points = [];
        foreach ($this->enumerateBuildings($buildings) as [$type, $b]) {
            if ($type === 'l') {
                if ($h->isEmpty() || $b->height > $h->top()->height) {
                    $lh = $b->height;
                    if (count($points) && end($points)[0] === $b->left) {
                        $l = end($points)[1];
                        array_pop($points);
                        if ($l > $lh) {
                            $lh = $l;
                        }
                    }
                    $points[] = [$b->left, $lh];
                }
                $h->insert($b);
            } else {
                if ($h->isEmpty()) {
                    if (end($points)[0] == $b->right) {
                        array_pop($points);
                        $lh = min($lh, end($points)[1]);
                    }
                    $points[] = [$b->right, 0];
                    continue;
                }

                $top = $h->top();
                if ($top === $b) {
                    $h->extract();
                    while (!$h->isEmpty() && $h->top()->right <= $b->right) {
                        $h->extract();
                    }
                }
                if ($h->isEmpty() || end($points)[1] !== $h->top()->height) {
                    if ($h->isEmpty()) {
                        $lh = 0;
                    } else {
                        $lh = $h->top()->height;
                    }
                    if (end($points)[0] == $b->right) {
                        array_pop($points);
                        $lh = min($lh, end($points)[1]);
                    }
                    $points[] = [$b->right, $lh];
                }
            }
        }

        return $points;
    }

    function enumerateBuildings($buildings) {
        $rh = new BuildingRightHeap();
        foreach ($buildings as $building) {
            $b = new Building($building);
            while (!$rh->isEmpty() && $rh->top()->right < $b->left) {
                yield ['r', $rh->extract()];
            }
            $rh->insert($b);
            yield ['l', $b];
        }
        while (!$rh->isEmpty()) {
            yield ['r', $rh->extract()];
        }
    }
}

// testing enumerateBuilding
$s = new Solution();
print_r(iterator_to_array($s->enumerateBuildings([[1,2,1],[1,2,2],[1,2,3]])));
print_r($s->getSkyline([[1,2,1],[1,2,2],[1,2,3]]));