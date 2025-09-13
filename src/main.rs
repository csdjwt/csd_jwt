use std::time::{Duration, Instant};
use std::env;
use std::str::FromStr;
use serde_json::{Map, Value};
use csd_jwt::adapters::accumulators::csd_jwt_adapter::CsdJwtAdapter;

use csd_jwt::adapters::adapter::Adapter;
use csd_jwt::adapters::hashes::merkle_tree_adapter::MerkleTreeAdapter;
use csd_jwt::adapters::hashes::sd_jwt_adapter::SdJwtAdapter;
use csd_jwt::adapters::signatures::bbs_plus_adapter::BBSPlusAdapter;
use csd_jwt::benchmark::Benchmark;
use csd_jwt::common_data::{CLAIMS, VC};
use csd_jwt::csv_writer::CSVWriter;

const INITIALIZATION_DURATION: &str = "initialization_duration";
const ISSUER_KEYPAIR_LENGTH: &str = "issuer_keypair_length";
const VC_ISSUANCE_DURATION: &str = "vc_issuance_duration";
const VP_ISSUANCE_DURATION: &str = "vp_issuance_duration";
const VC_VERIFICATION_DURATION: &str = "vc_verification_duration";
const VP_VERIFICATION_DURATION: &str = "vp_verification_duration";
const VC_JWT_LENGTH: &str = "vc_jwt_length";
const VP_JWT_LENGTH: &str = "vp_jwt_length";

fn setup_raw_vc() -> Result<Map<String, Value>, String> {

    let value_raw_vc: Value = match serde_json::from_str::<Value>(VC) {
        Ok(value_vc) => { value_vc }
        Err(err) => { return Err(format!("Failed to parse Raw Verifiable Credential from string. [{err}]")); }
    };

    match serde_json::from_value::<Map<String, Value>>(value_raw_vc) {
        Ok(vc) => { Ok(vc) }
        Err(err) => { Err(format!("Failed to parse Raw Verifiable Credential from Value. [{err}]")) }
    }
}

fn initialize_sd_algorithms(claims_len: usize, iterations: i8) -> Result<(Vec<Duration>, Vec<Box<dyn Adapter>>), String> {

    let mut sd_algorithms: Vec<Box<dyn Adapter>> = vec![];
    let mut durations: Vec<Duration> = vec![];

    let (duration, algo) = Benchmark::benchmark_initialization(|| SdJwtAdapter::new(claims_len), iterations)?;
    sd_algorithms.push(algo);
    durations.push(duration);

    let (duration, algo) = Benchmark::benchmark_initialization(|| CsdJwtAdapter::new(claims_len), iterations)?;
    sd_algorithms.push(algo);
    durations.push(duration);

    let (duration, algo) = Benchmark::benchmark_initialization(|| MerkleTreeAdapter::new(claims_len), iterations)?;
    sd_algorithms.push(algo);
    durations.push(duration);

    let (duration, algo) = Benchmark::benchmark_initialization(|| BBSPlusAdapter::new(claims_len), iterations)?;
    sd_algorithms.push(algo);
    durations.push(duration);
    Ok((durations, sd_algorithms))
}

fn substitute_with_mock_claims(raw_vc: &mut Map<String, Value>, n_mock_claims: usize) -> Result<(), String> {

    let mut claims: Map<String, Value> = Map::new();
    for i in 1..=n_mock_claims {
        claims.insert(
            String::from(format!("Claim Key {}", i)),
            Value::String(String::from(format!("Claim Value {}", i)))
        );
    }
    raw_vc.insert(CLAIMS.to_string(), Value::Object(claims));       // We simply ignore if previous claims were present

    Ok(())
}

fn create_mock_disclosures(disclosures: &mut Vec<String>, n_disclosures: usize) {

    disclosures.clear();
    for i in 1..=n_disclosures {
        disclosures.push(format!("Claim Key {}", i));
    }

}

