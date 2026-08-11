//! Explicit resource-watermark decisions over caller-supplied facts.
//!
//! The core does not observe resources or execute shedding. Consumer shells
//! retain scheduling, parking, eviction, release, and reacquisition authority.

/// Number of bytes in a BLAKE3 identity.
pub const BLAKE3_IDENTITY_BYTES: usize = 32;

/// Number of required non-claims carried by each decision.
pub const SHEDDING_NON_CLAIM_COUNT: usize = 4;

const IMPLEMENTATION_IDENTITY_DOMAIN: &[u8] = b"host-control/shedding/implementation/v1";
const POLICY_IDENTITY_DOMAIN: &[u8] = b"host-control/shedding/policy/v1";

/// Boundaries that every shedding decision preserves.
pub const SHEDDING_NON_CLAIMS: [&str; SHEDDING_NON_CLAIM_COUNT] = [
    "the decision does not prove that an item was parked or evicted",
    "the decision does not prove that resources were released",
    "the decision does not run or validate a scheduler",
    "the decision does not prove that memory or another resource decreased",
];

/// Current admission state for reacquisition of released items.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReacquisitionGate {
    /// Reacquisition is admitted by this pressure policy.
    Open,
    /// Reacquisition remains refused until the low watermark is reached.
    Closed,
}

/// Pressure-policy disposition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SheddingDisposition {
    /// Retain the current resource set.
    Keep,
    /// Pressure is above the high watermark, so the caller can select eligible idle items.
    Shed,
    /// Do not reacquire a released item before pressure reaches the low watermark.
    RefuseReacquire,
}

/// Explicit caller-supplied input to the pressure policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SheddingInput {
    /// Current bounded resident-item observation.
    pub resident_count: Option<u64>,
    /// Low watermark that permits the closed gate to reopen.
    pub low_watermark: Option<u64>,
    /// High watermark above which the policy requests shedding.
    pub high_watermark: Option<u64>,
    /// Current reacquisition gate state.
    pub reacquisition_gate: Option<ReacquisitionGate>,
}

/// Invalid or incomplete shedding input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SheddingInputError {
    /// The resident-item observation was absent.
    MissingResidentCount,
    /// The low watermark was absent.
    MissingLowWatermark,
    /// The high watermark was absent.
    MissingHighWatermark,
    /// The current reacquisition gate was absent.
    MissingReacquisitionGate,
    /// The low watermark was not strictly less than the high watermark.
    WatermarksNotIncreasing,
}

/// Validated explicit pressure policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SheddingPolicy {
    resident_count: u64,
    low_watermark: u64,
    high_watermark: u64,
    reacquisition_gate: ReacquisitionGate,
}

impl TryFrom<SheddingInput> for SheddingPolicy {
    type Error = SheddingInputError;

    fn try_from(input: SheddingInput) -> Result<Self, Self::Error> {
        let resident_count = input
            .resident_count
            .ok_or(SheddingInputError::MissingResidentCount)?;
        let low_watermark = input
            .low_watermark
            .ok_or(SheddingInputError::MissingLowWatermark)?;
        let high_watermark = input
            .high_watermark
            .ok_or(SheddingInputError::MissingHighWatermark)?;
        let reacquisition_gate = input
            .reacquisition_gate
            .ok_or(SheddingInputError::MissingReacquisitionGate)?;

        if low_watermark >= high_watermark {
            return Err(SheddingInputError::WatermarksNotIncreasing);
        }

        Ok(Self {
            resident_count,
            low_watermark,
            high_watermark,
            reacquisition_gate,
        })
    }
}

/// Deterministic pressure decision with explicit inputs, identities, and boundaries.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SheddingDecision {
    /// Supplied resident-item observation.
    pub resident_count: u64,
    /// Supplied low watermark.
    pub low_watermark: u64,
    /// Supplied high watermark.
    pub high_watermark: u64,
    /// Gate state before this decision.
    pub prior_gate: ReacquisitionGate,
    /// Gate state after this decision.
    pub next_gate: ReacquisitionGate,
    /// Deterministic policy disposition.
    pub disposition: SheddingDisposition,
    /// BLAKE3 identity of the fixed implementation profile.
    pub implementation_identity: [u8; BLAKE3_IDENTITY_BYTES],
    /// BLAKE3 identity of the explicit watermark policy.
    pub policy_identity: [u8; BLAKE3_IDENTITY_BYTES],
    /// Required authority and result boundaries.
    pub non_claims: [&'static str; SHEDDING_NON_CLAIM_COUNT],
}

