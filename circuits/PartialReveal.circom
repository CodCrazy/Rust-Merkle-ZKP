include "../node_modules/circomlib/circuits/poseidon.circom";

template PartialReveal(depth) {
   // Inputs
   signal input merchant_leaf;
   signal input merchant_root;
   signal input record_leaf;
   signal input record_root;
   signal input merchant_permissions;
   signal input requested_fields;
   signal input merchant_path_elements[depth];
   signal input merchant_path_indices[depth];
   signal input record_path_elements[depth];
   signal input record_path_indices[depth];

   // Internal Signals
   signal valid_merchant;
   signal valid_record;
   signal has_permission;

   // Merkle Proof Verification for Merchant using Poseidon
   var current_hash = merchant_leaf;
   for (var i = 0; i < depth; i++) {
      signal tmp;
      if (merchant_path_indices[i] == 0) {
         tmp = [merchant_path_elements[i], current_hash];
      } else {
         tmp = [current_hash, merchant_path_elements[i]];
      }
      current_hash = Poseidon(2);
      current_hash.inputs[0] <== tmp[0];
      current_hash.inputs[1] <== tmp[1];
   }
   valid_merchant <== (current_hash === merchant_root);

   // Merkle Proof Verification for Record using Poseidon
   current_hash = record_leaf;
   for (var i = 0; i < depth; i++) {
      signal tmp;
      if (record_path_indices[i] == 0) {
         tmp = [record_path_elements[i], current_hash];
      } else {
         tmp = [current_hash, record_path_elements[i]];
      }
      current_hash = Poseidon(tmp);
   }
   valid_record <== (current_hash === record_root);

   // Permission Check
   has_permission <== (merchant_permissions & requested_fields) == requested_fields;

   // Ensure all conditions are met
   valid_merchant * valid_record * has_permission === 1;
}

component main {public [merchant_leaf, merchant_root, record_leaf, record_root, merchant_permissions, requested_fields]} = PartialReveal(depth);

