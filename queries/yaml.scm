(block_mapping_pair 
  key: (flow_node
    (plain_scalar
      (string_scalar) @package))
  value: (block_node
    (block_mapping
      (block_mapping_pair
       key: (flow_node
         (plain_scalar
          (
           (string_scalar) @key
           (#eq? @key "version")
          )))
       value: (flow_node
          (double_quote_scalar) @version)
      )))
)
