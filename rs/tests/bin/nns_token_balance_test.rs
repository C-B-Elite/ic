use anyhow::Result;

use ic_tests::driver::new::group::SystemTestGroup;
use ic_tests::ledger_tests::token_balance::{config, test};
use ic_tests::systest;

fn main() -> Result<()> {
    SystemTestGroup::new()
        .with_setup(config)
        .add_test(systest!(test))
        .execute_from_args()?;
    Ok(())
}
