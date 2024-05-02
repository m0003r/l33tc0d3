<?php

/**
 * Definition for a binary tree node.
 */ class TreeNode {
      public $val = null;
      public $left = null;
      public $right = null;
      function __construct($value) { $this->val = $value; }
  }


class Codec {
    function __construct() {

    }

    /**
     * @param TreeNode $root
     * @return String
     */
    function serialize($root) {
        $q = new SplQueue();
        $q->enqueue($root);
        $o = [];
        $lnn = 0;

        while (!$q->isEmpty()) {
            $el = $q->dequeue();
            if (!$el) {
                $o[] = 'null';
            } else {
                $o[] = $el->val;
                $lnn = count($o);
                $q->enqueue($el->left);
                $q->enqueue($el->right);
            }
        }

        return '[' . implode(',', array_slice($o, 0, $lnn)) . ']';
    }

    /**
     * @param String $data
     * @return TreeNode
     */
    function deserialize($data) {
        $s = explode(',', trim($data, '[]'));
        if (!count($s) || $s[0] === '') {
            return null;
        }

        $rootNode = new TreeNode((int)$s[0]);
        $l = count($s);

        $q = new SplQueue();
        $q->enqueue([$rootNode, 0]);
        $q->enqueue([$rootNode, 1]);

        for ($i = 1; $i < $l; $i++) {
            $el = $s[$i];

            [$p, $isRight] = $q->dequeue();
            if ($el === 'null') {
                continue;
            }
            if (!$isRight) {
                $p->left = new TreeNode($el);
                $q->enqueue([$p->left, 0]);
                $q->enqueue([$p->left, 1]);
            } else {
                $p->right = new TreeNode($el);
                $q->enqueue([$p->right, 0]);
                $q->enqueue([$p->right, 1]);
            }
        }

        return $rootNode;
    }
}

$ser = new Codec();
$deser = new Codec();
print_r($ans = $deser->deserialize('[]'));
print_r($ans = $deser->deserialize('[1,2,3]'));
print_r($ans = $deser->deserialize('[1,null,2,3]'));
print_r($ans = $deser->deserialize('[1,2,3,null,null,4,5]'));
//$data = $ser->serialize($root);
