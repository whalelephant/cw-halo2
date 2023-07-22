use halo2_verifier::proof_system;

mod error;
mod fs;

pub fn main() {
    let cache = fs::get_current_dir().join("test_data");
    let crs_path = cache.join("common-reference-string").with_extension("bin");
    let program_path = cache.join("p").with_extension("json");
    let proof_path = cache.join("p").with_extension("proof");
    let program = fs::read_program_from_file(program_path).unwrap();
    let mut crs = fs::read_crs(crs_path).unwrap();
    let proof = fs::read_hex_data_proof(proof_path).unwrap();

    // Load public inputs (if any) from `verifier_name`.
    let public_abi = program.abi.public_abi();
    let (public_inputs_map, return_value) =
        fs::read_inputs_from_file(cache, "Verifier", &public_abi).unwrap();
    let public_inputs = public_abi.encode(&public_inputs_map, return_value).unwrap();

    let verified = proof_system::verify_with_vk(
        &mut crs,
        &proof,
        public_inputs,
        &program.bytecode,
        &program.verification_key,
    )
    .unwrap();

    println!("verifier: {}", verified);
}
