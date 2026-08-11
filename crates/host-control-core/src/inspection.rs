//! Read-only, transport-neutral inspection snapshots and continuations.
//!
//! The model validates supplied in-memory facts. It performs no I/O and grants
//! no product, mutation, persistence, transport, or source-truth authority.
//!
//! r[impl host_control.inspection.boundary]
//! r[impl host_control.inspection.model]
//! r[impl host_control.inspection.read_only]

use alloc::collections::BTreeSet;
use alloc::string::String;
use alloc::vec::Vec;

use crate::validation::{normalized_adapter_field, AdapterContractError, FieldName};

const SOURCE_ID_FIELD: FieldName = FieldName::new("inspection_source_id");
const SUBJECT_ID_FIELD: FieldName = FieldName::new("inspection_subject_id");
const SUBJECT_KIND_FIELD: FieldName = FieldName::new("inspection_subject_kind");
const SECTION_ID_FIELD: FieldName = FieldName::new("inspection_section_id");
const SECTION_TITLE_FIELD: FieldName = FieldName::new("inspection_section_title");
const POLICY_ID_FIELD: FieldName = FieldName::new("inspection_policy_id");
const REDACTION_PROFILE_FIELD: FieldName = FieldName::new("inspection_redaction_profile_id");
const SUMMARY_FIELD: FieldName = FieldName::new("inspection_summary");
const INTENT_FIELD: FieldName = FieldName::new("inspection_intent");
const COUNT_KIND_FIELD: FieldName = FieldName::new("inspection_count_kind");
const RELATION_KIND_FIELD: FieldName = FieldName::new("inspection_relation_kind");
const PROJECTION_SOURCE_KIND_FIELD: FieldName = FieldName::new("projection_source_kind");
const PROJECTION_SUBJECT_KIND_FIELD: FieldName = FieldName::new("projection_subject_kind");
const PROJECTION_SUMMARY_KIND_FIELD: FieldName = FieldName::new("projection_summary_kind");

const INSPECTION_INTENT_OBSERVE: &str = "observe";

/// Maximum admitted sections in one snapshot page.
pub const MAX_INSPECTION_SECTIONS: usize = 64;
/// Maximum admitted items in one section page.
pub const MAX_INSPECTION_ITEMS_PER_SECTION: usize = 1_024;
/// Maximum admitted relations on one item.
pub const MAX_INSPECTION_RELATIONS_PER_ITEM: usize = 64;
/// Maximum admitted typed counts on one item.
pub const MAX_INSPECTION_COUNTS_PER_ITEM: usize = 32;
/// Maximum admitted feature facts on one source.
pub const MAX_INSPECTION_FEATURES: usize = 32;
/// Maximum admitted UTF-8 bytes in one safe summary.
pub const MAX_INSPECTION_SUMMARY_BYTES: usize = 4_096;

const _: () = assert!(MAX_INSPECTION_SECTIONS > 0);
const _: () = assert!(MAX_INSPECTION_ITEMS_PER_SECTION > 0);
const _: () = assert!(MAX_INSPECTION_RELATIONS_PER_ITEM > 0);
const _: () = assert!(MAX_INSPECTION_COUNTS_PER_ITEM > 0);
const _: () = assert!(MAX_INSPECTION_FEATURES > 0);
const _: () = assert!(MAX_INSPECTION_SUMMARY_BYTES > 0);

/// Validation failure for an inspection envelope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InspectionError {
    /// Shared text-field validation failed.
    Contract(AdapterContractError),
    /// A generation or revision fence was zero.
    ZeroFence {
        /// Fence field.
        field: &'static str,
    },
    /// A declared limit was zero.
    ZeroLimit {
        /// Limit field.
        field: &'static str,
    },
    /// A declared or observed collection exceeded its bounded maximum.
    LimitExceeded {
        /// Bounded field.
        field: &'static str,
        /// Maximum accepted value.
        maximum: usize,
        /// Supplied value.
        actual: usize,
    },
    /// A source did not declare snapshot support.
    SnapshotFeatureMissing,
    /// A typed feature appeared more than once.
    DuplicateFeature,
    /// A section identity appeared more than once.
    DuplicateSection,
    /// A subject identity appeared more than once.
    DuplicateSubject,
    /// A count kind appeared more than once on one item.
    DuplicateCount,
    /// A relation appeared more than once on one item.
    DuplicateRelation,
    /// An item or relation named a source other than the snapshot source.
    UnknownSource,
    /// A relation target did not exist in the snapshot page.
    DanglingRelation,
    /// A relation was a forbidden self-cycle.
    ForbiddenRelationCycle,
    /// An unavailable source or section contained visible data.
    UnavailableSourceContainsData,
    /// Source generation and snapshot freshness facts disagreed.
    FreshnessMismatch,
    /// A continuation did not match the current snapshot facts.
    StaleContinuation,
    /// A continuation named a different section.
    ContinuationSectionMismatch,
    /// A continuation offset did not follow the current page.
    ContinuationOffsetMismatch,
    /// A continuation claimed no remaining items.
    EmptyContinuation,
    /// An item declared a payload class outside safe summaries.
    UnsafePayloadClass,
    /// A purported inspection item requested mutation or execution.
    MutationIntent,
    /// A frozen consumer projection lost product meaning.
    LossyConsumerProjection,
    /// A consumer projection promoted Host Control into product authority.
    AuthorityPromotingProjection,
}

