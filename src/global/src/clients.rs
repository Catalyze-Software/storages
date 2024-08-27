use catalyze_shared::{
    group_with_members::{GroupFilter, GroupSort, GroupWithMembers},
    storage_clients, StorageClientInsertable,
};

use crate::state::GROUP_CANISTER;

pub fn groups() -> impl StorageClientInsertable<GroupWithMembers, GroupFilter, GroupSort> {
    storage_clients::groups(&GROUP_CANISTER)
}
