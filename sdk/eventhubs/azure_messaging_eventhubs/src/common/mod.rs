// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::{
    error::{ErrorKind, EventHubsError},
    models::{EventHubPartitionProperties, EventHubProperties},
};
use azure_core::error::Result;
use azure_core_amqp::{
    AmqpManagement, AmqpManagementApis, AmqpOrderedMap, AmqpTimestamp, AmqpValue,
};
use std::time::SystemTime;

pub(crate) mod connection_manager;
pub(crate) mod user_agent;

pub(crate) struct ManagementInstance {
    pub management: AmqpManagement,
}

const EVENTHUB_ENTITY_TYPE: &str = "com.microsoft:eventhub";
const PARTITION_ENTITY_TYPE: &str = "com.microsoft:partition";

const EVENTHUB_PROPERTY_PARTITION_COUNT: &str = "partition_count";
const EVENTHUB_PROPERTY_PARTITION_IDS: &str = "partition_ids";
const EVENTHUB_PROPERTY_NAME: &str = "name";
const EVENTHUB_PROPERTY_PARTITION: &str = "partition";
const EVENTHUB_PROPERTY_CREATED_AT: &str = "created_at";

const EVENTHUB_PARTITION_PROPERTIES_TYPE: &str = "type";
const EVENTHUB_PARTITION_PROPERTIES_LAST_ENQUEUED_SEQUENCE_NUMBER_EPOCH: &str =
    "last_enqueued_sequence_number_epoch";
const EVENTHUB_PARTITION_PROPERTIES_BEGIN_SEQUENCE_NUMBER: &str = "begin_sequence_number";
const EVENTHUB_PARTITION_PROPERTIES_LAST_ENQUEUED_SEQUENCE_NUMBER: &str =
    "last_enqueued_sequence_number";
const EVENTHUB_PARTITION_PROPERTIES_LAST_ENQUEUED_OFFSET: &str = "last_enqueued_offset";
const EVENTHUB_PARTITION_PROPERTIES_LAST_ENQUEUED_TIME_UTC: &str = "last_enqueued_time_utc";
const EVENTHUB_PARTITION_PROPERTIES_IS_EMPTY: &str = "is_partition_empty";

impl ManagementInstance {
    pub fn new(management: AmqpManagement) -> Self {
        Self { management }
    }

    pub async fn get_eventhub_properties(&self, eventhub: &str) -> Result<EventHubProperties> {
        let mut application_properties: AmqpOrderedMap<String, AmqpValue> = AmqpOrderedMap::new();
        application_properties.insert(EVENTHUB_PROPERTY_NAME.to_string(), eventhub.into());

        let response = self
            .management
            .call(EVENTHUB_ENTITY_TYPE.to_string(), application_properties)
            .await?;

        if !response.contains_key(EVENTHUB_PROPERTY_PARTITION_COUNT) {
            return Err(EventHubsError::from(ErrorKind::InvalidManagementResponse).into());
        }
        let name: String = response
            .get(EVENTHUB_PROPERTY_NAME)
            .ok_or_else(|| EventHubsError::from(ErrorKind::InvalidManagementResponse))?
            .clone()
            .into();
        let created_at: Option<SystemTime> = Into::<AmqpTimestamp>::into(
            response
                .get(EVENTHUB_PROPERTY_CREATED_AT)
                .ok_or_else(|| EventHubsError::from(ErrorKind::InvalidManagementResponse))?
                .clone(),
        )
        .0;

        let partition_ids = response
            .get(EVENTHUB_PROPERTY_PARTITION_IDS)
            .ok_or_else(|| EventHubsError::from(ErrorKind::InvalidManagementResponse))?;

        let partition_ids = match partition_ids {
            AmqpValue::Array(partition_ids) => partition_ids
                .iter()
                .map(|id| match id {
                    AmqpValue::String(id) => Ok(id.clone()),
                    _ => Err(EventHubsError::from(ErrorKind::InvalidManagementResponse).into()),
                })
                .collect::<Result<Vec<String>>>()?,
            _ => return Err(EventHubsError::from(ErrorKind::InvalidManagementResponse).into()),
        };
        Ok(EventHubProperties {
            name,
            created_on: created_at,
            partition_ids,
        })
    }

    pub async fn get_eventhub_partition_properties(
        &self,
        eventhub: &str,
        partition_id: &str,
    ) -> Result<EventHubPartitionProperties> {
        let mut application_properties: AmqpOrderedMap<String, AmqpValue> = AmqpOrderedMap::new();
        application_properties.insert(EVENTHUB_PROPERTY_NAME.to_string(), eventhub.into());
        application_properties.insert(EVENTHUB_PROPERTY_PARTITION.to_string(), partition_id.into());

        let response = self
            .management
            .call(PARTITION_ENTITY_TYPE.to_string(), application_properties)
            .await?;

        // Look for the required response properties
        if !response.contains_key(EVENTHUB_PARTITION_PROPERTIES_TYPE)
            || !response
                .contains_key(EVENTHUB_PARTITION_PROPERTIES_LAST_ENQUEUED_SEQUENCE_NUMBER_EPOCH)
        {
            return Err(EventHubsError::from(ErrorKind::InvalidManagementResponse).into());
        }

        Ok(EventHubPartitionProperties {
            beginning_sequence_number: response
                .get(EVENTHUB_PARTITION_PROPERTIES_BEGIN_SEQUENCE_NUMBER)
                .ok_or_else(|| EventHubsError::from(ErrorKind::InvalidManagementResponse))?
                .clone()
                .into(),
            id: response
                .get(EVENTHUB_PROPERTY_PARTITION)
                .ok_or_else(|| EventHubsError::from(ErrorKind::InvalidManagementResponse))?
                .clone()
                .into(),
            eventhub: response
                .get(EVENTHUB_PROPERTY_NAME)
                .ok_or_else(|| EventHubsError::from(ErrorKind::InvalidManagementResponse))?
                .clone()
                .into(),

            last_enqueued_sequence_number: response
                .get(EVENTHUB_PARTITION_PROPERTIES_LAST_ENQUEUED_SEQUENCE_NUMBER)
                .ok_or_else(|| EventHubsError::from(ErrorKind::InvalidManagementResponse))?
                .clone()
                .into(),
            last_enqueued_offset: response
                .get(EVENTHUB_PARTITION_PROPERTIES_LAST_ENQUEUED_OFFSET)
                .ok_or_else(|| EventHubsError::from(ErrorKind::InvalidManagementResponse))?
                .clone()
                .into(),
            last_enqueued_time_utc: Into::<AmqpTimestamp>::into(
                response
                    .get(EVENTHUB_PARTITION_PROPERTIES_LAST_ENQUEUED_TIME_UTC)
                    .ok_or_else(|| EventHubsError::from(ErrorKind::InvalidManagementResponse))?
                    .clone(),
            )
            .0,
            is_empty: response
                .get(EVENTHUB_PARTITION_PROPERTIES_IS_EMPTY)
                .ok_or_else(|| EventHubsError::from(ErrorKind::InvalidManagementResponse))?
                .clone()
                .into(),
        })
    }
}