impl From<AdapterContractError> for InspectionError {
    fn from(error: AdapterContractError) -> Self {
        Self::Contract(error)
    }
}

/// Source availability for one bounded snapshot.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum InspectionSourceStatus {
    /// The source supplied a complete bounded page.
    Ready,
    /// The source supplied an explicitly partial bounded page.
    Partial,
    /// The source supplied no visible data.
    Unavailable,
}

/// Negotiated source feature fact.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum InspectionFeature {
    /// Snapshot-first reads are supported.
    Snapshot,
    /// Revision-fenced continuation is supported.
    Continuation,
    /// Typed relation references are supported.
    Relations,
    /// Typed bounded counts are supported.
    Counts,
    /// Explicit freshness facts are supported.
    Freshness,
}

/// Product-neutral inspection source.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct InspectionSource {
    /// Stable source identity.
    pub source_id: String,
    /// Source-owned generation fence.
    pub generation: u64,
    /// Availability of the supplied source page.
    pub status: InspectionSourceStatus,
    /// Explicit negotiated features.
    pub features: Vec<InspectionFeature>,
}

/// Subject reference scoped to an inspection source.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct InspectionSubject {
    /// Source identity that owns the subject.
    pub source_id: String,
    /// Product-owned stable subject identity.
    pub subject_id: String,
    /// Product-owned subject kind label.
    pub subject_kind: String,
}

/// Exact page limits selected by the caller.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct InspectionLimits {
    /// Maximum sections in the page.
    pub max_sections: usize,
    /// Maximum items in each section page.
    pub max_items_per_section: usize,
    /// Maximum relations on each item.
    pub max_relations_per_item: usize,
    /// Maximum UTF-8 bytes in each item summary.
    pub max_summary_bytes: usize,
}

/// Source generation and snapshot revision facts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct InspectionFreshness {
    /// Source generation used for the snapshot.
    pub source_generation: u64,
    /// Source-owned snapshot revision.
    pub snapshot_revision: u64,
}

/// Redaction result for one snapshot page.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum InspectionRedactionStatus {
    /// The source reports that no selected field needed redaction.
    Clean,
    /// The source reports that selected fields were redacted.
    Redacted,
}

/// Redaction profile and observed result.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct InspectionRedaction {
    /// Caller-selected redaction profile identity.
    pub profile_id: String,
    /// Redaction result for this page.
    pub status: InspectionRedactionStatus,
}

/// Section or item availability inside a source page.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum InspectionSectionStatus {
    /// Complete under the selected limits.
    Ready,
    /// Partial with an explicit continuation.
    Partial,
    /// No items were available.
    Unavailable,
}

/// Product-neutral item status.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum InspectionItemStatus {
    /// Source supplied no stronger status.
    Unknown,
    /// Item is idle.
    Idle,
    /// Item is active.
    Running,
    /// Item waits for input or a dependency.
    Waiting,
    /// Item is explicitly blocked.
    Blocked,
    /// Item completed successfully.
    Succeeded,
    /// Item failed.
    Failed,
}

/// Payload safety class supplied at the adapter boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum InspectionPayloadClass {
    /// Bounded redaction-safe product summary.
    SafeSummary,
    /// Raw transcript or log body.
    RawTranscript,
    /// Executable command or argument vector.
    Command,
    /// Credential or bearer token material.
    Credential,
    /// Environment names or values.
    Environment,
    /// Physical host path.
    PhysicalPath,
    /// Opaque unvalidated payload dump.
    Opaque,
}

/// Typed bounded count attached to an item.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct InspectionCount {
    /// Product-owned count kind.
    pub kind: String,
    /// Supplied non-negative count.
    pub value: u64,
}

