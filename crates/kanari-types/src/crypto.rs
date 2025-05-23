// Copyright (c) Kanari Network
// SPDX-License-Identifier: Apache-2.0

// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::{
    address::{BitcoinAddress, KanariAddress},
    authentication_key::AuthenticationKey,
    error::{KanariError, KanariResult},
    kanari_key::KANARI_SECRET_KEY_HRP,
};
use anyhow::{anyhow, bail};
use bech32::{encode, Bech32, EncodeError};
use bitcoin::secp256k1::SecretKey;
use derive_more::{AsRef, From};
pub use enum_dispatch::enum_dispatch;
use eyre::eyre;
pub use fastcrypto::traits::KeyPair as KeypairTraits;
pub use fastcrypto::traits::Signer;
pub use fastcrypto::traits::{
    AggregateAuthenticator, Authenticator, EncodeDecodeBase64, RecoverableSignature,
    RecoverableSigner, SigningKey, ToFromBytes, VerifyingKey,
};
use fastcrypto::{
    ed25519::{
        Ed25519KeyPair, Ed25519PublicKey, Ed25519PublicKeyAsBytes, Ed25519Signature,
        Ed25519SignatureAsBytes,
    },
    encoding::{Base64, Encoding},
};
use fastcrypto::{
    error::FastCryptoError,
    secp256k1::{Secp256k1KeyPair, Secp256k1PublicKeyAsBytes},
};
use fastcrypto::{
    hash::{Blake2b256, HashFunction},
    secp256k1::{Secp256k1PublicKey, Secp256k1Signature, Secp256k1SignatureAsBytes},
};
use schemars::JsonSchema;
use serde::ser::Serializer;
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::serde_as;
use std::{hash::Hash, str::FromStr};

pub type DefaultHash = Blake2b256;

#[derive(Debug, PartialEq)]
pub enum SignatureScheme {
    Ed25519,
    Secp256k1,
}

impl SignatureScheme {
    pub fn flag(&self) -> u8 {
        match self {
            SignatureScheme::Ed25519 => 0,
            SignatureScheme::Secp256k1 => 1,
        }
    }

    pub fn from_flag_byte(byte_int: u8) -> Result<SignatureScheme, KanariError> {
        match byte_int {
            0 => Ok(SignatureScheme::Ed25519),
            1 => Ok(SignatureScheme::Secp256k1),
            _ => Err(KanariError::InvalidSignatureScheme),
        }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, From, PartialEq, Eq)]
pub enum KanariKeyPair {
    ///For SessionKey
    Ed25519(Ed25519KeyPair),
    ///For Bitcoin
    Secp256k1(Secp256k1KeyPair),
}

impl KanariKeyPair {
    pub fn generate_ed25519() -> Self {
        let rng = &mut rand::thread_rng();
        let ed25519_keypair = Ed25519KeyPair::generate(rng);
        KanariKeyPair::Ed25519(ed25519_keypair)
    }

    pub fn generate_secp256k1() -> Self {
        let rng = &mut rand::thread_rng();
        let secp256k1_keypair = Secp256k1KeyPair::generate(rng);
        KanariKeyPair::Secp256k1(secp256k1_keypair)
    }

    pub fn from_ed25519_bytes(bytes: &[u8]) -> Result<Self, FastCryptoError> {
        Ok(KanariKeyPair::Ed25519(Ed25519KeyPair::from_bytes(bytes)?))
    }

    pub fn from_secp256k1_bytes(bytes: &[u8]) -> Result<Self, FastCryptoError> {
        Ok(KanariKeyPair::Secp256k1(Secp256k1KeyPair::from_bytes(
            bytes,
        )?))
    }

    pub fn sign(&self, msg: &[u8]) -> Signature {
        Signer::sign(self, msg)
    }

    pub fn sign_secure<T>(&self, value: &T) -> Signature
    where
        T: Serialize,
    {
        Signature::sign_secure(value, self)
    }

    pub fn public(&self) -> PublicKey {
        match self {
            KanariKeyPair::Ed25519(kp) => PublicKey::Ed25519(kp.public().into()),
            KanariKeyPair::Secp256k1(kp) => PublicKey::Secp256k1(kp.public().into()),
        }
    }

