//! Elliptic curve cryptography opcodes for advanced cryptographic operations

use crate::error::{AvmError, AvmResult};
use crate::types::StackValue;
use crate::vm::EvalContext;

use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_ff::{One, PrimeField, ToBytes};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

/// Elliptic curve types
#[derive(Debug, Clone)]
pub enum EllipticCurve {
    BN254g1,
    BN254g2,
    BLS12_381g1,
    BLS12_381g2,
}

impl EllipticCurve {
    /// Convert curve ID to curve type
    pub fn from_id(id: u8) -> AvmResult<Self> {
        match id {
            0 => Ok(Self::BN254g1),
            1 => Ok(Self::BN254g2),
            2 => Ok(Self::BLS12_381g1),
            3 => Ok(Self::BLS12_381g2),
            _ => Err(AvmError::invalid_program(format!(
                "Invalid elliptic curve: {id}"
            ))),
        }
    }
}

/// Add two points on an elliptic curve
pub fn op_ec_add(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let curve_id = ctx.read_bytes(1)?[0];
    ctx.advance_pc(1)?;

    let point_b = ctx.pop()?;
    let point_a = ctx.pop()?;

    let curve = EllipticCurve::from_id(curve_id)?;
    let a_bytes = point_a.as_bytes()?;
    let b_bytes = point_b.as_bytes()?;

    let result_bytes = match curve {
        EllipticCurve::BN254g1 => {
            use ark_bn254::G1Affine;
            let point_a = parse_g1_point::<G1Affine>(a_bytes)?;
            let point_b = parse_g1_point::<G1Affine>(b_bytes)?;
            let result = (point_a.into_projective() + point_b.into_projective()).into_affine();
            serialize_g1_point(&result)?
        }
        EllipticCurve::BLS12_381g1 => {
            use ark_bls12_381::G1Affine;
            let point_a = parse_g1_point::<G1Affine>(a_bytes)?;
            let point_b = parse_g1_point::<G1Affine>(b_bytes)?;
            let result = (point_a.into_projective() + point_b.into_projective()).into_affine();
            serialize_g1_point(&result)?
        }
        EllipticCurve::BN254g2 => {
            use ark_bn254::G2Affine;
            let point_a = parse_g2_point::<G2Affine>(a_bytes)?;
            let point_b = parse_g2_point::<G2Affine>(b_bytes)?;
            let result = (point_a.into_projective() + point_b.into_projective()).into_affine();
            serialize_g2_point(&result)?
        }
        EllipticCurve::BLS12_381g2 => {
            use ark_bls12_381::G2Affine;
            let point_a = parse_g2_point::<G2Affine>(a_bytes)?;
            let point_b = parse_g2_point::<G2Affine>(b_bytes)?;
            let result = (point_a.into_projective() + point_b.into_projective()).into_affine();
            serialize_g2_point(&result)?
        }
    };

    ctx.push(StackValue::Bytes(result_bytes))?;
    Ok(())
}

/// Multiply a point by a scalar on an elliptic curve
pub fn op_ec_scalar_mul(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let curve_id = ctx.read_bytes(1)?[0];
    ctx.advance_pc(1)?;

    let scalar = ctx.pop()?;
    let point = ctx.pop()?;

    let curve = EllipticCurve::from_id(curve_id)?;
    let point_bytes = point.as_bytes()?;
    let scalar_bytes = scalar.as_bytes()?;

    let result_bytes = match curve {
        EllipticCurve::BN254g1 => {
            use ark_bn254::{Fr, G1Affine};
            let point = parse_g1_point::<G1Affine>(point_bytes)?;
            let scalar = parse_scalar::<Fr>(scalar_bytes)?;
            let result = point
                .into_projective()
                .mul(scalar.into_repr())
                .into_affine();
            serialize_g1_point(&result)?
        }
        EllipticCurve::BLS12_381g1 => {
            use ark_bls12_381::{Fr, G1Affine};
            let point = parse_g1_point::<G1Affine>(point_bytes)?;
            let scalar = parse_scalar::<Fr>(scalar_bytes)?;
            let result = point
                .into_projective()
                .mul(scalar.into_repr())
                .into_affine();
            serialize_g1_point(&result)?
        }
        EllipticCurve::BN254g2 => {
            use ark_bn254::{Fr, G2Affine};
            let point = parse_g2_point::<G2Affine>(point_bytes)?;
            let scalar = parse_scalar::<Fr>(scalar_bytes)?;
            let result = point
                .into_projective()
                .mul(scalar.into_repr())
                .into_affine();
            serialize_g2_point(&result)?
        }
        EllipticCurve::BLS12_381g2 => {
            use ark_bls12_381::{Fr, G2Affine};
            let point = parse_g2_point::<G2Affine>(point_bytes)?;
            let scalar = parse_scalar::<Fr>(scalar_bytes)?;
            let result = point
                .into_projective()
                .mul(scalar.into_repr())
                .into_affine();
            serialize_g2_point(&result)?
        }
    };

    ctx.push(StackValue::Bytes(result_bytes))?;
    Ok(())
}

