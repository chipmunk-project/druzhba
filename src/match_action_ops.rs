use std::collections::HashMap;
pub fn generate_schedule() -> HashMap <i32, Vec<String>> {
  let schedule : HashMap <i32, Vec<String>> = HashMap::new();
  schedule.insert(0, ["ipv4_routing_MATCH", "table_2_MATCH", "table_3_MATCH"]);
  schedule.insert(22, ["ipv4_routing_ACTION", "table_2_ACTION", "_condition_2", "_condition_0", "table_3_ACTION"]);
  schedule
}