/// Relation direction from the current item.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum InspectionRelationDirection {
    /// Current item points to the target.
    Outbound,
    /// Target points to the current item.
    Inbound,
}

/// Scope in which a relation target must resolve.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum InspectionRelationScope {
    /// Target must exist in this exact snapshot page.
    Snapshot,
}

/// Typed relation to another subject.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct InspectionRelation {
    /// Product-owned relation kind.
    pub kind: String,
    /// Direction from the current item.
    pub direction: InspectionRelationDirection,
    /// Resolution scope.
    pub scope: InspectionRelationScope,
    /// Typed target reference.
    pub target: InspectionSubject,
    /// Whether a self-cycle is explicitly permitted.
    pub allows_self_cycle: bool,
}

/// Read-only inspection item.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct InspectionItem {
    /// Product subject represented by this item.
    pub subject: InspectionSubject,
    /// Product-neutral display status.
    pub status: InspectionItemStatus,
    /// Bounded product-projected safe summary.
    pub summary: String,
    /// Declared safety class of the summary.
    pub payload_class: InspectionPayloadClass,
    /// Typed bounded counts.
    pub counts: Vec<InspectionCount>,
    /// Typed bounded relations.
    pub relations: Vec<InspectionRelation>,
}

/// Exact revision-fenced cursor for one section.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct InspectionCursor {
    /// Source identity.
    pub source_id: String,
    /// Exact source generation.
    pub source_generation: u64,
    /// Exact snapshot revision.
    pub snapshot_revision: u64,
    /// Exact policy identity.
    pub policy_id: String,
    /// Exact redaction profile identity.
    pub redaction_profile_id: String,
    /// Exact selected page limits.
    pub limits: InspectionLimits,
    /// Section whose next page is requested.
    pub section_id: String,
    /// Zero-based offset of the next item.
    pub next_offset: usize,
}

/// Continuation facts for a bounded section page.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct InspectionContinuation {
    /// Exact next-page cursor.
    pub cursor: InspectionCursor,
    /// Source-declared items that remain after this page.
    pub remaining_items: usize,
}

/// Bounded section page.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct InspectionSection {
    /// Stable section identity within the source.
    pub section_id: String,
    /// Redaction-safe display title.
    pub title: String,
    /// Section availability.
    pub status: InspectionSectionStatus,
    /// Zero-based offset of the first supplied item.
    pub page_start: usize,
    /// Bounded item page.
    pub items: Vec<InspectionItem>,
    /// Exact next-page continuation when more items remain.
    pub continuation: Option<InspectionContinuation>,
}

/// Complete bounded inspection snapshot page.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct InspectionSnapshot {
    /// Source facts.
    pub source: InspectionSource,
    /// Exact freshness fences.
    pub freshness: InspectionFreshness,
    /// Caller-owned policy identity.
    pub policy_id: String,
    /// Redaction facts.
    pub redaction: InspectionRedaction,
    /// Exact caller-selected limits.
    pub limits: InspectionLimits,
    /// Bounded section pages.
    pub sections: Vec<InspectionSection>,
}

impl InspectionSnapshot {
    /// Validate all snapshot, item, relation, and continuation invariants.
    ///
    /// # Errors
    ///
    /// Returns [`InspectionError`] for malformed identities, bounds, source
    /// status, relations, unsafe payload classes, mutation intents, or stale
    /// continuations.
    // r[impl host_control.inspection.validation]
    // r[impl host_control.inspection.relations]
    pub fn validate(self) -> Result<Self, InspectionError> {
        validate_snapshot(&self)?;
        Ok(self)
    }
}

fn validate_snapshot(snapshot: &InspectionSnapshot) -> Result<(), InspectionError> {
    validate_source(&snapshot.source)?;
    validate_freshness(snapshot)?;
    validate_limits(snapshot.limits)?;
    normalized_adapter_field(POLICY_ID_FIELD, &snapshot.policy_id)?;
    normalized_adapter_field(REDACTION_PROFILE_FIELD, &snapshot.redaction.profile_id)?;
    validate_maximum(
        "inspection_sections",
        snapshot.sections.len(),
        snapshot.limits.max_sections,
    )?;
    if snapshot.source.status == InspectionSourceStatus::Unavailable
        && !snapshot.sections.is_empty()
    {
        return Err(InspectionError::UnavailableSourceContainsData);
    }

    let mut section_ids = BTreeSet::new();
    let mut subject_ids = BTreeSet::new();
    for section in &snapshot.sections {
        validate_section_shape(snapshot, section)?;
        if !section_ids.insert(section.section_id.as_str()) {
            return Err(InspectionError::DuplicateSection);
        }
        for item in &section.items {
            validate_item_shape(snapshot, item)?;
            if !subject_ids.insert(item.subject.subject_id.as_str()) {
                return Err(InspectionError::DuplicateSubject);
            }
        }
    }
    validate_relations(snapshot, &subject_ids)
}

