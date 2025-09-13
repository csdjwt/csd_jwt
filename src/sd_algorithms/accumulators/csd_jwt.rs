use std::collections::HashSet;
use std::hash::Hash;
use std::thread;
use std::thread::JoinHandle;
use ark_bn254::{Bn254, Fr, G1Affine};
use ark_ff::PrimeField;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::rand::rngs::StdRng;
use digest::Digest;
use serde_json::{Map, Value};
use sha2::Sha256;
use vb_accumulator::persistence::{State, UniversalAccumulatorState};
use vb_accumulator::positive::Accumulator;
use vb_accumulator::prelude::PositiveAccumulator;
use vb_accumulator::setup::{Keypair, PublicKey, SecretKey, SetupParams};
use vb_accumulator::witness::MembershipWitness;

use crate::sd_algorithms::sd_algorithm::SdAlgorithm;

/// Identifier for the accumulator value in the VC/VP.
const ACCUMULATOR: &str = "accumulator";
/// Identifier for the Witness-Value Container in the VC/VP.
const WVC: &str = "wvc";


/// Struct for an instance of the CSD-JWT algorithm.
pub struct CsdJwtInstance;

impl SdAlgorithm for CsdJwtInstance {
    const ALGORITHM: &'static str = "CSD-JWT";
}

impl CsdJwtInstance {

    /// Gathers the necessary parameters for the algorithm to work.
    ///
    /// # Arguments
    /// * `rng` - Random Number Generator for producing random data and keying material.
    ///
    /// # Returns
    /// This function returns a struct containing setup parameters and the cryptographic accumulator keys.
    pub fn initialize_params(rng: &mut StdRng) -> (SetupParams<Bn254>, Keypair<Bn254>) {

        let params = SetupParams::<Bn254>::generate_using_rng(rng);
        let keypair = Keypair::<Bn254>::generate_using_rng(rng, &params);

        (params, keypair)
    }


    /// Utility function to serialize structs that implement CanonicalSerialize like accumulators and witnesses.
    ///
    /// # Arguments
    /// * `element` - Element to be serialized.
    ///
    /// # Returns
    /// This function returns a result wrapping the encoding of the element or a string illustrating the error, if it occurs.
    pub fn serialize<S>(element: &S) -> Result<String, String>
    where S: CanonicalSerialize {
        let mut compressed_bytes: Vec<u8> = Vec::new();
        match element.serialize_compressed(&mut compressed_bytes) {
            Ok(()) => { () }
            Err(err) => { return Err(format!("Error in serialization of element: [{err}]")) }
        };

        Ok(multibase::Base::Base64Url.encode(compressed_bytes))
    }


    /// Utility function to deserialize structs that implement CanonicalDeserialize like accumulators and witnesses.
    ///
    /// # Arguments
    ///
    /// * `encoded_element` - String containing the element to be deserialized.
    ///
    /// # Returns
    /// This function returns a result wrapping the deserialization of element or a string illustrating the error, if it occurs.
    pub fn deserialize<D>(encoded_element: &String) -> Result<D, String>
    where D: CanonicalDeserialize {
        let decoded = match multibase::Base::Base64Url.decode(encoded_element) {
            Ok(byte_array) => { byte_array }
            Err(err) => { return Err(format!("Error in decoding element: [{err}]")) }
        };
        let deserialized_element = match CanonicalDeserialize::deserialize_compressed(&*decoded) {
            Ok(element) => { element },
            Err(err) => { return Err(format!("Error in deserializing element: [{err}]")) }
        };

        Ok(deserialized_element)
    }


    /// Maps claims to scalar values by concatenating key and value and hashing them.
    ///
    /// # Arguments
    ///
    /// * `key` - Name of the element.
    /// * `value` - Value of the element.
    ///
    /// # Returns
    /// This function returns the converted scalar.
    pub fn convert_claim_to_scalar(key: &String, value: &Value) -> Fr {

        let mut hasher = Sha256::new();
        let mut hash_input = key.clone();
        hash_input.push(':');
        hash_input.push_str(&*value.to_string());

        hasher.update(hash_input);
        let result = hasher.finalize();

        Fr::from_be_bytes_mod_order(&result.as_slice())

    }