    pub fn bitcoin_public_key(&self) -> Result<bitcoin::PublicKey, anyhow::Error> {
        match self {
            KanariKeyPair::Secp256k1(kp) => {
                Ok(bitcoin::PublicKey::from_slice(kp.public().as_bytes())?)
            }
            _ => bail!("Only secp256k1 public key can be converted to bitcoin public key"),
        }
    }

    pub fn private(&self) -> &[u8] {
        match self {
            KanariKeyPair::Ed25519(kp) => kp.as_bytes(),
            KanariKeyPair::Secp256k1(kp) => kp.as_bytes(),
        }
    }

    /// Get the secp256k1 keypair
    pub fn secp256k1_keypair(&self) -> Option<bitcoin::key::Keypair> {
        match self.secp256k1_secret_key() {
            Some(sk) => {
                let keypair = bitcoin::key::Keypair::from_secret_key(
                    &bitcoin::secp256k1::Secp256k1::new(),
                    &sk,
                );
                Some(keypair)
            }
            None => None,
        }
    }

    /// Get the secp256k1 private key
    pub fn secp256k1_secret_key(&self) -> Option<SecretKey> {
        match self {
            KanariKeyPair::Secp256k1(kp) => {
                SecretKey::from_slice(kp.secret.as_bytes()).ok()
                //The bitcoin and fastcrypto dependent on different version secp256k1 library
                //So we cannot directly return the private key
                //Some(&kp.secret.privkey)
            }
            _ => None,
        }
    }

    /// Authentication key is the hash of the public key
    pub fn authentication_key(&self) -> AuthenticationKey {
        self.public().authentication_key()
    }

    pub fn copy(&self) -> Self {
        match self {
            KanariKeyPair::Ed25519(kp) => KanariKeyPair::Ed25519(kp.copy()),
            KanariKeyPair::Secp256k1(kp) => KanariKeyPair::Secp256k1(kp.copy()),
        }
    }

    // Export Private Key method exports a private key in bech32 format
    pub fn export_private_key(&self) -> Result<String, EncodeError> {
        let mut priv_key_bytes =
            Vec::with_capacity(self.public().flag() as usize + self.private().len());
        priv_key_bytes.push(self.public().flag());
        priv_key_bytes.extend_from_slice(self.private());
        // encode hrp and private key bytes using bech32 method
        let bech32_encoded = encode::<Bech32>(*KANARI_SECRET_KEY_HRP, &priv_key_bytes)?;
        Ok(bech32_encoded)
    }
}

impl Signer<Signature> for KanariKeyPair {
    fn sign(&self, msg: &[u8]) -> Signature {
        match self {
            KanariKeyPair::Ed25519(kp) => kp.sign(msg),
            KanariKeyPair::Secp256k1(kp) => kp.sign(msg),
        }
    }
}

impl FromStr for KanariKeyPair {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let kp = Self::decode_base64(s).map_err(|e| eyre!("{}", e.to_string()))?;
        Ok(kp)
    }
}

impl EncodeDecodeBase64 for KanariKeyPair {
    /// Encode a KanariKeyPair as `flag || privkey` in Base64. Note that the pubkey is not encoded.
    fn encode_base64(&self) -> String {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.push(self.public().flag());

        match self {
            KanariKeyPair::Ed25519(kp) => {
                bytes.extend_from_slice(kp.as_bytes());
            }
            KanariKeyPair::Secp256k1(kp) => {
                bytes.extend_from_slice(kp.as_bytes());
            }
        }
        Base64::encode(&bytes[..])
    }

    /// Decode a KanariKeyPair from `flag || privkey` in Base64. The public key is computed directly from the private key bytes.
    fn decode_base64(value: &str) -> Result<Self, FastCryptoError> {
        let bytes = Base64::decode(value)?;
        match SignatureScheme::from_flag_byte(*bytes.first().ok_or(FastCryptoError::InvalidInput)?)
        {
            // Process Kanari key pair by default
            Ok(scheme) => match scheme {
                SignatureScheme::Ed25519 => Ok(KanariKeyPair::Ed25519(Ed25519KeyPair::from_bytes(
                    bytes.get(1..).ok_or(FastCryptoError::InvalidInput)?,
                )?)),
                SignatureScheme::Secp256k1 => {
                    Ok(KanariKeyPair::Secp256k1(Secp256k1KeyPair::from_bytes(
                        bytes.get(1..).ok_or(FastCryptoError::InvalidInput)?,
                    )?))
                }
            },
            _ => Err(FastCryptoError::InvalidInput),
        }
    }
}