/// Check if pairing equation holds for given points
pub fn op_ec_pairing_check(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let curve_id = ctx.read_bytes(1)?[0];
    ctx.advance_pc(1)?;

    let points = ctx.pop()?;

    let curve = EllipticCurve::from_id(curve_id)?;
    let points_bytes = points.as_bytes()?;

    let result = match curve {
        EllipticCurve::BN254g1 | EllipticCurve::BN254g2 => {
            use ark_bn254::{Bn254, G1Affine, G2Affine};
            use ark_ec::{PairingEngine, bn::G1Prepared, bn::G2Prepared};

            let pairs = parse_pairing_points::<G1Affine, G2Affine>(points_bytes)?;
            let prepared_pairs: Vec<(
                G1Prepared<ark_bn254::Parameters>,
                G2Prepared<ark_bn254::Parameters>,
            )> = pairs
                .into_iter()
                .map(|(g1, g2)| (G1Prepared::from(g1), G2Prepared::from(g2)))
                .collect();
            let result = Bn254::product_of_pairings(prepared_pairs.iter());
            if result.is_one() { 1 } else { 0 }
        }
        EllipticCurve::BLS12_381g1 | EllipticCurve::BLS12_381g2 => {
            use ark_bls12_381::{Bls12_381, G1Affine, G2Affine};
            use ark_ec::{PairingEngine, bls12::G1Prepared, bls12::G2Prepared};

            let pairs = parse_pairing_points::<G1Affine, G2Affine>(points_bytes)?;
            let prepared_pairs: Vec<(
                G1Prepared<ark_bls12_381::Parameters>,
                G2Prepared<ark_bls12_381::Parameters>,
            )> = pairs
                .into_iter()
                .map(|(g1, g2)| (G1Prepared::from(g1), G2Prepared::from(g2)))
                .collect();
            let result = Bls12_381::product_of_pairings(prepared_pairs.iter());
            if result.is_one() { 1 } else { 0 }
        }
    };

    ctx.push(StackValue::Uint(result))?;
    Ok(())
}

/// Multi-scalar multiplication on elliptic curves
pub fn op_ec_multi_scalar_mul(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let curve_id = ctx.read_bytes(1)?[0];
    ctx.advance_pc(1)?;

    let scalars = ctx.pop()?;
    let points = ctx.pop()?;

    let curve = EllipticCurve::from_id(curve_id)?;
    let points_bytes = points.as_bytes()?;
    let scalars_bytes = scalars.as_bytes()?;

    let result_bytes = match curve {
        EllipticCurve::BN254g1 => {
            use ark_bn254::{Fr, G1Affine};
            let (points, scalars) =
                parse_multi_scalar_data::<G1Affine, Fr>(points_bytes, scalars_bytes)?;
            let result = multi_scalar_mul_g1(&points, &scalars)?;
            serialize_g1_point(&result)?
        }
        EllipticCurve::BLS12_381g1 => {
            use ark_bls12_381::{Fr, G1Affine};
            let (points, scalars) =
                parse_multi_scalar_data::<G1Affine, Fr>(points_bytes, scalars_bytes)?;
            let result = multi_scalar_mul_g1(&points, &scalars)?;
            serialize_g1_point(&result)?
        }
        EllipticCurve::BN254g2 => {
            use ark_bn254::{Fr, G2Affine};
            let (points, scalars) =
                parse_multi_scalar_data::<G2Affine, Fr>(points_bytes, scalars_bytes)?;
            let result = multi_scalar_mul_g2(&points, &scalars)?;
            serialize_g2_point(&result)?
        }
        EllipticCurve::BLS12_381g2 => {
            use ark_bls12_381::{Fr, G2Affine};
            let (points, scalars) =
                parse_multi_scalar_data::<G2Affine, Fr>(points_bytes, scalars_bytes)?;
            let result = multi_scalar_mul_g2(&points, &scalars)?;
            serialize_g2_point(&result)?
        }
    };

    ctx.push(StackValue::Bytes(result_bytes))?;
    Ok(())
}