    /// High-Level function to verify the Witness-Value Container
    ///
    /// # Arguments
    /// * `wvc` - Witness-Value Container.
    /// * `accumulator` - Accumulator value.
    /// * `issuer_public_key` - Issuer's public key used to validate the signature with.
    ///
    /// # Returns
    /// This function returns a result containing a string representing an error in case of failure.
    fn verify_witness_value_container(wvc: &Map<String, Value>, accumulator: &PositiveAccumulator<Bn254>, issuer_public_key: &PublicKey<Bn254>, params: &SetupParams<Bn254>) -> Result<(), String> {

        let mut threads: Vec<JoinHandle<Result<(), String>>> = vec![];

        for (claim_key, array_value) in wvc.clone() {

            let thread_accumulator = accumulator.clone();
            let thread_pk = issuer_public_key.clone();
            let thread_params = params.clone();
            let thread = thread::spawn(move || {
                if let Value::Array(array) = array_value {
                    let witness_value = match array.get(0) {
                        None => { return Err("Salt not found in salt value container.".to_string()) }
                        Some(key) => { key }
                    };
                    let claim_value = match array.get(1) {
                        None => { return Err("Value not found in salt value container.".to_string()) }
                        Some(value) => { value }
                    };

                    let element: Fr;
                    let witness: MembershipWitness<G1Affine>;
                    match witness_value {
                        Value::String(witness_string) => {
                            witness = Self::deserialize(witness_string)?;
                            element = Self::convert_claim_to_scalar(&claim_key, claim_value);
                            thread_accumulator.verify_membership(&element, &witness, &thread_pk, &thread_params);
                        }
                        _ => { return Err("Either witnesses or values are not strings.".to_string()) }
                    }
                } else {
                    return Err("Error, array field in Witness value container is not an array".to_string())
                }
                Ok(())
            });
            threads.push(thread);
        }

        Ok(())
    }


    /// Given a raw VC containing a few fields and the credentialSubject field to include claims, create all the necessary data to create a VC using this algorithm.
    ///
    /// # Arguments
    /// * `raw_vc` - Template VC containing a credential.
    /// * `issuer_private_key` - Private key of the issuer used to generate the signature of the list of hashes.
    /// * `params` - Additional parameters needed for correct handling of the accumulator value.
    ///
    /// # Returns
    /// This function returns a VC both in the form of a Map and in the form of an unsigned JWT.
    pub fn issue_vc(raw_vc: &Map<String, Value>, issuer_private_key: &SecretKey<Fr>, params: &SetupParams<Bn254>) -> Result<(Map<String, Value>, String), String> {

        let mut vc = raw_vc.clone();

        let claims: &Map<String, Value> = Self::extract_claims(&vc)?;

        let accumulator: PositiveAccumulator<Bn254> = PositiveAccumulator::initialize(params);
        let mut state: InMemoryState<Fr> = InMemoryState::new();

        let mut elements: Vec<Fr> = vec![];

        for (field, value) in claims {
            elements.push(Self::convert_claim_to_scalar(field, value));
        }

        let accumulator = match accumulator.add_batch(elements.clone(), issuer_private_key, &mut state) {
            Ok(accumulator) => { accumulator }
            Err(err) => { return Err(format!("Error in adding batch claims: [{:?}]", err)) }
        };

        let mut witness_value_container: Map<String, Value> = Map::new();
        let witnesses = match accumulator.get_membership_witnesses_for_batch(&elements, issuer_private_key, &state) {
            Ok(witnesses) => { witnesses }
            Err(err) => { return Err(format!("Error in producing batch witnesses: [{:?}]", err)) }
        };

        let mut witness;
        for (index, (key, value)) in claims.iter().enumerate() {
            witness = Self::serialize(witnesses.get(index).unwrap())?;
            witness_value_container.insert(key.clone(), Value::Array(vec![Value::String(witness), value.clone()]));
        }

        let serialized_accumulator = Self::serialize(&accumulator)?;
        Self::serialize_and_insert(&mut vc, ACCUMULATOR.to_string(), &serialized_accumulator)?;
        Self::serialize_and_insert(&mut vc, WVC.to_string(), &witness_value_container)?;
        Self::remove_claims(&mut vc)?;

        let jwt = Self::encode_jwt(&vc)?;

        Ok((vc, jwt))
    }

