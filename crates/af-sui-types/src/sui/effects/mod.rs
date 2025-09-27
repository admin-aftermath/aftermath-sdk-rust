// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::collections::{BTreeMap, HashSet};

use sui_sdk_types::{
    Address,
    Digest,
    EpochId,
    ExecutionStatus,
    GasCostSummary,
    IdOperation,
    ObjectIn,
    ObjectOut,
    ObjectReference,
    ObjectReferenceWithOwner,
    Owner,
    TransactionEffects,
    TransactionEffectsV1,
    TransactionEffectsV2,
    UnchangedConsensusKind,
    Version,
};

use crate::{OBJECT_DIGEST_DELETED, OBJECT_DIGEST_WRAPPED};

mod api;

pub use self::api::{InputSharedObject, ObjectChange, TransactionEffectsAPI};

macro_rules! dispatch {
    ($self:ident, $method:ident) => {
        match $self {
            Self::V1(e) => e.$method(),
            Self::V2(e) => e.$method(),
        }
    };
}

impl TransactionEffectsAPI for TransactionEffects {
    fn status(&self) -> &ExecutionStatus {
        dispatch!(self, status)
    }

    fn into_status(self) -> ExecutionStatus {
        dispatch!(self, into_status)
    }

    fn executed_epoch(&self) -> EpochId {
        dispatch!(self, executed_epoch)
    }

    fn modified_at_versions(&self) -> Vec<(Address, Version)> {
        dispatch!(self, modified_at_versions)
    }

    fn lamport_version(&self) -> Version {
        dispatch!(self, lamport_version)
    }

    fn old_object_metadata(&self) -> Vec<(ObjectReference, Owner)> {
        dispatch!(self, old_object_metadata)
    }

    fn sequenced_input_shared_objects(&self) -> Vec<InputSharedObject> {
        dispatch!(self, sequenced_input_shared_objects)
    }

    fn created(&self) -> Vec<(ObjectReference, Owner)> {
        dispatch!(self, created)
    }

    fn mutated(&self) -> Vec<(ObjectReference, Owner)> {
        dispatch!(self, mutated)
    }

    fn unwrapped(&self) -> Vec<(ObjectReference, Owner)> {
        dispatch!(self, unwrapped)
    }

    fn deleted(&self) -> Vec<ObjectReference> {
        dispatch!(self, deleted)
    }

    fn unwrapped_then_deleted(&self) -> Vec<ObjectReference> {
        dispatch!(self, unwrapped_then_deleted)
    }

    fn wrapped(&self) -> Vec<ObjectReference> {
        dispatch!(self, wrapped)
    }

    fn object_changes(&self) -> Vec<ObjectChange> {
        dispatch!(self, object_changes)
    }

    fn gas_object(&self) -> Option<(ObjectReference, Owner)> {
        dispatch!(self, gas_object)
    }

    fn events_digest(&self) -> Option<&Digest> {
        dispatch!(self, events_digest)
    }

    fn dependencies(&self) -> &[Digest] {
        dispatch!(self, dependencies)
    }

    fn transaction_digest(&self) -> &Digest {
        dispatch!(self, transaction_digest)
    }

    fn gas_cost_summary(&self) -> &GasCostSummary {
        dispatch!(self, gas_cost_summary)
    }

    fn unchanged_shared_objects(&self) -> Vec<(Address, UnchangedConsensusKind)> {
        dispatch!(self, unchanged_shared_objects)
    }
}

impl TransactionEffectsAPI for TransactionEffectsV1 {
    fn status(&self) -> &ExecutionStatus {
        &self.status
    }

    fn into_status(self) -> ExecutionStatus {
        self.status
    }

    fn executed_epoch(&self) -> EpochId {
        self.epoch
    }

    fn modified_at_versions(&self) -> Vec<(Address, Version)> {
        self.modified_at_versions
            .iter()
            // V1 transaction effects "modified_at_versions" includes unwrapped_then_deleted
            // objects, so in order to have parity with the V2 transaction effects semantics of
            // "modified_at_versions", filter out any objects that are unwrapped_then_deleted'ed
            .filter(|key| {
                !self
                    .unwrapped_then_deleted
                    .iter()
                    .any(|deleted| deleted.object_id() == &key.object_id)
            })
            .map(|key| (key.object_id, key.version))
            .collect()
    }