/// Computes one pressure decision from validated, caller-supplied facts.
///
/// This function does not read a clock, resource counter, scheduler, process,
/// filesystem, network, or other ambient state.
///
/// # Panics
///
/// Panics only if a value bypasses [`SheddingPolicy::try_from`] or if the
/// fixed non-claim list does not match its declared length.
#[must_use]
// r[impl host_control.shedding.decision]
// r[impl host_control.shedding.policy]
pub fn decide_shedding(policy: SheddingPolicy) -> SheddingDecision {
    assert!(
        policy.low_watermark < policy.high_watermark,
        "validated watermarks must be strictly increasing"
    );

    let (disposition, next_gate) = match policy.reacquisition_gate {
        ReacquisitionGate::Closed => {
            if policy.resident_count > policy.low_watermark {
                (
                    SheddingDisposition::RefuseReacquire,
                    ReacquisitionGate::Closed,
                )
            } else {
                (SheddingDisposition::Keep, ReacquisitionGate::Open)
            }
        }
        ReacquisitionGate::Open => {
            if policy.resident_count > policy.high_watermark {
                (SheddingDisposition::Shed, ReacquisitionGate::Closed)
            } else {
                (SheddingDisposition::Keep, ReacquisitionGate::Open)
            }
        }
    };

    let decision = SheddingDecision {
        resident_count: policy.resident_count,
        low_watermark: policy.low_watermark,
        high_watermark: policy.high_watermark,
        prior_gate: policy.reacquisition_gate,
        next_gate,
        disposition,
        implementation_identity: implementation_identity(),
        policy_identity: policy_identity(policy.low_watermark, policy.high_watermark),
        non_claims: SHEDDING_NON_CLAIMS,
    };

    assert_eq!(
        decision.non_claims.len(),
        SHEDDING_NON_CLAIM_COUNT,
        "every decision must retain all required non-claims"
    );
    decision
}

/// Caller-supplied activity fact for one possible shedding candidate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CandidateActivity {
    /// The caller reports no active work or live external session.
    Idle,
    /// The caller reports active work.
    Active,
    /// The caller reports at least one live external session.
    LiveExternalSession,
}

/// Reason that one supplied candidate cannot be selected for shedding.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CandidateRejection {
    /// Active work prevents selection.
    ActiveItem,
    /// A live external session prevents selection.
    LiveExternalSession,
}

/// Admits only a caller-reported idle candidate.
///
/// The result does not select, park, evict, or mutate the candidate.
///
/// # Errors
///
/// Returns a typed rejection for active work or a live external session.
pub const fn admit_shedding_candidate(
    activity: CandidateActivity,
) -> Result<(), CandidateRejection> {
    match activity {
        CandidateActivity::Idle => Ok(()),
        CandidateActivity::Active => Err(CandidateRejection::ActiveItem),
        CandidateActivity::LiveExternalSession => Err(CandidateRejection::LiveExternalSession),
    }
}

/// Claim that a consumer can make from a shedding decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SheddingClaim {
    /// The pure core returned the recorded disposition from supplied facts.
    DecisionOnly,
    /// A concrete item was evicted.
    EvictionSucceeded,
    /// A resource was released.
    ResourceReleased,
    /// A scheduler executed the decision.
    SchedulerExecuted,
    /// Observed pressure decreased.
    PressureDecreased,
}

/// Rejected authority or effect claim.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SheddingClaimError {
    /// The claim promotes a pure decision into caller-owned authority or effects.
    AuthorityPromotion,
}

/// Accepts only the bounded claim that the pure decision occurred.
///
/// # Errors
///
/// Returns [`SheddingClaimError::AuthorityPromotion`] for effect, scheduler,
/// release, eviction, or resource-success claims.
// r[impl host_control.shedding.boundary]
pub const fn validate_shedding_claim(claim: SheddingClaim) -> Result<(), SheddingClaimError> {
    match claim {
        SheddingClaim::DecisionOnly => Ok(()),
        SheddingClaim::EvictionSucceeded
        | SheddingClaim::ResourceReleased
        | SheddingClaim::SchedulerExecuted
        | SheddingClaim::PressureDecreased => Err(SheddingClaimError::AuthorityPromotion),
    }
}

fn implementation_identity() -> [u8; BLAKE3_IDENTITY_BYTES] {
    *blake3::hash(IMPLEMENTATION_IDENTITY_DOMAIN).as_bytes()
}