    /// Given a VC, verify it using all the necessary data.
    ///
    /// # Arguments
    /// * `vc` - Verifiable Credential.
    /// * `issuer_public_key` - Issuer's public key to verify the signature of the list of hashes.
    /// * `params` - Additional parameters needed for correct handling of the accumulator value.
    ///
    /// # Returns
    /// This function returns a string containing an error in case of failure.
    pub fn verify_vc(vc: &Map<String, Value>, issuer_public_key: &PublicKey<Bn254>, params: &SetupParams<Bn254>) -> Result<(), String> {

        let witness_value_container: Map<String, Value> = Self::get_and_decode(vc, WVC.to_string())?;
        let serialized_accumulator: String = Self::get_and_decode(vc, ACCUMULATOR.to_string())?;

        let accumulator: PositiveAccumulator<Bn254> = Self::deserialize(&serialized_accumulator)?;

        Self::verify_witness_value_container(&witness_value_container, &accumulator, issuer_public_key, params)?;

        Ok(())
    }


    /// Given a VC, and a set of disclosures, create a Verifiable Presentation accordingly.
    ///
    /// # Arguments
    /// * `vc` - Verifiable Credential.
    /// * `disclosures` - List of strings containing the names of the claims that are to be disclosed.
    /// * `holder_private_key` - Holder's private key necessary for proof of possession.
    ///
    /// # Returns
    /// This function returns the VP both in form of a Map and in form of a signed JWT.
    pub fn issue_vp(vc: &Map<String, Value>, disclosures: &Vec<String>, holder_private_key: &impl AsRef<[u8]>) -> Result<(Map<String, Value>, String), String> {

        let mut vp: Map<String, Value> = vc.clone();

        let witness_value_container: Map<String, Value> = Self::get_and_decode(&mut vp, WVC.to_string())?;
        let mut new_witness_value_container: Map<String, Value> = Map::new();

        for (field, value) in witness_value_container {
            if disclosures.contains(&field) {
                new_witness_value_container.insert(field, value);
            }
        }

        Self::serialize_and_insert(&mut vp, WVC.to_string(), &new_witness_value_container)?;
        let jwt: String = Self::encode_and_sign_jwt(&mut vp, holder_private_key)?;

        Ok((vp, jwt))
    }


    /// Given a VP, verify it using all the necessary data.
    ///
    /// # Arguments
    /// * `jwt` - Verifiable Presentation encoded as a jwt.
    /// * `issuer_public_key` - Issuer's public key to verify the signature of the list of hashes.
    /// * `holder_public_key` - Holder's public key to verify the proof of possession.
    /// * `params` - Additional parameters needed for correct handling of the accumulator value.
    ///
    /// # Returns
    /// This function returns a string containing an error in case of failure.
    pub fn verify_vp(jwt: &String, issuer_public_key: &PublicKey<Bn254>, holder_public_key: &impl AsRef<[u8]>, params: &SetupParams<Bn254>) -> Result<(), String> {

        let vp = Self::decode_and_verify_jwt(jwt, holder_public_key)?;
        let witness_value_container: Map<String, Value> = Self::get_and_decode(&vp, WVC.to_string())?;
        let serialized_accumulator: String = Self::get_and_decode(&vp, ACCUMULATOR.to_string())?;
        let accumulator: PositiveAccumulator<Bn254> = Self::deserialize(&serialized_accumulator)?;
        
        Self::verify_witness_value_container(&witness_value_container, &accumulator, issuer_public_key, params)?;

        Ok(())
    }

}