impl Serialize for KanariKeyPair {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = self.encode_base64();
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for KanariKeyPair {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let s = String::deserialize(deserializer)?;
        <KanariKeyPair as EncodeDecodeBase64>::decode_base64(&s)
            .map_err(|e| Error::custom(e.to_string()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, JsonSchema)]
pub enum PublicKey {
    Ed25519(Ed25519PublicKeyAsBytes),
    Secp256k1(Secp256k1PublicKeyAsBytes),
}

impl AsRef<[u8]> for PublicKey {
    fn as_ref(&self) -> &[u8] {
        match self {
            PublicKey::Ed25519(pk) => &pk.0,
            PublicKey::Secp256k1(pk) => &pk.0,
        }
    }
}

impl EncodeDecodeBase64 for PublicKey {
    fn encode_base64(&self) -> String {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend_from_slice(&[self.flag()]);
        bytes.extend_from_slice(self.as_ref());
        Base64::encode(&bytes[..])
    }

    fn decode_base64(value: &str) -> Result<Self, FastCryptoError> {
        let bytes = Base64::decode(value)?;
        match SignatureScheme::from_flag_byte(*bytes.first().ok_or(FastCryptoError::InvalidInput)?)
        {
            Ok(x) => match x {
                SignatureScheme::Ed25519 => {
                    let pk: Ed25519PublicKey = Ed25519PublicKey::from_bytes(
                        bytes.get(1..).ok_or(FastCryptoError::InvalidInput)?,
                    )?;
                    Ok(PublicKey::Ed25519((&pk).into()))
                }
                SignatureScheme::Secp256k1 => {
                    let pk: Secp256k1PublicKey = Secp256k1PublicKey::from_bytes(
                        bytes.get(1..).ok_or(FastCryptoError::InvalidInput)?,
                    )?;
                    Ok(PublicKey::Secp256k1((&pk).into()))
                }
            },
            Err(_) => Err(FastCryptoError::InvalidInput),
        }
    }
}

impl Serialize for PublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = self.to_string();
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for PublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let s = String::deserialize(deserializer)?;
        Self::from_str(s.as_str()).map_err(|e| Error::custom(e.to_string()))
    }
}

impl PublicKey {
    pub fn flag(&self) -> u8 {
        self.scheme().flag()
    }

    pub fn scheme(&self) -> SignatureScheme {
        match self {
            PublicKey::Ed25519(_) => Ed25519KanariSignature::SCHEME,
            PublicKey::Secp256k1(_) => Secp256k1KanariSignature::SCHEME,
        }
    }

    pub fn authentication_key(&self) -> AuthenticationKey {
        self.into()
    }

    pub fn kanari_address(&self) -> Result<KanariAddress, anyhow::Error> {
        let bitcoin_address = self.bitcoin_address()?;
        Ok(bitcoin_address.to_kanari_address())
    }

    pub fn bitcoin_address(&self) -> Result<BitcoinAddress, anyhow::Error> {
        match self {
            PublicKey::Secp256k1(pk) => {
                let xonly_pubkey =
                    bitcoin::XOnlyPublicKey::from(bitcoin::PublicKey::from_slice(&pk.0)?);
                let secp = bitcoin::secp256k1::Secp256k1::verification_only();
                // Kanari BitcoinAddress do not distinguish between network
                Ok(BitcoinAddress::from(bitcoin::Address::p2tr(
                    &secp,
                    xonly_pubkey,
                    None,
                    bitcoin::Network::Bitcoin,
                )))
            }
            _ => bail!("Only secp256k1 public key can be converted to bitcoin address"),
        }
    }