    fn lamport_version(&self) -> Version {
        self.modified_at_versions
            .iter()
            .map(|key| key.version)
            .fold(0, std::cmp::max)
            + 1
    }

    fn old_object_metadata(&self) -> Vec<(ObjectReference, Owner)> {
        unimplemented!("Only supposed by v2 and above");
    }

    fn sequenced_input_shared_objects(&self) -> Vec<InputSharedObject> {
        let modified: HashSet<_> = self
            .modified_at_versions
            .iter()
            .map(|key| key.object_id)
            .collect();
        self.consensus_objects
            .iter()
            .cloned()
            .map(|r| {
                if modified.contains(r.object_id()) {
                    InputSharedObject::Mutate(r)
                } else {
                    InputSharedObject::ReadOnly(r)
                }
            })
            .collect()
    }

    fn created(&self) -> Vec<(ObjectReference, Owner)> {
        self.created.iter().cloned().map(into_parts).collect()
    }

    fn mutated(&self) -> Vec<(ObjectReference, Owner)> {
        self.mutated.iter().cloned().map(into_parts).collect()
    }

    fn unwrapped(&self) -> Vec<(ObjectReference, Owner)> {
        self.unwrapped.iter().cloned().map(into_parts).collect()
    }

    fn deleted(&self) -> Vec<ObjectReference> {
        self.deleted.to_vec()
    }

    fn unwrapped_then_deleted(&self) -> Vec<ObjectReference> {
        self.unwrapped_then_deleted.to_vec()
    }

    fn wrapped(&self) -> Vec<ObjectReference> {
        self.wrapped.to_vec()
    }

    fn object_changes(&self) -> Vec<ObjectChange> {
        let modified_at: BTreeMap<_, _> = self
            .modified_at_versions
            .iter()
            .map(|m| (m.object_id, m.version))
            .collect();

        let created = self.created.iter().map(|r| ObjectChange {
            id: *r.reference.object_id(),
            input_version: None,
            input_digest: None,
            output_version: Some(r.reference.version()),
            output_digest: Some(*r.reference.digest()),
            id_operation: IdOperation::Created,
        });

        let mutated = self.mutated.iter().map(|r| ObjectChange {
            id: *r.reference.object_id(),
            input_version: modified_at.get(r.reference.object_id()).copied(),
            input_digest: None,
            output_version: Some(r.reference.version()),
            output_digest: Some(*r.reference.digest()),
            id_operation: IdOperation::None,
        });

        let unwrapped = self.unwrapped.iter().map(|r| ObjectChange {
            id: *r.reference.object_id(),
            input_version: None,
            input_digest: None,
            output_version: Some(r.reference.version()),
            output_digest: Some(*r.reference.digest()),
            id_operation: IdOperation::None,
        });

        let deleted = self.deleted.iter().map(|r| ObjectChange {
            id: *r.object_id(),
            input_version: modified_at.get(r.object_id()).copied(),
            input_digest: None,
            output_version: None,
            output_digest: None,
            id_operation: IdOperation::Deleted,
        });

        let unwrapped_then_deleted = self.unwrapped_then_deleted.iter().map(|r| ObjectChange {
            id: *r.object_id(),
            input_version: None,
            input_digest: None,
            output_version: None,
            output_digest: None,
            id_operation: IdOperation::Deleted,
        });

        let wrapped = self.wrapped.iter().map(|r| ObjectChange {
            id: *r.object_id(),
            input_version: modified_at.get(r.object_id()).copied(),
            input_digest: None,
            output_version: None,
            output_digest: None,
            id_operation: IdOperation::None,
        });

        created
            .chain(mutated)
            .chain(unwrapped)
            .chain(deleted)
            .chain(unwrapped_then_deleted)
            .chain(wrapped)
            .collect()
    }

    fn gas_object(&self) -> Option<(ObjectReference, Owner)> {
        Some(into_parts(self.gas_object.clone()))
    }
    fn events_digest(&self) -> Option<&Digest> {
        self.events_digest.as_ref()
    }

    fn dependencies(&self) -> &[Digest] {
        &self.dependencies
    }

    fn transaction_digest(&self) -> &Digest {
        &self.transaction_digest
    }

    fn gas_cost_summary(&self) -> &GasCostSummary {
        &self.gas_used
    }

