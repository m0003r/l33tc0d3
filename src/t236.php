<?php

namespace t236;

/**
 * Definition for a binary tree node.*/
class TreeNode
{
    public $val = null;
    public $left = null;
    public $right = null;

    function __construct($value)
    {
        $this->val = $value;
    }
}

class Solution
{
    /**
     * @param TreeNode $root
     * @param TreeNode $p
     * @param TreeNode $q
     * @return TreeNode
     */
    function lowestCommonAncestor($root, $p, $q)
    {
        // find both nodes in the tree
        $pp = $pq = null;
        $queue = [[$root]];

        while ($path = array_pop($queue)) {
            $el = end($path);
            if ($pp === null && $el === $p) {
                $pp = $path;
            }
            if ($pq === null && $el === $q) {
                $pq = $path;
            }
            if ($pq !== null && $pp !== null) {
                break;
            }
            if ($el->left) {
                $queue[] = [...$path, $el->left];
            }
            if ($el->right) {
                $queue[] = [...$path, $el->right];
            }
        }


        // calculate LCA
        $i = 0;
        while ($pp[$i] === $pq[$i]) {
            $i++;
        }

        return $pp[$i - 1];
    }
}