fn benchmark_multiple_mock_claims(max_mock_claims: usize, iterations: i8) -> Result<(), String> {

    let (_, algorithms) = initialize_sd_algorithms(1, iterations)?;
    let algorithm_names: Vec<String> = algorithms
        .iter()
        .map(|algo| algo.sd_algorithm())
        .collect();


    println!("Algorithms = {:?}", algorithm_names);

    let mut writer = CSVWriter::new(algorithm_names)?;
    writer.add_file(&INITIALIZATION_DURATION.to_string())?;
    writer.add_file(&ISSUER_KEYPAIR_LENGTH.to_string())?;
    writer.add_file(&VC_ISSUANCE_DURATION.to_string())?;
    writer.add_file(&VC_VERIFICATION_DURATION.to_string())?;
    writer.add_file(&VC_JWT_LENGTH.to_string())?;

    let raw_vc: &mut Map<String, Value> = &mut setup_raw_vc()?;
    let disclosures: &mut Vec<String> = &mut vec![];

    for n_mock_claims in 1..=max_mock_claims {

        let now = Instant::now();
        substitute_with_mock_claims(raw_vc, n_mock_claims)?;

        ////////////////////////////////////////////////////////////////////////////////////////////
        /////////////////////  SETUP TIME AND ISSUER KEYPAIR LENGTH  ///////////////////////////////
        ////////////////////////////////////////////////////////////////////////////////////////////
        let (durations, sd_algorithms) = initialize_sd_algorithms(n_mock_claims, iterations)?;
        let issuer_keypair_length_vector: Vec<usize> = sd_algorithms
            .iter()
            .map(|algo| algo.issuer_keypair().unwrap())
            .map(|(pk, sk)| pk.len() + sk.len())
            .collect();
        let initialization_durations: Vec<u128> = durations.iter().map(|duration| duration.as_micros()).collect();
        writer.write_record_to_file(&INITIALIZATION_DURATION.to_string(), &initialization_durations)?;
        writer.write_record_to_file(&ISSUER_KEYPAIR_LENGTH.to_string(), &issuer_keypair_length_vector)?;


        ////////////////////////////////////////////////////////////////////////////////////////////
        /////////////////////  VC ISSUANCE TIME, JWT LENGTH AND VERIFICATION TIME  /////////////////
        ////////////////////////////////////////////////////////////////////////////////////////////
        let mut vcs: Vec<Map<String, Value>> = vec![];
        let mut vc_jwts: Vec<usize> = vec![];
        let mut vc_issuance_durations: Vec<u128> = vec![];
        let mut vc_verification_durations: Vec<u128> = vec![];

        for algo in &sd_algorithms {
            let clone = raw_vc.clone();
            let (duration, (vc, vc_jwt)) = Benchmark::benchmark_function(|| algo.issue_vc(&clone), iterations)?;

            vcs.push(vc.clone());
            vc_jwts.push(vc_jwt.len());
            vc_issuance_durations.push(duration.as_micros());

            let (duration, _) = Benchmark::benchmark_function(|| algo.verify_vc(&vc), iterations)?;
            vc_verification_durations.push(duration.as_micros());
        }

        writer.write_record_to_file(&VC_ISSUANCE_DURATION.to_string(), &vc_issuance_durations)?;
        writer.write_record_to_file(&VC_JWT_LENGTH.to_string(), &vc_jwts)?;
        writer.write_record_to_file(&VC_VERIFICATION_DURATION.to_string(), &vc_verification_durations)?;


        ////////////////////////////////////////////////////////////////////////////////////////////
        /////////////////////  VP ISSUANCE TIME, JWT LENGTH AND VERIFICATION TIME  /////////////////
        ////////////////////////////////////////////////////////////////////////////////////////////
        if n_mock_claims % 10 == 0 {

            let step: usize = n_mock_claims / 10;
            let mut duration_csv_name: String = n_mock_claims.to_string();
            duration_csv_name.push('_');
            duration_csv_name.push_str(VP_ISSUANCE_DURATION);
            writer.add_file(&duration_csv_name)?;

            let mut length_csv_name: String = n_mock_claims.to_string();
            length_csv_name.push('_');
            length_csv_name.push_str(VP_JWT_LENGTH);
            writer.add_file(&length_csv_name)?;

            let mut verification_csv_name: String = n_mock_claims.to_string();
            verification_csv_name.push('_');
            verification_csv_name.push_str(VP_VERIFICATION_DURATION);
            writer.add_file(&verification_csv_name)?;

            for n_disclosures in (1..=n_mock_claims).step_by(step) {
                let mut vps: Vec<Map<String, Value>> = vec![];
                let mut vp_jwts: Vec<usize> = vec![];
                let mut vp_issuance_durations: Vec<u128> = vec![];
                let mut vp_verification_durations: Vec<u128> = vec![];
                create_mock_disclosures(disclosures, n_disclosures);

                for (index, algo) in sd_algorithms.iter().enumerate() {
                    let clone = vcs.get(index).unwrap().clone();
                    let (duration, (vp, vp_jwt)) = Benchmark::benchmark_function(|| algo.issue_vp(&clone, &disclosures), iterations)?;

                    vps.push(vp.clone());
                    vp_jwts.push(vp_jwt.len());
                    vp_issuance_durations.push(duration.as_micros());

                    let (duration, _) = Benchmark::benchmark_function(|| algo.verify_vp(&vp_jwt), iterations)?;
                    vp_verification_durations.push(duration.as_micros());
                }

                writer.write_record_to_file(&duration_csv_name, &vp_issuance_durations)?;
                writer.write_record_to_file(&length_csv_name, &vp_jwts)?;
                writer.write_record_to_file(&verification_csv_name, &vp_verification_durations)?;
            }

        }
        let elapsed = now.elapsed();
        println!("Iteration:{:>4} - Total time: {:>12?}", n_mock_claims, elapsed);
    }

    Ok(())
}


pub fn main() -> Result<(), String> {

    match env::var("CSD_JWT_ITERATIONS") {
        Ok(iterations_string) => {
            println!("The environment variable CSD_JWT_ITERATIONS is set. Its string value is: \"{}\"", iterations_string);
            let iterations = match i8::from_str(iterations_string.as_str()) {
                Ok(iterations) => iterations,
                Err(e) => {
                    println!("The environment variable CSD_JWT_ITERATIONS cannot be parsed to i8. Exiting.");
                    return Err(e.to_string())
                }
            };
            println!("The benchmark will be executed for: {}", iterations);
            benchmark_multiple_mock_claims(100, iterations)

        }
        Err(e) => {
            println!("The environment variable CSD_JWT_ITERATIONS is not set. Exiting with error: {}", e);
            Err(e.to_string())
        },
    }


}