fn policy_identity(low_watermark: u64, high_watermark: u64) -> [u8; BLAKE3_IDENTITY_BYTES] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(POLICY_IDENTITY_DOMAIN);
    hasher.update(&low_watermark.to_le_bytes());
    hasher.update(&high_watermark.to_le_bytes());
    *hasher.finalize().as_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    const LOW_WATERMARK: u64 = 10;
    const HIGH_WATERMARK: u64 = 20;
    const BELOW_HIGH_COUNT: u64 = 15;
    const ABOVE_HIGH_COUNT: u64 = 21;
    const ABOVE_LOW_COUNT: u64 = 11;

    fn input(resident_count: u64, gate: ReacquisitionGate) -> SheddingInput {
        SheddingInput {
            resident_count: Some(resident_count),
            low_watermark: Some(LOW_WATERMARK),
            high_watermark: Some(HIGH_WATERMARK),
            reacquisition_gate: Some(gate),
        }
    }

    fn decision(resident_count: u64, gate: ReacquisitionGate) -> SheddingDecision {
        let policy = SheddingPolicy::try_from(input(resident_count, gate))
            .expect("complete increasing watermarks must validate");
        decide_shedding(policy)
    }

    // r[verify host_control.shedding.verification]
    #[test]
    fn open_gate_keeps_at_or_below_high_and_sheds_above_high() {
        let keep = decision(BELOW_HIGH_COUNT, ReacquisitionGate::Open);
        assert_eq!(keep.disposition, SheddingDisposition::Keep);
        assert_eq!(keep.next_gate, ReacquisitionGate::Open);

        let shed = decision(ABOVE_HIGH_COUNT, ReacquisitionGate::Open);
        assert_eq!(shed.disposition, SheddingDisposition::Shed);
        assert_eq!(shed.next_gate, ReacquisitionGate::Closed);
    }

    // r[verify host_control.shedding.verification]
    #[test]
    fn closed_gate_refuses_reacquisition_until_low_is_reached() {
        let refused = decision(ABOVE_LOW_COUNT, ReacquisitionGate::Closed);
        assert_eq!(refused.disposition, SheddingDisposition::RefuseReacquire);
        assert_eq!(refused.next_gate, ReacquisitionGate::Closed);

        let reopened = decision(LOW_WATERMARK, ReacquisitionGate::Closed);
        assert_eq!(reopened.disposition, SheddingDisposition::Keep);
        assert_eq!(reopened.next_gate, ReacquisitionGate::Open);
    }

    // r[verify host_control.shedding.verification]
    #[test]
    fn missing_or_non_increasing_values_fail_closed() {
        let missing_count = SheddingInput {
            resident_count: None,
            ..input(BELOW_HIGH_COUNT, ReacquisitionGate::Open)
        };
        assert_eq!(
            SheddingPolicy::try_from(missing_count),
            Err(SheddingInputError::MissingResidentCount)
        );

        let missing_low = SheddingInput {
            low_watermark: None,
            ..input(BELOW_HIGH_COUNT, ReacquisitionGate::Open)
        };
        assert_eq!(
            SheddingPolicy::try_from(missing_low),
            Err(SheddingInputError::MissingLowWatermark)
        );

        let missing_high = SheddingInput {
            high_watermark: None,
            ..input(BELOW_HIGH_COUNT, ReacquisitionGate::Open)
        };
        assert_eq!(
            SheddingPolicy::try_from(missing_high),
            Err(SheddingInputError::MissingHighWatermark)
        );

        let missing_gate = SheddingInput {
            reacquisition_gate: None,
            ..input(BELOW_HIGH_COUNT, ReacquisitionGate::Open)
        };
        assert_eq!(
            SheddingPolicy::try_from(missing_gate),
            Err(SheddingInputError::MissingReacquisitionGate)
        );

        let reversed = SheddingInput {
            low_watermark: Some(HIGH_WATERMARK),
            high_watermark: Some(LOW_WATERMARK),
            ..input(BELOW_HIGH_COUNT, ReacquisitionGate::Open)
        };
        assert_eq!(
            SheddingPolicy::try_from(reversed),
            Err(SheddingInputError::WatermarksNotIncreasing)
        );

        let equal = SheddingInput {
            low_watermark: Some(HIGH_WATERMARK),
            high_watermark: Some(HIGH_WATERMARK),
            ..input(BELOW_HIGH_COUNT, ReacquisitionGate::Open)
        };
        assert_eq!(
            SheddingPolicy::try_from(equal),
            Err(SheddingInputError::WatermarksNotIncreasing)
        );
    }

    // r[verify host_control.shedding.boundary]
    #[test]
    fn candidate_and_claim_boundaries_reject_authority_promotion() {
        assert_eq!(admit_shedding_candidate(CandidateActivity::Idle), Ok(()));
        assert_eq!(
            admit_shedding_candidate(CandidateActivity::Active),
            Err(CandidateRejection::ActiveItem)
        );
        assert_eq!(
            admit_shedding_candidate(CandidateActivity::LiveExternalSession),
            Err(CandidateRejection::LiveExternalSession)
        );
        assert_eq!(validate_shedding_claim(SheddingClaim::DecisionOnly), Ok(()));
        assert_eq!(
            validate_shedding_claim(SheddingClaim::EvictionSucceeded),
            Err(SheddingClaimError::AuthorityPromotion)
        );
        assert_eq!(
            validate_shedding_claim(SheddingClaim::ResourceReleased),
            Err(SheddingClaimError::AuthorityPromotion)
        );
        assert_eq!(
            validate_shedding_claim(SheddingClaim::SchedulerExecuted),
            Err(SheddingClaimError::AuthorityPromotion)
        );
        assert_eq!(
            validate_shedding_claim(SheddingClaim::PressureDecreased),
            Err(SheddingClaimError::AuthorityPromotion)
        );
    }

    #[test]
    fn identities_are_stable_and_policy_scoped() {
        let first = decision(BELOW_HIGH_COUNT, ReacquisitionGate::Open);
        let second = decision(ABOVE_HIGH_COUNT, ReacquisitionGate::Closed);
        assert_eq!(
            first.implementation_identity,
            second.implementation_identity
        );
        assert_eq!(first.policy_identity, second.policy_identity);
        assert_ne!(first.implementation_identity, first.policy_identity);
        assert_eq!(first.non_claims, SHEDDING_NON_CLAIMS);
    }
}
