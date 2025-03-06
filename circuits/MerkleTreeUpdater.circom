include "./MerkleTree.circom";

// inserts a leaf into a tree
// checks that tree previously contained zero in the same position
template MerkleTreeUpdater(depth, zeroLeaf) {
    signal input current_root;
    signal input new_root;
    signal input new_leaf;
    signal input pathIndices[depth];
    signal input pathElements[depth];

    // Compute indexBits once for both trees
    // Since Num2Bits is non deterministic, 2 duplicate calls to it cannot be
    // optimized by circom compiler
    component indexBits = Num2Bits(depth);
    indexBits.in <== pathIndices;

    component treeBefore = RawMerkleTree(depth);
    for(var i = 0; i < depth; i++) {
        treeBefore.pathIndices[i] <== indexBits.out[i];
        treeBefore.pathElements[i] <== pathElements[i];
    }
    treeBefore.leaf <== zeroLeaf;
    treeBefore.root === current_root;

    component treeAfter = RawMerkleTree(depth);
    for(var i = 0; i < depth; i++) {
        treeAfter.pathIndices[i] <== indexBits.out[i];
        treeAfter.pathElements[i] <== pathElements[i];
    }
    treeAfter.leaf <== new_leaf;
    treeAfter.root === new_root;
}

component main(public [current_root, new_leaf, new_root]) = MerkleTreeUpdater(depth, zeroLeaf);