    pub fn xonly_public_key(&self) -> Result<bitcoin::XOnlyPublicKey, anyhow::Error> {
        match self {
            PublicKey::Secp256k1(pk) => {
                let xonly_pubkey =
                    bitcoin::XOnlyPublicKey::from(bitcoin::PublicKey::from_slice(&pk.0)?);
                Ok(xonly_pubkey)
            }
            _ => bail!("Only secp256k1 public key can be converted to xonly public key"),
        }
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.as_ref())
    }

    pub fn to_hex_literal(&self) -> String {
        format!("0x{}", self.to_hex())
    }

    pub fn from_hex(hex: &str) -> Result<Self, anyhow::Error> {
        let bytes = hex::decode(hex.strip_prefix("0x").unwrap_or(hex))?;
        Self::from_bytes(&bytes)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, anyhow::Error> {
        match SignatureScheme::from_flag_byte(
            *bytes
                .first()
                .ok_or_else(|| anyhow!("Invalid public key length"))?,
        ) {
            Ok(x) => match x {
                SignatureScheme::Ed25519 => {
                    let pk: Ed25519PublicKey = Ed25519PublicKey::from_bytes(
                        bytes
                            .get(1..)
                            .ok_or_else(|| anyhow!("Invalid public key length"))?,
                    )?;
                    Ok(PublicKey::Ed25519((&pk).into()))
                }
                SignatureScheme::Secp256k1 => {
                    let pk: Secp256k1PublicKey = Secp256k1PublicKey::from_bytes(
                        bytes
                            .get(1..)
                            .ok_or_else(|| anyhow!("Invalid public key length"))?,
                    )?;
                    Ok(PublicKey::Secp256k1((&pk).into()))
                }
            },
            Err(e) => Err(anyhow!("Invalid bytes :{}", e)),
        }
    }

    pub fn from_bitcoin_pubkey(pk: &bitcoin::PublicKey) -> Result<Self, anyhow::Error> {
        let bytes = pk.to_bytes();
        let pk = Secp256k1PublicKey::from_bytes(&bytes)?;
        Ok(PublicKey::Secp256k1((&pk).into()))
    }
}

impl std::fmt::Display for PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex_literal())
    }
}

impl FromStr for PublicKey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pk = Self::from_hex(s).map_err(|e| anyhow!("{}", e.to_string()))?;
        Ok(pk)
    }
}

pub trait KanariPublicKey: VerifyingKey {
    const SIGNATURE_SCHEME: SignatureScheme;
}

impl KanariPublicKey for Ed25519PublicKey {
    const SIGNATURE_SCHEME: SignatureScheme = SignatureScheme::Ed25519;
}

///The authentication key is the hash of the public key
/// The address and authentication key are the same for now
impl From<&PublicKey> for AuthenticationKey {
    fn from(pk: &PublicKey) -> Self {
        let mut hasher = DefaultHash::default();
        hasher.update([pk.flag()]);
        hasher.update(pk);
        let g_arr = hasher.finalize();
        AuthenticationKey::new(g_arr.digest.to_vec())
    }
}

//
// Account Signatures
//
// This struct exists due to the limitations of the `enum_dispatch` library.
//
pub trait KanariSignatureInner: Sized + ToFromBytes + PartialEq + Eq + Hash {
    type Sig: Authenticator<PubKey = Self::PubKey>;
    type PubKey: VerifyingKey<Sig = Self::Sig> + KanariPublicKey;
    type KeyPair: KeypairTraits<PubKey = Self::PubKey, Sig = Self::Sig>;

    const LENGTH: usize = Self::Sig::LENGTH + Self::PubKey::LENGTH + 1;
    const SCHEME: SignatureScheme = Self::PubKey::SIGNATURE_SCHEME;

    fn get_verification_inputs(&self) -> KanariResult<(Self::Sig, Self::PubKey)> {
        // Is this signature emitted by the expected author?
        let bytes = self.public_key_bytes();
        let pk = Self::PubKey::from_bytes(bytes)
            .map_err(|_| KanariError::KeyConversionError("Invalid public key".to_owned()))?;

        // deserialize the signature
        let signature = Self::Sig::from_bytes(self.signature_bytes()).map_err(|_| {
            KanariError::InvalidSignature {
                error: "Fail to get pubkey and sig".to_owned(),
            }
        })?;

        Ok((signature, pk))
    }

    fn new(kp: &Self::KeyPair, message: &[u8]) -> Self {
        let sig = Signer::sign(kp, message);

        let mut signature_bytes: Vec<u8> = Vec::new();
        signature_bytes
            .extend_from_slice(&[<Self::PubKey as KanariPublicKey>::SIGNATURE_SCHEME.flag()]);

        signature_bytes.extend_from_slice(sig.as_ref());
        signature_bytes.extend_from_slice(kp.public().as_ref());
        Self::from_bytes(&signature_bytes[..])
            .expect("Serialized signature did not have expected size")
    }
}

