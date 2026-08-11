//! Optional serde wire DTOs that always re-enter core validation.
//!
// r[impl host_control.core.serialization]

use crate::inspection::{InspectionError, InspectionSnapshot};
use crate::{HostControlError, HostSelector, HostSelectorDraft};
use alloc::string::String;

/// Structurally decoded selector fields.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct SelectorWire {
    /// Host identity.
    pub host_id: String,
    /// Resource identity.
    pub resource_id: String,
    /// Resource kind label.
    pub kind: String,
    /// Lifetime label.
    pub lifetime: String,
    /// Observed topology revision.
    pub revision: u64,
}

impl SelectorWire {
    /// Revalidate decoded fields through the semantic core.
    ///
    /// # Errors
    /// Returns the core selector diagnostic for malformed decoded values.
    pub fn validate(&self) -> Result<HostSelector, HostControlError> {
        HostSelector::validate(HostSelectorDraft {
            host_id: &self.host_id,
            resource_id: &self.resource_id,
            kind: &self.kind,
            lifetime: &self.lifetime,
            revision: self.revision,
        })
    }
}

/// Revalidate a structurally decoded inspection snapshot through the pure core.
///
/// # Errors
///
/// Returns exact inspection diagnostics for malformed decoded values.
// r[impl host_control.inspection.codecs]
pub fn validate_inspection_snapshot(
    decoded: InspectionSnapshot,
) -> Result<InspectionSnapshot, InspectionError> {
    decoded.validate()
}

#[cfg(test)]
mod tests {
    use alloc::vec;
    use alloc::vec::Vec;

    use crate::inspection::{
        InspectionFeature, InspectionFreshness, InspectionLimits, InspectionRedaction,
        InspectionRedactionStatus, InspectionSource, InspectionSourceStatus,
    };

    use super::*;

    const SOURCE_GENERATION: u64 = 2;
    const SNAPSHOT_REVISION: u64 = 3;
    const PAGE_LIMIT: usize = 1;
    const SUMMARY_BYTE_LIMIT: usize = 128;

    fn inspection_snapshot() -> InspectionSnapshot {
        InspectionSnapshot {
            source: InspectionSource {
                source_id: "source:codec".into(),
                generation: SOURCE_GENERATION,
                status: InspectionSourceStatus::Ready,
                features: vec![InspectionFeature::Snapshot],
            },
            freshness: InspectionFreshness {
                source_generation: SOURCE_GENERATION,
                snapshot_revision: SNAPSHOT_REVISION,
            },
            policy_id: "policy:codec".into(),
            redaction: InspectionRedaction {
                profile_id: "redaction:codec".into(),
                status: InspectionRedactionStatus::Clean,
            },
            limits: InspectionLimits {
                max_sections: PAGE_LIMIT,
                max_items_per_section: PAGE_LIMIT,
                max_relations_per_item: PAGE_LIMIT,
                max_summary_bytes: SUMMARY_BYTE_LIMIT,
            },
            sections: Vec::new(),
        }
    }

    #[test]
    fn decoded_selector_revalidates() {
        let value = SelectorWire {
            host_id: "host:1".into(),
            resource_id: "pane:1".into(),
            kind: "pane".into(),
            lifetime: "live".into(),
            revision: 1,
        };
        assert!(value.validate().is_ok());
    }
    #[test]
    fn malformed_decoded_selector_is_rejected() {
        let value = SelectorWire {
            host_id: String::new(),
            resource_id: "pane:1".into(),
            kind: "pane".into(),
            lifetime: "live".into(),
            revision: 1,
        };
        assert!(value.validate().is_err());
    }

    // r[verify host_control.inspection.codecs]
    #[test]
    fn decoded_inspection_snapshot_reenters_core_validation() {
        let valid = inspection_snapshot();
        let mut invalid = inspection_snapshot();
        invalid.source.generation = 0;

        assert!(validate_inspection_snapshot(valid).is_ok());
        assert_eq!(
            validate_inspection_snapshot(invalid),
            Err(InspectionError::ZeroFence {
                field: "inspection_source_generation"
            })
        );
    }
}
