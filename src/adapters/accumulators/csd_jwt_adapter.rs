use ark_bn254::{Bn254, Fr};
use ark_std::rand::rngs::StdRng;
use ark_std::rand::SeedableRng;
use serde_json::{Map, Value};
use vb_accumulator::setup::{Keypair, PublicKey, SecretKey, SetupParams};
use crate::common_data::CommonData;
use crate::adapters::adapter::Adapter;
use crate::sd_algorithms::accumulators::csd_jwt::CsdJwtInstance;
use crate::sd_algorithms::sd_algorithm::SdAlgorithm;

pub struct CsdJwtAdapter {
    holder_public_key: Vec<u8>,
    holder_private_key: Vec<u8>,
    issuer_public_key: PublicKey<Bn254>,
    issuer_private_key: SecretKey<Fr>,
    params: SetupParams<Bn254>,
}

impl Adapter for CsdJwtAdapter {

    fn sd_algorithm(&self) -> String {
        CsdJwtInstance::ALGORITHM.to_string()
    }

    fn new(_claims_len: usize) -> Result<Self, String> {
        let (holder_public_key, holder_private_key) = CommonData::holder_keys()?;
        let mut rng: StdRng = StdRng::from_entropy();
        let (params, Keypair { secret_key: ref issuer_private_key, public_key: ref issuer_public_key}) = CsdJwtInstance::initialize_params(&mut rng);

        Ok(CsdJwtAdapter {
            holder_public_key,
            holder_private_key,
            issuer_public_key: issuer_public_key.clone(),
            issuer_private_key: issuer_private_key.clone(),
            params
        })
    }

    fn issue_vc(&self, raw_vc: &Map<String, Value>) -> Result<(Map<String, Value>, String), String> {
        CsdJwtInstance::issue_vc(raw_vc, &self.issuer_private_key, &self.params)
    }

    fn verify_vc(&self, vc: &Map<String, Value>) -> Result<(), String> {
        CsdJwtInstance::verify_vc(vc, &self.issuer_public_key, &self.params)
    }

    fn issue_vp(&self, vc: &Map<String, Value>, disclosures: &Vec<String>) -> Result<(Map<String, Value>, String), String> {
        CsdJwtInstance::issue_vp(vc, disclosures, &self.holder_private_key)
    }

    fn verify_vp(&self, vp_jwt: &String) -> Result<(), String> {
        CsdJwtInstance::verify_vp(vp_jwt, &self.issuer_public_key, &self.holder_public_key, &self.params)
    }

    fn issuer_keypair(&self) -> Result<(String, String), String> {
        let issuer_public_key = match serde_json::to_string(&self.issuer_public_key) {
            Ok(ipk) => {ipk}
            Err(err) => { return Err(format!("Error in serializing issuer public key: [{err}]")) }
        };
        let issuer_private_key = match serde_json::to_string(&self.issuer_private_key) {
            Ok(ipk) => {ipk}
            Err(err) => { return Err(format!("Error in serializing issuer private key: [{err}]")) }
        };

        Ok((issuer_public_key, issuer_private_key))
    }
}