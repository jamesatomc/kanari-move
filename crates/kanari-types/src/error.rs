// Copyright (c) Kanari Network
// SPDX-License-Identifier: Apache-2.0

use coerce::actor::ActorRefErr;
use move_binary_format::errors::VMError;
use moveos_types::genesis_info::GenesisInfo;
use std::io;
use thiserror::Error;

pub type KanariResult<T> = Result<T, KanariError>;

/// Custom error type for Kanari.
#[derive(Eq, PartialEq, Clone, Debug, Error)]
pub enum KanariError {
    /// config
    #[error("Unable to find config {0}, have you run `kanari init`?")]
    ConfigNotFoundError(String),
    #[error("Unable to load config: {0}, Reason: {1}.")]
    ConfigLoadError(String, String),

    /// common
    #[error("Aborted command")]
    AbortedError,
    #[error("Invalid arguments: {0}")]
    CommandArgumentError(String),

    /// move
    #[error("Move compilation failed: {0}")]
    MoveCompilationError(String),
    #[error("Move unit tests failed")]
    MoveTestError,
    #[error("Move Prover failed: {0}")]
    MoveProverError(String),
    #[error("Unable to parse '{0}': error: {1}")]
    UnableToParse(&'static str, String),
    #[error("Unable to read file '{0}', error: {1}")]
    UnableToReadFile(String, String),
    #[error("Error: {0}")]
    UnexpectedError(String),

    #[error("Simulation failed with status: {0}")]
    SimulationError(String),

    #[error("Coverage failed with status: {0}")]
    CoverageError(String),
    #[error("BCS failed with status: {0}")]
    BcsError(String),
    #[error("IO error: {0}")]
    IOError(String),
    #[error("Sign message error: {0}")]
    SignMessageError(String),
    #[error("Transaction error: {0}")]
    TransactionError(String),
    #[error("DryRun Transaction error: {0}")]
    DryRunTransactionError(String),
    #[error("View function error: {0}")]
    ViewFunctionError(String),
    #[error("Import account error: {0}")]
    ImportAccountError(String),
    #[error("Switch account error: {0}")]
    SwitchAccountError(String),
    #[error("Update account error: {0}")]
    UpdateAccountError(String),
    #[error("Nullify account error: {0}")]
    NullifyAccountError(String),
    #[error("Generate key error: {0}")]
    GenerateKeyError(String),
    #[error("Rotate authentication key error: {0}")]
    RotateAuthenticationKeyError(String),
    #[error("Remove authentication key error: {0}")]
    RemoveAuthenticationKeyError(String),
    #[error("Account not found error: {0}")]
    AccountNotFoundError(String),
    #[error("Account balance error: {0}")]
    AccountBalanceError(String),

    //#[error("base64 decode error: {0}")]
    //Base64DecodeError(String),
    #[error("Invalid length error:")]
    InvalidlengthError(),

    // Cryptography errors.
    #[error("Signature key generation error: {0}")]
    SignatureKeyGenError(String),
    #[error("Key Conversion Error: {0}")]
    KeyConversionError(String),

    #[error("Switch env error: {0}")]
    SwitchEnvError(String),
    #[error("Remove env error: {0}")]
    RemoveEnvError(String),

    // Signature verification
    #[error("Signature is not valid: {}", error)]
    InvalidSignature { error: String },
    #[error("Value was not signed by the correct sender: {}", error)]
    IncorrectSigner { error: String },
    #[error("Invalid chain ID")]
    InvalidChainID,
    #[error("Invalid password error: {0}")]
    InvalidPasswordError(String),
    #[error("Invalid signature scheme error")]
    InvalidSignatureScheme,

    #[error("Clean server error: {0}")]
    CleanServerError(String),

    #[error("Use of disabled feature: {:?}", error)]
    UnsupportedFeatureError { error: String },

    #[error("Active address does not exist error")]
    ActiveAddressDoesNotExistError,

    #[error("Sequencer key pair does not exist error: {0}")]
    SequencerKeyPairDoesNotExistError(String),

    #[error("Proposer key pair does not exist error: {0}")]
    ProposerKeyPairDoesNotExistError(String),

    #[error("Relayer key pair does not exist error: {0}")]
    RelayerKeyPairDoesNotExistError(String),

    #[error("Invalid sequencer or proposer or relayer key pair")]
    InvalidSequencerOrProposerOrRelayerKeyPair,

    #[error("The local gas_config version {0} is lower than the onchain version {1}")]
    InvalidLocalGasVersion(u64, u64),

    #[error("The content length of local gas schedule is less")]
    LessLocalGasScheduleLength,

    #[error("The content of local gas schedule must be subset of onchain gas schedule")]
    LocalIncorrectGasSchedule,

    #[error("The onchain gas schedule is empty.")]
    OnchainGasScheduleIsEmpty,

    #[error("The l1 tx has been executed.")]
    L1TxAlreadyExecuted,

    #[error("VM error: {0}")]
    VMError(VMError),

    // Add new variant for ActorRefErr
    #[error("Actor reference error: {0}")]
    ActorRefError(String),

    #[error("Failed to dispatch subscription: {0}")]
    FailedToDispatchSubscription(String),
}

impl From<anyhow::Error> for KanariError {
    fn from(e: anyhow::Error) -> Self {
        let message = e
            .chain()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        KanariError::UnexpectedError(message)
    }
}

impl From<bcs::Error> for KanariError {
    fn from(e: bcs::Error) -> Self {
        KanariError::BcsError(e.to_string())
    }
}

impl From<io::Error> for KanariError {
    fn from(e: io::Error) -> Self {
        KanariError::IOError(e.to_string())
    }
}

impl From<bitcoin::io::Error> for KanariError {
    fn from(e: bitcoin::io::Error) -> Self {
        KanariError::IOError(e.to_string())
    }
}

impl From<VMError> for KanariError {
    fn from(e: VMError) -> Self {
        KanariError::VMError(e)
    }
}

impl From<serde_json::Error> for KanariError {
    fn from(e: serde_json::Error) -> Self {
        KanariError::UnexpectedError(e.to_string())
    }
}

impl From<bitcoin::psbt::Error> for KanariError {
    fn from(e: bitcoin::psbt::Error) -> Self {
        KanariError::CommandArgumentError(e.to_string())
    }
}

impl From<hex::FromHexError> for KanariError {
    fn from(e: hex::FromHexError) -> Self {
        KanariError::CommandArgumentError(e.to_string())
    }
}

impl From<ActorRefErr> for KanariError {
    fn from(e: ActorRefErr) -> Self {
        KanariError::ActorRefError(e.to_string())
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum GenesisError {
    #[error("Genesis version mismatch: from store({from_store}), from binary({from_binary}).")]
    GenesisVersionMismatch {
        from_store: Box<GenesisInfo>,
        from_binary: Box<GenesisInfo>,
    },
    #[error("Genesis load fail {0}")]
    GenesisLoadFailure(String),
    #[error("Genesis block not exist in {0}.")]
    GenesisNotExist(String),
}
