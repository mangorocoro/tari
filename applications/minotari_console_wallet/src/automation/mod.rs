// Copyright 2020. The Tari Project
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

pub mod commands;
pub mod error;
mod utils;
// removed temporarily add back in when used.
// mod prompt;

use serde::{Deserialize, Serialize};
use tari_common_types::{
    tari_address::TariAddress,
    transaction::TxId,
    types::{Commitment, PrivateKey, PublicKey, Signature},
};
use tari_core::transactions::{
    key_manager::TariKeyId,
    tari_amount::MicroMinotari,
    transaction_components::{EncryptedData, OutputFeatures},
};
use tari_script::{CheckSigSchnorrSignature, ExecutionStack, TariScript};

// Step 1 outputs for all with `PreMineSpendSessionInfo`
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
struct PreMineSpendStep1SessionInfo {
    session_id: String,
    fee_per_gram: MicroMinotari,
    recipient_info: Vec<RecipientInfo>,
    use_pre_mine_input_file: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
struct RecipientInfo {
    output_to_be_spend: usize,
    recipient_address: TariAddress,
}

impl SessionId for PreMineSpendStep1SessionInfo {
    fn session_id(&self) -> String {
        self.session_id.clone()
    }
}

// Step 2 outputs for self with `PreMineSpendPartyDetails`
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
struct PreMineSpendStep2OutputsForSelf {
    outputs_for_self: Vec<Step2OutputsForSelf>,
    alias: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Step2OutputsForSelf {
    output_index: usize,
    recipient_address: TariAddress,
    script_nonce_key_id: TariKeyId,
    sender_offset_key_id: TariKeyId,
    sender_offset_nonce_key_id: TariKeyId,
    pre_mine_script_key_id: TariKeyId,
}

// Step 2 outputs for leader with `PreMineSpendPartyDetails`
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
struct PreMineSpendStep2OutputsForLeader {
    outputs_for_leader: Vec<Step2OutputsForLeader>,
    alias: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Step2OutputsForLeader {
    output_index: usize,
    recipient_address: TariAddress,
    script_input_signature: CheckSigSchnorrSignature,
    public_script_nonce_key: PublicKey,
    public_sender_offset_key: PublicKey,
    public_sender_offset_nonce_key: PublicKey,
    dh_shared_secret_public_key: PublicKey,
    pre_mine_public_script_key: PublicKey,
}

// Step 3 outputs for self with `PreMineSpendEncumberAggregateUtxo`
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
struct PreMineSpendStep3OutputsForSelf {
    outputs_for_self: Vec<Step3OutputsForSelf>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
struct Step3OutputsForSelf {
    output_index: usize,
    tx_id: TxId,
}

// Step 3 outputs for parties with `PreMineSpendEncumberAggregateUtxo`
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
struct PreMineSpendStep3OutputsForParties {
    outputs_for_parties: Vec<Step3OutputsForParties>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
struct Step3OutputsForParties {
    output_index: usize,
    input_stack: ExecutionStack,
    input_script: TariScript,
    total_script_key: PublicKey,
    script_signature_ephemeral_commitment: Commitment,
    script_signature_ephemeral_pubkey: PublicKey,
    output_commitment: Commitment,
    sender_offset_pubkey: PublicKey,
    metadata_signature_ephemeral_commitment: Commitment,
    metadata_signature_ephemeral_pubkey: PublicKey,
    encrypted_data: EncryptedData,
    output_features: OutputFeatures,
    shared_secret: PublicKey,
}

// Step 4 outputs for leader with `PreMineSpendInputOutputSigs`
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
struct PreMineSpendStep4OutputsForLeader {
    outputs_for_leader: Vec<Step4OutputsForLeader>,
    alias: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
struct Step4OutputsForLeader {
    output_index: usize,
    script_signature: Signature,
    metadata_signature: Signature,
    script_offset: PrivateKey,
}

trait SessionId {
    fn session_id(&self) -> String;
}