// Enums for signature auth validator signatures
#[enum_dispatch]
#[derive(Clone, JsonSchema, Debug, PartialEq, Eq, Hash)]
pub enum Signature {
    Ed25519KanariSignature,
    Secp256k1KanariSignature,
}

impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = self.as_ref();

        if serializer.is_human_readable() {
            let s = Base64::encode(bytes);
            serializer.serialize_str(&s)
        } else {
            serializer.serialize_bytes(bytes)
        }
    }
}

impl<'de> Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        let bytes = if deserializer.is_human_readable() {
            let s = String::deserialize(deserializer)?;
            Base64::decode(&s).map_err(|e| Error::custom(e.to_string()))?
        } else {
            let data: Vec<u8> = Vec::deserialize(deserializer)?;
            data
        };

        Self::from_bytes(&bytes).map_err(|e| Error::custom(e.to_string()))
    }
}

impl Signature {
    /// Sign the message with the secret key.
    /// Different SignatureScheme will have different hash function.
    /// The Secp256k1 will use sha256 to hash the message.
    pub fn sign(msg: &[u8], secret: &dyn Signer<Signature>) -> Self {
        Signer::sign(secret, msg)
    }

    /// Sign the message with bcs serialization and use Blake2b256 to hash the message.
    pub fn sign_secure<T>(value: &T, secret: &dyn Signer<Signature>) -> Self
    where
        T: Serialize,
    {
        let mut hasher = DefaultHash::default();
        hasher.update(bcs::to_bytes(&value).expect("Message serialization should not fail"));
        Signer::sign(secret, &hasher.finalize().digest)
    }

    /// Parse [enum CompressedSignature] from trait Signature `flag || sig || pk`.
    /// This is useful for the MultiSig to combine partial signature into a MultiSig public key.
    pub fn to_compressed(&self) -> Result<CompressedSignature, KanariError> {
        match self {
            Signature::Ed25519KanariSignature(sig) => Ok(CompressedSignature::Ed25519(
                (&Ed25519Signature::from_bytes(sig.signature_bytes()).map_err(|_| {
                    KanariError::InvalidSignature {
                        error: "Cannot parse sig".to_owned(),
                    }
                })?)
                    .into(),
            )),
            Signature::Secp256k1KanariSignature(sig) => Ok(CompressedSignature::Secp256k1(
                (&Secp256k1Signature::from_bytes(sig.signature_bytes()).map_err(|_| {
                    KanariError::InvalidSignature {
                        error: "Cannot parse sig".to_owned(),
                    }
                })?)
                    .into(),
            )),
        }
    }

    /// Parse [struct PublicKey] from trait KanariSignature `flag || sig || pk`.
    /// This is useful for the MultiSig to construct the bitmap in [struct MultiPublicKey].
    pub fn to_public_key(&self) -> Result<PublicKey, KanariError> {
        let bytes = self.public_key_bytes();
        // Process Kanari signature by default
        Ok(PublicKey::Ed25519(
            (&Ed25519PublicKey::from_bytes(bytes)
                .map_err(|_| KanariError::KeyConversionError("Cannot parse pk".to_owned()))?)
                .into(),
        ))
    }
}

impl AsRef<[u8]> for Signature {
    fn as_ref(&self) -> &[u8] {
        match self {
            Signature::Ed25519KanariSignature(sig) => sig.as_ref(),
            Signature::Secp256k1KanariSignature(sig) => sig.as_ref(),
        }
    }
}

impl ToFromBytes for Signature {
    fn from_bytes(bytes: &[u8]) -> Result<Self, FastCryptoError> {
        match bytes.first() {
            Some(x) => {
                if x == &Ed25519KanariSignature::SCHEME.flag() {
                    Ok(<Ed25519KanariSignature as ToFromBytes>::from_bytes(bytes)?.into())
                } else if x == &Secp256k1KanariSignature::SCHEME.flag() {
                    Ok(<Secp256k1KanariSignature as ToFromBytes>::from_bytes(bytes)?.into())
                } else {
                    Err(FastCryptoError::InvalidInput)
                }
            }
            _ => Err(FastCryptoError::InvalidInput),
        }
    }
}