fn validate_source(source: &InspectionSource) -> Result<(), InspectionError> {
    normalized_adapter_field(SOURCE_ID_FIELD, &source.source_id)?;
    validate_nonzero("inspection_source_generation", source.generation)?;
    validate_maximum(
        "inspection_features",
        source.features.len(),
        MAX_INSPECTION_FEATURES,
    )?;
    let mut features = BTreeSet::new();
    for feature in &source.features {
        if !features.insert(*feature) {
            return Err(InspectionError::DuplicateFeature);
        }
    }
    if !features.contains(&InspectionFeature::Snapshot) {
        return Err(InspectionError::SnapshotFeatureMissing);
    }
    Ok(())
}

fn validate_freshness(snapshot: &InspectionSnapshot) -> Result<(), InspectionError> {
    validate_nonzero(
        "inspection_freshness_generation",
        snapshot.freshness.source_generation,
    )?;
    validate_nonzero(
        "inspection_snapshot_revision",
        snapshot.freshness.snapshot_revision,
    )?;
    if snapshot.source.generation != snapshot.freshness.source_generation {
        return Err(InspectionError::FreshnessMismatch);
    }
    Ok(())
}

fn validate_limits(limits: InspectionLimits) -> Result<(), InspectionError> {
    validate_limit(
        "inspection_max_sections",
        limits.max_sections,
        MAX_INSPECTION_SECTIONS,
    )?;
    validate_limit(
        "inspection_max_items_per_section",
        limits.max_items_per_section,
        MAX_INSPECTION_ITEMS_PER_SECTION,
    )?;
    validate_limit(
        "inspection_max_relations_per_item",
        limits.max_relations_per_item,
        MAX_INSPECTION_RELATIONS_PER_ITEM,
    )?;
    validate_limit(
        "inspection_max_summary_bytes",
        limits.max_summary_bytes,
        MAX_INSPECTION_SUMMARY_BYTES,
    )
}

fn validate_section_shape(
    snapshot: &InspectionSnapshot,
    section: &InspectionSection,
) -> Result<(), InspectionError> {
    normalized_adapter_field(SECTION_ID_FIELD, &section.section_id)?;
    normalized_adapter_field(SECTION_TITLE_FIELD, &section.title)?;
    validate_maximum(
        "inspection_section_items",
        section.items.len(),
        snapshot.limits.max_items_per_section,
    )?;
    if section.status == InspectionSectionStatus::Unavailable && !section.items.is_empty() {
        return Err(InspectionError::UnavailableSourceContainsData);
    }
    match (&section.status, &section.continuation) {
        (InspectionSectionStatus::Partial, Some(continuation)) => {
            validate_continuation(snapshot, section, continuation)
        }
        (InspectionSectionStatus::Partial, None) => Err(InspectionError::EmptyContinuation),
        (_, Some(_continuation)) => Err(InspectionError::StaleContinuation),
        (_, None) => Ok(()),
    }
}

fn validate_item_shape(
    snapshot: &InspectionSnapshot,
    item: &InspectionItem,
) -> Result<(), InspectionError> {
    validate_subject(&item.subject)?;
    if item.subject.source_id != snapshot.source.source_id {
        return Err(InspectionError::UnknownSource);
    }
    normalized_adapter_field(SUMMARY_FIELD, &item.summary)?;
    validate_maximum(
        "inspection_summary_bytes",
        item.summary.len(),
        snapshot.limits.max_summary_bytes,
    )?;
    if item.payload_class != InspectionPayloadClass::SafeSummary {
        return Err(InspectionError::UnsafePayloadClass);
    }
    validate_maximum(
        "inspection_item_counts",
        item.counts.len(),
        MAX_INSPECTION_COUNTS_PER_ITEM,
    )?;
    validate_maximum(
        "inspection_item_relations",
        item.relations.len(),
        snapshot.limits.max_relations_per_item,
    )?;
    let mut count_kinds = BTreeSet::new();
    for count in &item.counts {
        let normalized = normalized_adapter_field(COUNT_KIND_FIELD, &count.kind)?;
        if !count_kinds.insert(normalized) {
            return Err(InspectionError::DuplicateCount);
        }
    }
    Ok(())
}

