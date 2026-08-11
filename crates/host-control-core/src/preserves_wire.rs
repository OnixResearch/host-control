//! Optional Preserves codec over the serde wire DTO.

use alloc::{string::String, vec::Vec};

use crate::inspection::{InspectionError, InspectionSnapshot};
use crate::{wire::SelectorWire, HostControlError, HostSelector};

/// Bounded codec diagnostic that does not grant transport authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PreservesWireError {
    /// Preserves syntax or structural decoding failed.
    Malformed,
    /// Decoded selector data failed semantic core validation.
    Invalid(HostControlError),
    /// Decoded inspection data failed semantic core validation.
    InvalidInspection(InspectionError),
    /// Text encoding did not produce UTF-8.
    InvalidUtf8,
}

/// Decode Preserves text and revalidate selector semantics.
///
/// # Errors
/// Returns syntax or semantic validation diagnostics.
pub fn decode_selector_text(text: &str) -> Result<HostSelector, PreservesWireError> {
    let wire: SelectorWire =
        preserves::de::from_text(text).map_err(|_error| PreservesWireError::Malformed)?;
    wire.validate().map_err(PreservesWireError::Invalid)
}

/// Encode a structurally valid selector wire value as Preserves text.
///
/// # Errors
/// Returns semantic, codec, or UTF-8 diagnostics.
pub fn encode_selector_text(wire: &SelectorWire) -> Result<String, PreservesWireError> {
    wire.validate().map_err(PreservesWireError::Invalid)?;
    let mut buffer = Vec::new();
    let mut writer = preserves::value::TextWriter::new(&mut buffer);
    preserves::ser::to_writer(&mut writer, wire).map_err(|_error| PreservesWireError::Malformed)?;
    String::from_utf8(buffer).map_err(|_error| PreservesWireError::InvalidUtf8)
}

/// Decode Preserves text and revalidate an inspection snapshot.
///
/// # Errors
///
/// Returns syntax or exact inspection validation diagnostics.
// r[impl host_control.inspection.codecs]
pub fn decode_inspection_snapshot_text(
    text: &str,
) -> Result<InspectionSnapshot, PreservesWireError> {
    let snapshot: InspectionSnapshot =
        preserves::de::from_text(text).map_err(|_error| PreservesWireError::Malformed)?;
    snapshot
        .validate()
        .map_err(PreservesWireError::InvalidInspection)
}

/// Validate and encode an inspection snapshot as Preserves text.
///
/// # Errors
///
/// Returns semantic, codec, or UTF-8 diagnostics.
pub fn encode_inspection_snapshot_text(
    snapshot: &InspectionSnapshot,
) -> Result<String, PreservesWireError> {
    snapshot
        .clone()
        .validate()
        .map_err(PreservesWireError::InvalidInspection)?;
    let mut buffer = Vec::new();
    let mut writer = preserves::value::TextWriter::new(&mut buffer);
    preserves::ser::to_writer(&mut writer, snapshot)
        .map_err(|_error| PreservesWireError::Malformed)?;
    String::from_utf8(buffer).map_err(|_error| PreservesWireError::InvalidUtf8)
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;
    use alloc::vec;

    use crate::inspection::{
        InspectionFeature, InspectionFreshness, InspectionLimits, InspectionRedaction,
        InspectionRedactionStatus, InspectionSource, InspectionSourceStatus,
    };

    use super::*;

    const REVISION: u64 = 3;
    const SOURCE_GENERATION: u64 = 2;
    const PAGE_LIMIT: usize = 1;
    const SUMMARY_BYTE_LIMIT: usize = 128;

    fn valid_wire() -> SelectorWire {
        SelectorWire {
            host_id: "host:1".to_string(),
            resource_id: "pane:1".to_string(),
            kind: "pane".to_string(),
            lifetime: "live".to_string(),
            revision: REVISION,
        }
    }

    fn inspection_snapshot() -> InspectionSnapshot {
        InspectionSnapshot {
            source: InspectionSource {
                source_id: "source:preserves".to_string(),
                generation: SOURCE_GENERATION,
                status: InspectionSourceStatus::Ready,
                features: vec![InspectionFeature::Snapshot],
            },
            freshness: InspectionFreshness {
                source_generation: SOURCE_GENERATION,
                snapshot_revision: REVISION,
            },
            policy_id: "policy:preserves".to_string(),
            redaction: InspectionRedaction {
                profile_id: "redaction:preserves".to_string(),
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
    fn preserves_round_trip_revalidates() {
        let text = encode_selector_text(&valid_wire()).expect("valid selector should encode");
        let selector = decode_selector_text(&text).expect("encoded selector should decode");
        assert_eq!(selector.revision(), REVISION);
    }

    #[test]
    fn malformed_and_semantically_invalid_preserves_fail_closed() {
        assert_eq!(
            decode_selector_text("<"),
            Err(PreservesWireError::Malformed)
        );
        let mut invalid = valid_wire();
        invalid.host_id = String::new();
        assert!(matches!(
            encode_selector_text(&invalid),
            Err(PreservesWireError::Invalid(_))
        ));
    }

    // r[verify host_control.inspection.codecs]
    #[test]
    fn inspection_preserves_round_trip_revalidates() {
        let snapshot = inspection_snapshot();
        let text = encode_inspection_snapshot_text(&snapshot)
            .expect("valid inspection snapshot should encode");
        let decoded = decode_inspection_snapshot_text(&text)
            .expect("encoded inspection snapshot should decode");
        let mut invalid = snapshot;
        invalid.freshness.snapshot_revision = 0;

        assert_eq!(decoded.source.source_id, "source:preserves");
        assert!(matches!(
            encode_inspection_snapshot_text(&invalid),
            Err(PreservesWireError::InvalidInspection(_))
        ));
    }
}