/// Unlike [enum Signature], [enum CompressedSignature] does not contain public key.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
pub enum CompressedSignature {
    Ed25519(Ed25519SignatureAsBytes),
    Secp256k1(Secp256k1SignatureAsBytes),
}

impl AsRef<[u8]> for CompressedSignature {
    fn as_ref(&self) -> &[u8] {
        match self {
            CompressedSignature::Ed25519(sig) => &sig.0,
            CompressedSignature::Secp256k1(sig) => &sig.0,
        }
    }
}

#[enum_dispatch(Signature)]
pub trait KanariSignature: Sized + ToFromBytes {
    fn signature_bytes(&self) -> &[u8];
    fn public_key_bytes(&self) -> &[u8];
    fn scheme(&self) -> SignatureScheme;

    fn verify(&self, value: &[u8]) -> KanariResult<()>;
    fn verify_secure<T>(&self, value: &T) -> KanariResult<()>
    where
        T: Serialize,
    {
        let mut hasher = DefaultHash::default();
        hasher.update(bcs::to_bytes(&value).expect("Message serialization should not fail"));
        let digest = hasher.finalize().digest;
        self.verify(digest.as_ref())
    }
}

impl<S: KanariSignatureInner + Sized> KanariSignature for S {
    fn signature_bytes(&self) -> &[u8] {
        // Access array slice is safe because the array bytes is initialized as
        // flag || signature || pubkey with its defined length.
        &self.as_ref()[1..1 + S::Sig::LENGTH]
    }

    fn public_key_bytes(&self) -> &[u8] {
        // Access array slice is safe because the array bytes is initialized as
        // flag || signature || pubkey with its defined length.
        &self.as_ref()[S::Sig::LENGTH + 1..]
    }

    fn scheme(&self) -> SignatureScheme {
        S::PubKey::SIGNATURE_SCHEME
    }

    fn verify(&self, value: &[u8]) -> KanariResult<()> {
        let (sig, pk) = &self.get_verification_inputs()?;
        pk.verify(value, sig)
            .map_err(|e| KanariError::InvalidSignature {
                error: format!("Fail to verify user sig {}", e),
            })
    }
}

//
// Ed25519 Kanari Signature port
//
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, AsRef)]
#[as_ref(forward)]
pub struct Ed25519KanariSignature(
    #[schemars(with = "Base64")]
    // Replace the problematic serde_as attribute with a simpler one
    #[serde_as(as = "serde_with::base64::Base64")]
    [u8; Ed25519PublicKey::LENGTH + Ed25519Signature::LENGTH + 1],
);

// Implementation useful for simplify testing when mock signature is needed
impl Default for Ed25519KanariSignature {
    fn default() -> Self {
        Self([0; Ed25519PublicKey::LENGTH + Ed25519Signature::LENGTH + 1])
    }
}

impl ToFromBytes for Ed25519KanariSignature {
    fn from_bytes(bytes: &[u8]) -> Result<Self, FastCryptoError> {
        if bytes.len() != Self::LENGTH {
            return Err(FastCryptoError::InputLengthWrong(Self::LENGTH));
        }
        let mut sig_bytes = [0; Self::LENGTH];
        sig_bytes.copy_from_slice(bytes);
        Ok(Self(sig_bytes))
    }
}

impl Signer<Signature> for Ed25519KeyPair {
    fn sign(&self, msg: &[u8]) -> Signature {
        Ed25519KanariSignature::new(self, msg).into()
    }
}

impl KanariSignatureInner for Ed25519KanariSignature {
    type Sig = Ed25519Signature;
    type PubKey = Ed25519PublicKey;
    type KeyPair = Ed25519KeyPair;
    const LENGTH: usize = Ed25519PublicKey::LENGTH + Ed25519Signature::LENGTH + 1;
}

//
// Secp256k1 Signature port
//
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, AsRef)]
#[as_ref(forward)]
pub struct Secp256k1KanariSignature(
    #[schemars(with = "Base64")]
    // Replace the problematic serde_as attribute with a simpler one
    #[serde_as(as = "serde_with::base64::Base64")]
    [u8; Secp256k1PublicKey::LENGTH + Secp256k1Signature::LENGTH + 1],
);

impl KanariSignatureInner for Secp256k1KanariSignature {
    type Sig = Secp256k1Signature;
    type PubKey = Secp256k1PublicKey;
    type KeyPair = Secp256k1KeyPair;
    const LENGTH: usize = Secp256k1PublicKey::LENGTH + Secp256k1Signature::LENGTH + 1;
}

