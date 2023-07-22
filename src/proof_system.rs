use acvm::acir::circuit::Circuit as NoirCircuit;
use acvm::acir::native_types::WitnessMap;
use acvm::FieldElement;
use noir_halo2_backend_common::errors::BackendError;
use noir_halo2_backend_common::noir_field_to_halo2_field;
use pse_halo2wrong::halo2::halo2curves::bn256::Fr;
use pse_halo2wrong::halo2::halo2curves::bn256::{Bn256, G1Affine};

use pse_halo2wrong::halo2::plonk::VerifyingKey;

use pse_halo2wrong::halo2::poly::kzg::commitment::ParamsKZG;
use pse_halo2wrong::halo2::SerdeFormat;

use noir_halo2_backend_pse::{
    circuit_translator::NoirHalo2Translator,
    halo2_plonk_api::{halo2_verify, OpcodeFlags},
};

/// Verify proof with Verification Key
pub fn verify_with_vk(
    mut common_reference_string: &[u8],
    proof: &[u8],
    public_inputs: WitnessMap,
    circuit: &NoirCircuit,
    verification_key: &[u8],
) -> Result<bool, BackendError> {
    let params =
        ParamsKZG::<Bn256>::read_custom(&mut common_reference_string, SerdeFormat::RawBytes)
            .unwrap();

    let opcode_flags = OpcodeFlags::new(&circuit.opcodes);

    let vk = VerifyingKey::<G1Affine>::from_bytes::<NoirHalo2Translator<Fr>>(
        verification_key,
        SerdeFormat::RawBytes,
        opcode_flags,
    )
    .unwrap();

    let instance: Vec<Fr> = public_inputs
        .into_iter()
        .map(|(_, el)| noir_field_to_halo2_field(el))
        .collect();

    Ok(halo2_verify(&params, &vk, proof, &instance[..]).is_ok())
}

noir_field_to_halo2_field!(Fr);
