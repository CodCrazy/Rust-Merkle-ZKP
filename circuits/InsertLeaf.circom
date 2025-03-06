pragma circom  2.2.1;
include "./MerkleTree.circom";
include "../node_modules/circomlib/circuits/poseidon.circom";

// // inserts a leaf into a tree
// // checks that tree previously contained zero in the same position
template InsertLeaf(MAX_DEPTH) {
    signal input newRoot;
    signal input newLeaf;
    signal input pathIndices;
    signal input depth;
    signal input pathElements[MAX_DEPTH];


    // Compute indexBits once for both trees
    // Since Num2Bits is non deterministic, 2 duplicate calls to it cannot be
    // optimized by circom compiler
    component indexBits = Num2Bits(MAX_DEPTH);
    indexBits.in <== pathIndices;

    component treeAfter = RawMerkleTree(MAX_DEPTH);
    treeAfter.depth <== depth;
    for(var i = 0; i < MAX_DEPTH; i++) {
        treeAfter.indices[i] <== indexBits.out[i];
        treeAfter.siblings[i] <== pathElements[i];
    }
    treeAfter.leaf <== newLeaf;
    treeAfter.out === newRoot;
}

component main {public [newLeaf, newRoot, pathIndices]} = InsertLeaf(20);