/// Validate that an adapter request is observation-only before projection.
///
/// The validated core item has no intent or mutation field.
///
/// # Errors
///
/// Returns [`InspectionError::MutationIntent`] for execute, approve, retry,
/// cancel, edit, delete, command, or any other non-observation intent.
// r[impl host_control.inspection.read_only]
pub fn validate_inspection_intent(intent: &str) -> Result<(), InspectionError> {
    let normalized = normalized_adapter_field(INTENT_FIELD, intent)?;
    if normalized == INSPECTION_INTENT_OBSERVE {
        Ok(())
    } else {
        Err(InspectionError::MutationIntent)
    }
}

fn validate_subject(subject: &InspectionSubject) -> Result<(), InspectionError> {
    normalized_adapter_field(SOURCE_ID_FIELD, &subject.source_id)?;
    normalized_adapter_field(SUBJECT_ID_FIELD, &subject.subject_id)?;
    normalized_adapter_field(SUBJECT_KIND_FIELD, &subject.subject_kind)?;
    Ok(())
}

fn validate_relations(
    snapshot: &InspectionSnapshot,
    subject_ids: &BTreeSet<&str>,
) -> Result<(), InspectionError> {
    for section in &snapshot.sections {
        for item in &section.items {
            let mut relation_keys = BTreeSet::new();
            for relation in &item.relations {
                normalized_adapter_field(RELATION_KIND_FIELD, &relation.kind)?;
                validate_subject(&relation.target)?;
                if relation.target.source_id != snapshot.source.source_id {
                    return Err(InspectionError::UnknownSource);
                }
                if !subject_ids.contains(relation.target.subject_id.as_str()) {
                    return Err(InspectionError::DanglingRelation);
                }
                if relation.target.subject_id == item.subject.subject_id
                    && !relation.allows_self_cycle
                {
                    return Err(InspectionError::ForbiddenRelationCycle);
                }
                let key = (
                    relation.kind.as_str(),
                    relation.direction,
                    relation.scope,
                    relation.target.source_id.as_str(),
                    relation.target.subject_id.as_str(),
                );
                if !relation_keys.insert(key) {
                    return Err(InspectionError::DuplicateRelation);
                }
            }
        }
    }
    Ok(())
}

fn validate_continuation(
    snapshot: &InspectionSnapshot,
    section: &InspectionSection,
    continuation: &InspectionContinuation,
) -> Result<(), InspectionError> {
    let cursor = &continuation.cursor;
    if continuation.remaining_items == 0 {
        return Err(InspectionError::EmptyContinuation);
    }
    if cursor.section_id != section.section_id {
        return Err(InspectionError::ContinuationSectionMismatch);
    }
    if cursor.source_id != snapshot.source.source_id
        || cursor.source_generation != snapshot.source.generation
        || cursor.snapshot_revision != snapshot.freshness.snapshot_revision
        || cursor.policy_id != snapshot.policy_id
        || cursor.redaction_profile_id != snapshot.redaction.profile_id
        || cursor.limits != snapshot.limits
    {
        return Err(InspectionError::StaleContinuation);
    }
    let expected_offset = section
        .page_start
        .checked_add(section.items.len())
        .ok_or(InspectionError::ContinuationOffsetMismatch)?;
    if cursor.next_offset != expected_offset {
        return Err(InspectionError::ContinuationOffsetMismatch);
    }
    Ok(())
}

fn validate_nonzero(field: &'static str, value: u64) -> Result<(), InspectionError> {
    if value == 0 {
        Err(InspectionError::ZeroFence { field })
    } else {
        Ok(())
    }
}

fn validate_limit(
    field: &'static str,
    value: usize,
    maximum: usize,
) -> Result<(), InspectionError> {
    if value == 0 {
        return Err(InspectionError::ZeroLimit { field });
    }
    validate_maximum(field, value, maximum)
}

fn validate_maximum(
    field: &'static str,
    value: usize,
    maximum: usize,
) -> Result<(), InspectionError> {
    assert!(maximum > 0, "inspection maximum must be positive");
    if value > maximum {
        Err(InspectionError::LimitExceeded {
            field,
            maximum,
            actual: value,
        })
    } else {
        Ok(())
    }
}