#[derive(Clone, Debug)]
pub struct InMemoryState<T: Clone> {
    pub db: HashSet<T>,
}

impl<T: Clone> InMemoryState<T> {
    pub fn new() -> Self {
        let db = HashSet::<T>::new();
        Self { db }
    }
}

impl<T: Clone + Hash + Eq + Sized> State<T> for InMemoryState<T> {
    fn add(&mut self, element: T) {
        self.db.insert(element);
    }

    fn remove(&mut self, element: &T) {
        self.db.remove(element);
    }

    fn has(&self, element: &T) -> bool {
        self.db.get(element).is_some()
    }

    fn size(&self) -> u64 {
        self.db.len() as u64
    }
}

impl<'a, T: Clone + Hash + Eq + Sized + 'a> UniversalAccumulatorState<'a, T> for InMemoryState<T> {
    type ElementIterator = std::collections::hash_set::Iter<'a, T>;

    fn elements(&'a self) -> Self::ElementIterator {
        self.db.iter()
    }
}


#[cfg(test)]
mod tests {
    use ark_std::rand::SeedableRng;
    use serde_json::{Map, Value};

    use crate::common_data::{CommonData, VC};

    use super::*;

    #[test]
    fn sd_jwt() -> Result<(), String> {

        let value_raw_vc: Value = match serde_json::from_str::<Value>(VC) {
            Ok(value_vc) => { value_vc }
            Err(err) => { return Err(format!("[CSD-JWT] Failed to parse Raw Verifiable Credential from string. [{err}]")); }
        };

        let mut raw_vc: Map<String, Value> = match serde_json::from_value::<Map<String, Value>>(value_raw_vc) {
            Ok(vc) => { vc }
            Err(err) => { return Err(format!("[CSD-JWT] Failed to parse Raw Verifiable Credential from Value. [{err}]")); }
        };

        let raw_vc = &mut raw_vc;
        let mut rng = StdRng::from_entropy();
        let (holder_public_key, holder_private_key) = CommonData::holder_keys()?;
        let (params, Keypair { secret_key: ref issuer_private_key, public_key: ref issuer_public_key}) = CsdJwtInstance::initialize_params(&mut rng);

        let (vc, _vc_jwt) = match CsdJwtInstance::issue_vc(raw_vc, &issuer_private_key, &params) {
            Ok((vc, jwt)) => { (vc, jwt) }
            Err(err) => { return Err(format!("[CSD-JWT] Failed to issue vc [{err}]."))}
        };

        match CsdJwtInstance::verify_vc(&vc, &issuer_public_key, &params) {
            Ok(_) => { println!("[CSD-JWT] Successfully verified vc.")}
            Err(err) => { return Err(format!("[CSD-JWT] Failed to verify vc [{err}]."))}
        };

        let disclosures = vec!["name", "birthdate"].iter().map(|x| x.to_string()).collect();

        let (_vp, vp_jwt) = match CsdJwtInstance::issue_vp(&vc, &disclosures, &holder_private_key) {
            Ok(vp_jwt) => { vp_jwt }
            Err(err) => { return Err(format!("[CSD-JWT] Failed to issue vp: [{err}].")) }
        };

        match CsdJwtInstance::verify_vp(&vp_jwt, &issuer_public_key, &holder_public_key, &params) {
            Ok(_) => { println!("[CSD-JWT] Successfully verified vp.")}
            Err(err) => { return Err(format!("[CSD-JWT] Failed to verify vp [{err}].")) }
        };

        Ok(())
    }
}