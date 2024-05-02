<?php

namespace t199;

/**
 * Definition for a binary tree node. */
class TreeNode
{
    public $val = null;
    public $left = null;
    public $right = null;

    function __construct($val = 0, $left = null, $right = null)
    {
        $this->val = $val;
        $this->left = $left;
        $this->right = $right;
    }
}

class Solutionx
{

    /**
     * @param TreeNode $root
     * @return Integer[]
     */
    function rightSideView($root)
    {
        return $root
            ? [$root->val, ...($this->rightSideView($root->right) + $this->rightSideView($root->left))]
            : [];
    }
}