<?php

namespace t33;
class Solution
{

    /**
     * @param Integer[] $nums
     * @param Integer $target
     * @return Integer
     */
    function search($nums, $target)
    {
        /**
         *
         * 1 <= nums.length <= 5000
         * -10^4 <= nums[i] <= 10^4
         * All values of nums are unique.
         * nums is an ascending array that is possibly rotated.
         * -10^4 <= target <= 10^4
         */

        $sv = $nums[0];
        $ev = $nums[count($nums) - 1];


        $ird = $target > $ev; // In RotateD

        $start = 0;
        $end = count($nums) - 1;

        // [0 1 2 3 4 5 6]
        // [4,5,6,7,0,1,2], target=0, ird = false
        // [- - - - 0 1 2]

        // 0, 3, 6
        // 4, 5, 6
        // 4, 4, 4

        while ($start <= $end) {
            $mid = (int)(($start + $end) / 2);
            $val = $nums[$mid];

            if ($val === $target) {
                return $mid;
            }

            if ($val < $sv && $ird) {
                $val = PHP_INT_MAX;
            } elseif ($val > $ev && !$ird) {
                $val = PHP_INT_MIN;
            }

            if ($val < $target) {
                $start = $mid + 1;
            } else {
                $end = $mid - 1;
            }
        }

        return -1;
    }
}