/// Check if a point is in the correct subgroup
pub fn op_ec_subgroup_check(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let curve_id = ctx.read_bytes(1)?[0];
    ctx.advance_pc(1)?;

    let point = ctx.pop()?;

    let curve = EllipticCurve::from_id(curve_id)?;
    let point_bytes = point.as_bytes()?;

    let result = match curve {
        EllipticCurve::BN254g1 => {
            use ark_bn254::G1Affine;
            let point = parse_g1_point::<G1Affine>(point_bytes)?;
            if point.is_in_correct_subgroup_assuming_on_curve() {
                1
            } else {
                0
            }
        }
        EllipticCurve::BLS12_381g1 => {
            use ark_bls12_381::G1Affine;
            let point = parse_g1_point::<G1Affine>(point_bytes)?;
            if point.is_in_correct_subgroup_assuming_on_curve() {
                1
            } else {
                0
            }
        }
        EllipticCurve::BN254g2 => {
            use ark_bn254::G2Affine;
            let point = parse_g2_point::<G2Affine>(point_bytes)?;
            if point.is_in_correct_subgroup_assuming_on_curve() {
                1
            } else {
                0
            }
        }
        EllipticCurve::BLS12_381g2 => {
            use ark_bls12_381::G2Affine;
            let point = parse_g2_point::<G2Affine>(point_bytes)?;
            if point.is_in_correct_subgroup_assuming_on_curve() {
                1
            } else {
                0
            }
        }
    };

    ctx.push(StackValue::Uint(result))?;
    Ok(())
}

/// Map field element to curve point
pub fn op_ec_map_to(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let curve_id = ctx.read_bytes(1)?[0];
    ctx.advance_pc(1)?;

    let field_element = ctx.pop()?;

    let curve = EllipticCurve::from_id(curve_id)?;
    let fe_bytes = field_element.as_bytes()?;

    // Validate field element length (32 bytes for field elements)
    if fe_bytes.len() != 32 {
        return Err(AvmError::InvalidByteArrayLength {
            expected: 32,
            actual: fe_bytes.len(),
        });
    }

    let result_bytes = match curve {
        EllipticCurve::BN254g1 => {
            use ark_bn254::{Fq, G1Affine};
            let field_elem = bytes_to_field_element::<Fq>(fe_bytes)?;
            let point: G1Affine = simple_map_to_curve_g1(&field_elem);
            serialize_g1_point(&point)?
        }
        EllipticCurve::BLS12_381g1 => {
            use ark_bls12_381::{Fq, G1Affine};
            let field_elem = bytes_to_field_element::<Fq>(fe_bytes)?;
            let point: G1Affine = simple_map_to_curve_g1(&field_elem);
            serialize_g1_point(&point)?
        }
        EllipticCurve::BN254g2 => {
            return Err(AvmError::invalid_program(
                "Map-to-curve for G2 groups not implemented",
            ));
        }
        EllipticCurve::BLS12_381g2 => {
            return Err(AvmError::invalid_program(
                "Map-to-curve for G2 groups not implemented",
            ));
        }
    };

    ctx.push(StackValue::Bytes(result_bytes))?;
    Ok(())
}

/// Helper function to parse scalar from bytes
fn parse_scalar<F: PrimeField>(bytes: &[u8]) -> AvmResult<F> {
    if bytes.len() != 32 {
        return Err(AvmError::InvalidByteArrayLength {
            expected: 32,
            actual: bytes.len(),
        });
    }

    // from_be_bytes_mod_order in arkworks 0.3 doesn't return Option
    Ok(F::from_be_bytes_mod_order(bytes))
}

/// Helper function to parse G1 points from bytes
fn parse_g1_point<G1Point>(bytes: &[u8]) -> AvmResult<G1Point>
where
    G1Point: AffineCurve + CanonicalDeserialize,
{
    if bytes.is_empty() {
        return Ok(G1Point::zero());
    }

    if bytes.len() != 64 {
        return Err(AvmError::InvalidByteArrayLength {
            expected: 64,
            actual: bytes.len(),
        });
    }

    G1Point::deserialize_uncompressed(bytes)
        .map_err(|e| AvmError::crypto_error(format!("Invalid G1 point: {e}")))
}