    fn unchanged_shared_objects(&self) -> Vec<(Address, UnchangedConsensusKind)> {
        self.sequenced_input_shared_objects()
            .iter()
            .filter_map(|o| match o {
                // In effects v1, the only unchanged shared objects are read-only shared objects.
                InputSharedObject::ReadOnly(oref) => Some((
                    *oref.object_id(),
                    UnchangedConsensusKind::ReadOnlyRoot {
                        version: oref.version(),
                        digest: *oref.digest(),
                    },
                )),
                _ => None,
            })
            .collect()
    }
}

impl TransactionEffectsAPI for TransactionEffectsV2 {
    fn status(&self) -> &ExecutionStatus {
        &self.status
    }

    fn into_status(self) -> ExecutionStatus {
        self.status
    }

    fn executed_epoch(&self) -> EpochId {
        self.epoch
    }

    fn modified_at_versions(&self) -> Vec<(Address, Version)> {
        self.changed_objects
            .iter()
            .filter_map(|c| {
                if let ObjectIn::Exist { version, .. } = &c.input_state {
                    Some((c.object_id, *version))
                } else {
                    None
                }
            })
            .collect()
    }

    fn lamport_version(&self) -> Version {
        self.lamport_version
    }

    fn old_object_metadata(&self) -> Vec<(ObjectReference, Owner)> {
        self.changed_objects
            .iter()
            .filter_map(|c| {
                if let ObjectIn::Exist {
                    version,
                    digest,
                    owner,
                } = c.input_state
                {
                    Some((oref(c.object_id, version, digest), owner))
                } else {
                    None
                }
            })
            .collect()
    }

    fn sequenced_input_shared_objects(&self) -> Vec<InputSharedObject> {
        self.changed_objects
            .iter()
            .filter_map(|c| match c.input_state {
                ObjectIn::Exist {
                    version, digest, ..
                } => Some(InputSharedObject::Mutate(oref(
                    c.object_id,
                    version,
                    digest,
                ))),
                _ => None,
            })
            .chain(
                self.unchanged_consensus_objects
                    .iter()
                    .filter_map(|u| match u.kind {
                        UnchangedConsensusKind::ReadOnlyRoot { version, digest } => Some(
                            InputSharedObject::ReadOnly(oref(u.object_id, version, digest)),
                        ),
                        UnchangedConsensusKind::MutateDeleted { version } => {
                            Some(InputSharedObject::MutateDeleted(u.object_id, version))
                        }
                        UnchangedConsensusKind::ReadDeleted { version } => {
                            Some(InputSharedObject::ReadDeleted(u.object_id, version))
                        }
                        UnchangedConsensusKind::Canceled { version } => {
                            Some(InputSharedObject::Canceled(u.object_id, version))
                        }
                        UnchangedConsensusKind::PerEpochConfigWithSequenceNumber { .. } => None,
                        // We can not expose the per epoch config object as input shared object,
                        // since it does not require sequencing, and hence shall not be considered
                        // as a normal input shared object.
                        UnchangedConsensusKind::PerEpochConfig => None,
                        _ => panic!("unknown UnchangedConsensusKind variant"),
                    }),
            )
            .collect()
    }

    fn created(&self) -> Vec<(ObjectReference, Owner)> {
        self.changed_objects
            .iter()
            .filter_map(
                |c| match (&c.input_state, &c.output_state, c.id_operation) {
                    (
                        ObjectIn::NotExist,
                        ObjectOut::ObjectWrite { digest, owner },
                        IdOperation::Created,
                    ) => Some((oref(c.object_id, self.lamport_version, *digest), *owner)),
                    (
                        ObjectIn::NotExist,
                        ObjectOut::PackageWrite { version, digest },
                        IdOperation::Created,
                    ) => Some((oref(c.object_id, *version, *digest), Owner::Immutable)),
                    _ => None,
                },
            )
            .collect()
    }

    fn mutated(&self) -> Vec<(ObjectReference, Owner)> {
        self.changed_objects
            .iter()
            .filter_map(|c| match (&c.input_state, &c.output_state) {
                (ObjectIn::Exist { .. }, ObjectOut::ObjectWrite { digest, owner }) => {
                    Some((oref(c.object_id, self.lamport_version, *digest), *owner))
                }
                (ObjectIn::Exist { .. }, ObjectOut::PackageWrite { version, digest }) => {
                    Some((oref(c.object_id, *version, *digest), Owner::Immutable))
                }
                _ => None,
            })
            .collect()
    }

