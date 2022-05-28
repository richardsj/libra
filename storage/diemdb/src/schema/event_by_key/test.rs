// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use super::*;
use proptest::prelude::*;
use schemadb::{schema::fuzzing::assert_encode_decode, test_no_panic_decoding};

proptest! {
    #[test]
    fn test_encode_decode(
        event_key in any::<EventKey>(),
        seq_num in any::<u64>(),
        version in any::<Version>(),
        index in any::<u64>(),
    ) {
        assert_encode_decode::<EventByKeySchema>(&(event_key, seq_num), &(version, index));
    }
}

test_no_panic_decoding!(EventByKeySchema);