/// Helper function to parse G2 points from bytes
fn parse_g2_point<G2Point>(bytes: &[u8]) -> AvmResult<G2Point>
where
    G2Point: AffineCurve + CanonicalDeserialize,
{
    if bytes.is_empty() {
        return Ok(G2Point::zero());
    }

    if bytes.len() != 128 {
        return Err(AvmError::InvalidByteArrayLength {
            expected: 128,
            actual: bytes.len(),
        });
    }

    G2Point::deserialize_uncompressed(bytes)
        .map_err(|e| AvmError::crypto_error(format!("Invalid G2 point: {e}")))
}

/// Helper function to serialize G1 points to bytes
fn serialize_g1_point<G1Point>(point: &G1Point) -> AvmResult<Vec<u8>>
where
    G1Point: AffineCurve + CanonicalSerialize,
{
    let mut bytes = Vec::new();
    point
        .serialize_uncompressed(&mut bytes)
        .map_err(|e| AvmError::crypto_error(format!("Point serialization failed: {e}")))?;
    Ok(bytes)
}

/// Helper function to serialize G2 points to bytes
fn serialize_g2_point<G2Point>(point: &G2Point) -> AvmResult<Vec<u8>>
where
    G2Point: AffineCurve + CanonicalSerialize,
{
    let mut bytes = Vec::new();
    point
        .serialize_uncompressed(&mut bytes)
        .map_err(|e| AvmError::crypto_error(format!("Point serialization failed: {e}")))?;
    Ok(bytes)
}

/// Helper function to parse pairing points from bytes
fn parse_pairing_points<G1Point, G2Point>(bytes: &[u8]) -> AvmResult<Vec<(G1Point, G2Point)>>
where
    G1Point: AffineCurve + CanonicalDeserialize,
    G2Point: AffineCurve + CanonicalDeserialize,
{
    // Each pair consists of 64 bytes (G1) + 128 bytes (G2) = 192 bytes
    const PAIR_SIZE: usize = 192;

    if bytes.len() % PAIR_SIZE != 0 {
        return Err(AvmError::invalid_program(format!(
            "Invalid pairing points length: expected multiple of {PAIR_SIZE}, got {}",
            bytes.len()
        )));
    }

    let num_pairs = bytes.len() / PAIR_SIZE;
    let mut pairs = Vec::with_capacity(num_pairs);

    for i in 0..num_pairs {
        let offset = i * PAIR_SIZE;
        let g1_bytes = &bytes[offset..offset + 64];
        let g2_bytes = &bytes[offset + 64..offset + 192];

        let g1_point = parse_g1_point(g1_bytes)?;
        let g2_point = parse_g2_point(g2_bytes)?;

        pairs.push((g1_point, g2_point));
    }

    Ok(pairs)
}

/// Parse multi-scalar multiplication data from bytes
fn parse_multi_scalar_data<P, F>(
    points_bytes: &[u8],
    scalars_bytes: &[u8],
) -> AvmResult<(Vec<P>, Vec<F>)>
where
    P: AffineCurve + CanonicalDeserialize,
    F: PrimeField,
{
    // Points: each G1 point is 64 bytes, each G2 point is 128 bytes
    let point_size = if std::mem::size_of::<P>() <= 64 {
        64
    } else {
        128
    };
    // Scalars: each scalar is 32 bytes
    const SCALAR_SIZE: usize = 32;

    if points_bytes.len() % point_size != 0 {
        return Err(AvmError::invalid_program(format!(
            "Invalid points length: expected multiple of {point_size}, got {}",
            points_bytes.len()
        )));
    }

    if scalars_bytes.len() % SCALAR_SIZE != 0 {
        return Err(AvmError::invalid_program(format!(
            "Invalid scalars length: expected multiple of {SCALAR_SIZE}, got {}",
            scalars_bytes.len()
        )));
    }

    let num_points = points_bytes.len() / point_size;
    let num_scalars = scalars_bytes.len() / SCALAR_SIZE;

    if num_points != num_scalars {
        return Err(AvmError::invalid_program(format!(
            "Mismatched point/scalar count: {num_points} points, {num_scalars} scalars"
        )));
    }

    let mut points = Vec::with_capacity(num_points);
    let mut scalars = Vec::with_capacity(num_scalars);

    // Parse points
    for i in 0..num_points {
        let offset = i * point_size;
        let point_bytes = &points_bytes[offset..offset + point_size];

        let point = if point_size == 64 {
            parse_g1_point(point_bytes)?
        } else {
            parse_g2_point(point_bytes)?
        };
        points.push(point);
    }

    // Parse scalars
    for i in 0..num_scalars {
        let offset = i * SCALAR_SIZE;
        let scalar_bytes = &scalars_bytes[offset..offset + SCALAR_SIZE];
        let scalar = parse_scalar::<F>(scalar_bytes)?;
        scalars.push(scalar);
    }

    Ok((points, scalars))
}

