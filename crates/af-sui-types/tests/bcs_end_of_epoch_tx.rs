use sui_sdk_types::Transaction;
use sui_sdk_types::bcs::FromBcs;

// epoch = 594 (testnet)
//
// query epochLastTransaction($epoch: UInt53) {
//   epoch(id: $epoch) {
//     epochId
//     checkpoints(last: 1) {
//       nodes {
//         transactionBlocks(last: 1) {
//           nodes {
//             digest
//             kind {
//               __typename
//             }
//             bcs
//           }
//         }
//       }
//     }
//   }
// }
const BASE64_BCS: &str = "AAUCAlICAAAAAAAAdACtAAAAAAAAUwIAAAAAAABGAAAAAAAAAIDnfFEREwAAGB4InlYEAAAgoP6XPhAAAGAJiQEqAAAAU49m/5MBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAA=";

#[test]
fn transaction_deser() {
    let _: Transaction = Transaction::from_bcs_base64(BASE64_BCS).unwrap();
}