impl KanariPublicKey for Secp256k1PublicKey {
    const SIGNATURE_SCHEME: SignatureScheme = SignatureScheme::Secp256k1;
}

impl ToFromBytes for Secp256k1KanariSignature {
    fn from_bytes(bytes: &[u8]) -> Result<Self, FastCryptoError> {
        if bytes.len() != Self::LENGTH {
            return Err(FastCryptoError::InputLengthWrong(Self::LENGTH));
        }
        let mut sig_bytes = [0; Self::LENGTH];
        sig_bytes.copy_from_slice(bytes);
        Ok(Self(sig_bytes))
    }
}

impl Signer<Signature> for Secp256k1KeyPair {
    fn sign(&self, msg: &[u8]) -> Signature {
        Secp256k1KanariSignature::new(self, msg).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bitcoin::network;
    use ethers::utils::keccak256;
    use fastcrypto::{
        secp256k1::{Secp256k1KeyPair, Secp256k1PrivateKey},
        traits::{KeyPair, ToFromBytes},
    };

    // this test ensure the Kanari public key to address keep the same as the old version
    // we should also keep the Kanari public key to address algorithm the same as the move version
    #[test]
    fn test_kanari_public_key_to_address() {
        let private_key = Secp256k1PrivateKey::from_bytes(&[0xcd; 32]).unwrap();
        let kanari_keypair = KanariKeyPair::Secp256k1(Secp256k1KeyPair::from(private_key));
        let btc_address = kanari_keypair.public().bitcoin_address().unwrap();
        let btc_address_str = btc_address
            .format(network::Network::Bitcoin.to_num())
            .unwrap();
        //println!("{}", btc_address_str);
        assert_eq!(
            btc_address_str,
            "bc1pesylj5fdhxktcnl34t9r9ple0anatqwhanhtsh33szpehqhgtagqcj0rk5"
        );
        let kanari_address = btc_address.to_kanari_address();
        //println!("hex:{}, bech32:{}", kanari_address.to_hex(), kanari_address.to_bech32());
        assert_eq!(
            kanari_address.to_hex_literal(),
            "0x0c9fae081aec16249e3c9c94e09170eb7222767e0b2db04e9c7144d6e5a4e804"
        );
        assert_eq!(
            kanari_address.to_bech32(),
            "kanari1pj06uzq6astzf83unj2wpytsadezyan7pvkmqn5uw9zddedyaqzq4090g0"
        );
    }

    // this test is to ensure that the ECDSA recoverable algorithm works for Ethereum public key to address
    #[test]
    fn test_ethereum_public_key_to_address() {
        let private_key = Secp256k1PrivateKey::from_bytes(&[1u8; 32]).unwrap(); // use 1u8.
        let keypair: Secp256k1KeyPair = private_key.into();
        let public_key = keypair.public();
        let uncompressed = public_key.pubkey.serialize_uncompressed();
        let uncompressed_64 = uncompressed[1..65].to_vec();
        let hashed = keccak256(uncompressed_64);
        let address_bytes = hashed[12..32].to_vec();
        let address_str = format!("0x{}", hex::encode(address_bytes)); // Include "0x" prefix
        let expected_address = "0x1a642f0e3c3af545e7acbd38b07251b3990914f1";
        assert_eq!(address_str, expected_address);
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct SignData {
        value: Vec<u8>,
    }

    #[test]
    fn test_secp256k1_signature() {
        let kp = KanariKeyPair::generate_secp256k1();
        let message = b"hello world";
        let signature = kp.sign(message);
        assert!(signature.verify(message).is_ok());

        let value = SignData {
            value: message.to_vec(),
        };
        let signature = kp.sign_secure(&value);
        assert!(signature.verify_secure(&value).is_ok());
    }

    #[test]
    fn test_ed25519_signature() {
        let kp = KanariKeyPair::generate_ed25519();
        let message = b"hello world";
        let signature = kp.sign(message);
        assert!(signature.verify(message).is_ok());

        let value = SignData {
            value: message.to_vec(),
        };
        let signature = kp.sign_secure(&value);
        assert!(signature.verify_secure(&value).is_ok());
    }
}