/// Perform multi-scalar multiplication for G1 points
fn multi_scalar_mul_g1<G1Point>(
    points: &[G1Point],
    scalars: &[G1Point::ScalarField],
) -> AvmResult<G1Point>
where
    G1Point: AffineCurve,
{
    if points.is_empty() {
        return Ok(G1Point::zero());
    }

    let mut result = points[0].into_projective().mul(scalars[0].into_repr());

    for (point, scalar) in points.iter().skip(1).zip(scalars.iter().skip(1)) {
        result += point.into_projective().mul(scalar.into_repr());
    }

    Ok(result.into_affine())
}

/// Perform multi-scalar multiplication for G2 points
fn multi_scalar_mul_g2<G2Point>(
    points: &[G2Point],
    scalars: &[G2Point::ScalarField],
) -> AvmResult<G2Point>
where
    G2Point: AffineCurve,
{
    if points.is_empty() {
        return Ok(G2Point::zero());
    }

    let mut result = points[0].into_projective().mul(scalars[0].into_repr());

    for (point, scalar) in points.iter().skip(1).zip(scalars.iter().skip(1)) {
        result += point.into_projective().mul(scalar.into_repr());
    }

    Ok(result.into_affine())
}

/// Convert bytes to field element
fn bytes_to_field_element<F: PrimeField>(bytes: &[u8]) -> AvmResult<F> {
    if bytes.len() != 32 {
        return Err(AvmError::InvalidByteArrayLength {
            expected: 32,
            actual: bytes.len(),
        });
    }

    Ok(F::from_be_bytes_mod_order(bytes))
}

/// Simple map-to-curve for G1 points (Simplified approach)
/// This is a basic implementation - production would use optimized hash-to-curve
fn simple_map_to_curve_g1<F, G1Point>(field_element: &F) -> G1Point
where
    F: PrimeField,
    G1Point: AffineCurve<BaseField = F>,
{
    // Simplified map-to-curve implementation
    // For security-critical applications, use a proper hash-to-curve implementation
    // such as the Simplified SWU method or Elligator Squared

    // For now, we use a deterministic but simple approach:
    // Hash the field element and use it to scale the generator point

    // Convert field element to a "scalar-like" value by taking modular reduction
    // This is not cryptographically optimal but works for basic functionality
    let generator = G1Point::prime_subgroup_generator();

    // Since we can't easily construct arbitrary points in arkworks 0.3,
    // we'll use a simplified approach: multiply the generator by a value derived
    // from the field element. This is not a proper map-to-curve but provides
    // deterministic point generation.

    // Convert field element to bytes and back as a scalar (simplified)
    // Use a hash-based approach for better compatibility
    use sha2::{Digest, Sha256};

    // Hash the field element to create deterministic randomness
    let mut hasher = Sha256::new();
    let repr = field_element.into_repr();

    // Convert the representation to bytes using arkworks serialization
    let mut field_bytes = Vec::new();
    repr.write(&mut field_bytes).unwrap_or_else(|_| {
        // Fallback: use a simple conversion if serialization fails
        field_bytes = vec![1u8; 32]; // Use a constant fallback
    });

    hasher.update(&field_bytes);
    let hash_bytes = hasher.finalize();

    // Create a scalar from the hash
    let scalar = G1Point::ScalarField::from_le_bytes_mod_order(&hash_bytes);

    // Return generator multiplied by the derived scalar
    generator
        .into_projective()
        .mul(scalar.into_repr())
        .into_affine()
}