    fn unwrapped(&self) -> Vec<(ObjectReference, Owner)> {
        self.changed_objects
            .iter()
            .filter_map(
                |c| match (&c.input_state, &c.output_state, &c.id_operation) {
                    (
                        ObjectIn::NotExist,
                        ObjectOut::ObjectWrite { digest, owner },
                        IdOperation::None,
                    ) => Some((oref(c.object_id, self.lamport_version, *digest), *owner)),
                    _ => None,
                },
            )
            .collect()
    }

    fn deleted(&self) -> Vec<ObjectReference> {
        self.changed_objects
            .iter()
            .filter_map(
                |c| match (&c.input_state, &c.output_state, &c.id_operation) {
                    (ObjectIn::Exist { .. }, ObjectOut::NotExist, IdOperation::Deleted) => Some(
                        oref(c.object_id, self.lamport_version, OBJECT_DIGEST_DELETED),
                    ),
                    _ => None,
                },
            )
            .collect()
    }

    fn unwrapped_then_deleted(&self) -> Vec<ObjectReference> {
        self.changed_objects
            .iter()
            .filter_map(
                |c| match (&c.input_state, &c.output_state, &c.id_operation) {
                    (ObjectIn::NotExist, ObjectOut::NotExist, IdOperation::Deleted) => Some(oref(
                        c.object_id,
                        self.lamport_version,
                        OBJECT_DIGEST_DELETED,
                    )),
                    _ => None,
                },
            )
            .collect()
    }

    fn wrapped(&self) -> Vec<ObjectReference> {
        self.changed_objects
            .iter()
            .filter_map(
                |c| match (&c.input_state, &c.output_state, &c.id_operation) {
                    (ObjectIn::Exist { .. }, ObjectOut::NotExist, IdOperation::None) => Some(oref(
                        c.object_id,
                        self.lamport_version,
                        OBJECT_DIGEST_WRAPPED,
                    )),
                    _ => None,
                },
            )
            .collect()
    }

    fn object_changes(&self) -> Vec<ObjectChange> {
        self.changed_objects
            .iter()
            .map(|c| {
                let input_version_digest = match &c.input_state {
                    ObjectIn::NotExist => None,
                    ObjectIn::Exist {
                        version, digest, ..
                    } => Some((*version, *digest)),
                    _ => panic!("unknown ObjectIn variant"),
                };

                let output_version_digest = match &c.output_state {
                    ObjectOut::NotExist => None,
                    ObjectOut::ObjectWrite { digest, .. } => Some((self.lamport_version, *digest)),
                    ObjectOut::PackageWrite {
                        version, digest, ..
                    } => Some((*version, *digest)),
                    _ => panic!("unknown ObjectOut variant"),
                };

                ObjectChange {
                    id: c.object_id,

                    input_version: input_version_digest.map(|k| k.0),
                    input_digest: input_version_digest.map(|k| k.1),

                    output_version: output_version_digest.map(|k| k.0),
                    output_digest: output_version_digest.map(|k| k.1),

                    id_operation: c.id_operation,
                }
            })
            .collect()
    }

    fn gas_object(&self) -> Option<(ObjectReference, Owner)> {
        self.gas_object_index.map(|gas_object_index| {
            let entry = &self.changed_objects[gas_object_index as usize];
            match entry.output_state {
                ObjectOut::ObjectWrite { digest, owner } => {
                    (oref(entry.object_id, self.lamport_version, digest), owner)
                }
                _ => panic!("Gas object must be an ObjectWrite in changed_objects"),
            }
        })
    }

    fn events_digest(&self) -> Option<&Digest> {
        self.events_digest.as_ref()
    }

    fn dependencies(&self) -> &[Digest] {
        &self.dependencies
    }

    fn transaction_digest(&self) -> &Digest {
        &self.transaction_digest
    }

    fn gas_cost_summary(&self) -> &GasCostSummary {
        &self.gas_used
    }

    fn unchanged_shared_objects(&self) -> Vec<(Address, UnchangedConsensusKind)> {
        self.unchanged_consensus_objects
            .clone()
            .into_iter()
            .map(|u| (u.object_id, u.kind))
            .collect()
    }
}

const fn into_parts(r: ObjectReferenceWithOwner) -> (ObjectReference, Owner) {
    (r.reference, r.owner)
}

fn oref(id: Address, version: Version, digest: Digest) -> ObjectReference {
    ObjectReference::new(id, version, digest)
}
