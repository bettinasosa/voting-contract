use pbc_zk::*;

/// Perform a zk computation on secret-shared data to count the number
/// of accepting votes (non-zero).
///
/// ### Returns:
///
/// The number of accepting votes.
#[zk_compute(shortname = 0x61)]
pub fn count_votes() -> (Sbi32, Sbi32, Sbi32) {
    // Initialize counters
    let mut votes_for: Sbi32 = Sbi32::from(0);
    let mut votes_option_a: Sbi32 = Sbi32::from(0);
    let mut votes_option_b: Sbi32 = Sbi32::from(0);


    // Count votes based on variable type
    for variable_id in secret_variable_ids() {
      let vote_value = load_sbi::<Sbi32>(variable_id);

      // Count non-zero values for each counter as per your specific logic
      if vote_value == Sbi32::from(1) {
          votes_for = votes_for + Sbi32::from(1);
      } else if vote_value == Sbi32::from(2) {
          votes_option_a = votes_option_a + Sbi32::from(1);
      } else if vote_value == Sbi32::from(3) {
          votes_option_b = votes_option_b + Sbi32::from(1);
      }
    }


    (votes_for, votes_option_a, votes_option_b)
}