/// Product profile represented by a frozen projection fixture.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InspectionConsumer {
    /// Animus agent/session projection.
    Animus,
    /// Lattice workflow/node projection.
    Lattice,
    /// Molten actor/vat projection.
    Molten,
    /// Tile workspace/pane projection.
    Tile,
}

/// Supplied product projection facts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsumerProjection {
    /// Product profile.
    pub consumer: InspectionConsumer,
    /// Product-owned source kind.
    pub source_kind: String,
    /// Product-owned subject kind.
    pub subject_kind: String,
    /// Product-owned summary kind.
    pub summary_kind: String,
    /// Product identity remains present after projection.
    pub preserves_product_identity: bool,
    /// Product retains source-truth and mutation authority.
    pub product_retains_authority: bool,
}

/// Validate a frozen consumer projection without importing a runtime crate.
///
/// # Errors
///
/// Returns a loss or authority diagnostic if the projection does not match the
/// reviewed profile or transfers authority to Host Control.
// r[impl host_control.inspection.consumers]
pub fn validate_consumer_projection(
    projection: &ConsumerProjection,
) -> Result<(), InspectionError> {
    let source_kind =
        normalized_adapter_field(PROJECTION_SOURCE_KIND_FIELD, &projection.source_kind)?;
    let subject_kind =
        normalized_adapter_field(PROJECTION_SUBJECT_KIND_FIELD, &projection.subject_kind)?;
    let summary_kind =
        normalized_adapter_field(PROJECTION_SUMMARY_KIND_FIELD, &projection.summary_kind)?;
    let expected = match projection.consumer {
        InspectionConsumer::Animus => ("agent_runtime", "agent", "agent_status"),
        InspectionConsumer::Lattice => ("workflow_runtime", "workflow_node", "node_status"),
        InspectionConsumer::Molten => ("dataspace_runtime", "actor", "actor_status"),
        InspectionConsumer::Tile => ("workspace_runtime", "pane", "pane_status"),
    };
    if (
        source_kind.as_str(),
        subject_kind.as_str(),
        summary_kind.as_str(),
    ) != expected
        || !projection.preserves_product_identity
    {
        return Err(InspectionError::LossyConsumerProjection);
    }
    if !projection.product_retains_authority {
        return Err(InspectionError::AuthorityPromotingProjection);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;
    use alloc::vec;

    const SOURCE_ID: &str = "source:agents";
    const POLICY_ID: &str = "policy:reviewed";
    const REDACTION_ID: &str = "redaction:operator-safe";
    const SECTION_ID: &str = "section:agents";
    const SECTION_TITLE: &str = "Agents";
    const SUBJECT_ID: &str = "agent:1";
    const RELATED_SUBJECT_ID: &str = "agent:2";
    const SUBJECT_KIND: &str = "agent";
    const SUMMARY: &str = "waiting for reviewed input";
    const SOURCE_GENERATION: u64 = 3;
    const SNAPSHOT_REVISION: u64 = 7;
    const PAGE_SECTION_LIMIT: usize = 2;
    const PAGE_ITEM_LIMIT: usize = 2;
    const PAGE_RELATION_LIMIT: usize = 2;
    const PAGE_SUMMARY_BYTE_LIMIT: usize = 128;
    const PAGE_START: usize = 0;
    const NEXT_OFFSET: usize = 1;
    const REMAINING_ITEMS: usize = 1;
    const QUEUE_COUNT: u64 = 4;

    fn limits() -> InspectionLimits {
        InspectionLimits {
            max_sections: PAGE_SECTION_LIMIT,
            max_items_per_section: PAGE_ITEM_LIMIT,
            max_relations_per_item: PAGE_RELATION_LIMIT,
            max_summary_bytes: PAGE_SUMMARY_BYTE_LIMIT,
        }
    }

    fn subject(subject_id: &str) -> InspectionSubject {
        InspectionSubject {
            source_id: SOURCE_ID.to_string(),
            subject_id: subject_id.to_string(),
            subject_kind: SUBJECT_KIND.to_string(),
        }
    }

    fn item(subject_id: &str) -> InspectionItem {
        InspectionItem {
            subject: subject(subject_id),
            status: InspectionItemStatus::Waiting,
            summary: SUMMARY.to_string(),
            payload_class: InspectionPayloadClass::SafeSummary,
            counts: vec![InspectionCount {
                kind: "queued_events".to_string(),
                value: QUEUE_COUNT,
            }],
            relations: Vec::new(),
        }
    }

    fn continuation() -> InspectionContinuation {
        InspectionContinuation {
            cursor: InspectionCursor {
                source_id: SOURCE_ID.to_string(),
                source_generation: SOURCE_GENERATION,
                snapshot_revision: SNAPSHOT_REVISION,
                policy_id: POLICY_ID.to_string(),
                redaction_profile_id: REDACTION_ID.to_string(),
                limits: limits(),
                section_id: SECTION_ID.to_string(),
                next_offset: NEXT_OFFSET,
            },
            remaining_items: REMAINING_ITEMS,
        }
    }

    fn snapshot() -> InspectionSnapshot {
        InspectionSnapshot {
            source: InspectionSource {
                source_id: SOURCE_ID.to_string(),
                generation: SOURCE_GENERATION,
                status: InspectionSourceStatus::Partial,
                features: vec![
                    InspectionFeature::Snapshot,
                    InspectionFeature::Continuation,
                    InspectionFeature::Relations,
                    InspectionFeature::Counts,
                    InspectionFeature::Freshness,
                ],
            },
            freshness: InspectionFreshness {
                source_generation: SOURCE_GENERATION,
                snapshot_revision: SNAPSHOT_REVISION,
            },
            policy_id: POLICY_ID.to_string(),
            redaction: InspectionRedaction {
                profile_id: REDACTION_ID.to_string(),
                status: InspectionRedactionStatus::Redacted,
            },
            limits: limits(),
            sections: vec![InspectionSection {
                section_id: SECTION_ID.to_string(),
                title: SECTION_TITLE.to_string(),
                status: InspectionSectionStatus::Partial,
                page_start: PAGE_START,
                items: vec![item(SUBJECT_ID)],
                continuation: Some(continuation()),
            }],
        }
    }

    // r[verify host_control.inspection.model]
    // r[verify host_control.inspection.validation]
    // r[verify host_control.inspection.verification]
    #[test]
    fn snapshot_first_page_and_matching_continuation_are_valid() {
        let snapshot = snapshot();

        assert_eq!(snapshot.clone().validate(), Ok(snapshot));
        assert_eq!(validate_inspection_intent("observe"), Ok(()));
    }

    // r[verify host_control.inspection.relations]
    #[test]
    fn typed_relation_resolves_within_the_snapshot() {
        let mut snapshot = snapshot();
        snapshot.sections[0].status = InspectionSectionStatus::Ready;
        snapshot.sections[0].continuation = None;
        snapshot.sections[0].items.push(item(RELATED_SUBJECT_ID));
        snapshot.sections[0].items[0]
            .relations
            .push(InspectionRelation {
                kind: "waits_for".to_string(),
                direction: InspectionRelationDirection::Outbound,
                scope: InspectionRelationScope::Snapshot,
                target: subject(RELATED_SUBJECT_ID),
                allows_self_cycle: false,
            });

        assert!(snapshot.validate().is_ok());
    }

    // r[verify host_control.inspection.validation]
    // r[verify host_control.inspection.verification]
    #[test]
    fn stale_duplicate_ambiguous_and_oversized_pages_fail_closed() {
        let mut stale = snapshot();
        stale.sections[0]
            .continuation
            .as_mut()
            .expect("fixture continuation exists")
            .cursor
            .snapshot_revision += 1;
        let mut duplicate_section = snapshot();
        duplicate_section
            .sections
            .push(duplicate_section.sections[0].clone());
        let mut duplicate_subject = snapshot();
        duplicate_subject.sections[0].status = InspectionSectionStatus::Ready;
        duplicate_subject.sections[0].continuation = None;
        duplicate_subject.sections[0].items.push(item(SUBJECT_ID));
        let mut oversized = snapshot();
        oversized.limits.max_items_per_section = 1;
        oversized.sections[0].status = InspectionSectionStatus::Ready;
        oversized.sections[0].continuation = None;
        oversized.sections[0].items.push(item(RELATED_SUBJECT_ID));

        assert_eq!(stale.validate(), Err(InspectionError::StaleContinuation));
        assert_eq!(
            duplicate_section.validate(),
            Err(InspectionError::DuplicateSection)
        );
        assert_eq!(
            duplicate_subject.validate(),
            Err(InspectionError::DuplicateSubject)
        );
        assert!(matches!(
            oversized.validate(),
            Err(InspectionError::LimitExceeded { .. })
        ));
    }

    // r[verify host_control.inspection.boundary]
    // r[verify host_control.inspection.read_only]
    // r[verify host_control.inspection.verification]
    #[test]
    fn unknown_sources_unsafe_payloads_mutations_and_bad_relations_fail_closed() {
        let mut unknown_source = snapshot();
        unknown_source.sections[0].items[0].subject.source_id = "source:unknown".to_string();
        let mut unsafe_payload = snapshot();
        unsafe_payload.sections[0].items[0].payload_class = InspectionPayloadClass::Credential;
        let mutation = validate_inspection_intent("cancel");
        let mut dangling = snapshot();
        dangling.sections[0].items[0]
            .relations
            .push(InspectionRelation {
                kind: "waits_for".to_string(),
                direction: InspectionRelationDirection::Outbound,
                scope: InspectionRelationScope::Snapshot,
                target: subject(RELATED_SUBJECT_ID),
                allows_self_cycle: false,
            });
        let mut ambiguous = snapshot();
        let relation = InspectionRelation {
            kind: "related".to_string(),
            direction: InspectionRelationDirection::Outbound,
            scope: InspectionRelationScope::Snapshot,
            target: subject(SUBJECT_ID),
            allows_self_cycle: true,
        };
        ambiguous.sections[0].items[0].relations = vec![relation.clone(), relation];
        let mut forbidden_cycle = snapshot();
        forbidden_cycle.sections[0].items[0]
            .relations
            .push(InspectionRelation {
                kind: "depends_on".to_string(),
                direction: InspectionRelationDirection::Outbound,
                scope: InspectionRelationScope::Snapshot,
                target: subject(SUBJECT_ID),
                allows_self_cycle: false,
            });

        assert_eq!(
            unknown_source.validate(),
            Err(InspectionError::UnknownSource)
        );
        assert_eq!(
            unsafe_payload.validate(),
            Err(InspectionError::UnsafePayloadClass)
        );
        assert_eq!(mutation, Err(InspectionError::MutationIntent));
        assert_eq!(dangling.validate(), Err(InspectionError::DanglingRelation));
        assert_eq!(
            ambiguous.validate(),
            Err(InspectionError::DuplicateRelation)
        );
        assert_eq!(
            forbidden_cycle.validate(),
            Err(InspectionError::ForbiddenRelationCycle)
        );
    }

    // r[verify host_control.inspection.consumers]
    #[test]
    fn frozen_consumer_projections_preserve_product_ownership() {
        let fixtures = [
            (
                InspectionConsumer::Animus,
                "agent_runtime",
                "agent",
                "agent_status",
            ),
            (
                InspectionConsumer::Lattice,
                "workflow_runtime",
                "workflow_node",
                "node_status",
            ),
            (
                InspectionConsumer::Molten,
                "dataspace_runtime",
                "actor",
                "actor_status",
            ),
            (
                InspectionConsumer::Tile,
                "workspace_runtime",
                "pane",
                "pane_status",
            ),
        ];
        for (consumer, source_kind, subject_kind, summary_kind) in fixtures {
            let projection = ConsumerProjection {
                consumer,
                source_kind: source_kind.to_string(),
                subject_kind: subject_kind.to_string(),
                summary_kind: summary_kind.to_string(),
                preserves_product_identity: true,
                product_retains_authority: true,
            };
            assert_eq!(validate_consumer_projection(&projection), Ok(()));
        }
    }

    #[test]
    fn lossy_or_authority_promoting_projection_is_rejected() {
        let mut lossy = ConsumerProjection {
            consumer: InspectionConsumer::Animus,
            source_kind: "agent_runtime".to_string(),
            subject_kind: "agent".to_string(),
            summary_kind: "generic_blob".to_string(),
            preserves_product_identity: true,
            product_retains_authority: true,
        };
        assert_eq!(
            validate_consumer_projection(&lossy),
            Err(InspectionError::LossyConsumerProjection)
        );
        lossy.summary_kind = "agent_status".to_string();
        lossy.product_retains_authority = false;
        assert_eq!(
            validate_consumer_projection(&lossy),
            Err(InspectionError::AuthorityPromotingProjection)
        );
    }

    #[test]
    fn unavailable_sources_and_sections_cannot_claim_visible_data() {
        let mut unavailable_source = snapshot();
        unavailable_source.source.status = InspectionSourceStatus::Unavailable;
        let mut unavailable_section = snapshot();
        unavailable_section.sections[0].status = InspectionSectionStatus::Unavailable;
        unavailable_section.sections[0].continuation = None;

        assert_eq!(
            unavailable_source.validate(),
            Err(InspectionError::UnavailableSourceContainsData)
        );
        assert_eq!(
            unavailable_section.validate(),
            Err(InspectionError::UnavailableSourceContainsData)
        );
    }
}
