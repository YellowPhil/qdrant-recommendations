use qdrant_client::qdrant::{PointId, RetrievedPoint, ScoredPoint, point_id::PointIdOptions};

use crate::{TOPIC_CONTENT_KEY, TOPIC_NAME_KEY};

use super::Idea;

pub fn try_extract_u64_id(id: PointId) -> Option<u64> {
    match &id.point_id_options {
        Some(PointIdOptions::Num(num)) => Some(*num),
        Some(PointIdOptions::Uuid(_)) => panic!("Expected u64 id, got UUID"),
        None => None,
    }
}

impl From<ScoredPoint> for Idea {
    fn from(point: ScoredPoint) -> Self {
        Self {
            id: point.id.and_then(try_extract_u64_id),
            topic_name: point.payload.get(TOPIC_NAME_KEY).unwrap().to_string(),
            content: point.payload.get(TOPIC_CONTENT_KEY).unwrap().to_string(),
        }
    }
}

impl From<&ScoredPoint> for Idea {
    fn from(point: &ScoredPoint) -> Self {
        Self {
            id: point.id.clone().and_then(try_extract_u64_id),
            topic_name: point.payload.get(TOPIC_NAME_KEY).unwrap().to_string(),
            content: point.payload.get(TOPIC_CONTENT_KEY).unwrap().to_string(),
        }
    }
}

impl From<RetrievedPoint> for Idea {
    fn from(point: RetrievedPoint) -> Self {
        Self {
            id: point.id.and_then(try_extract_u64_id),
            topic_name: point.payload.get(TOPIC_NAME_KEY).unwrap().to_string(),
            content: point.payload.get(TOPIC_CONTENT_KEY).unwrap().to_string(),
        }
    }
}

impl From<&RetrievedPoint> for Idea {
    fn from(point: &RetrievedPoint) -> Self {
        Self {
            id: point.id.clone().and_then(try_extract_u64_id),
            topic_name: point.payload.get(TOPIC_NAME_KEY).unwrap().to_string(),
            content: point.payload.get(TOPIC_CONTENT_KEY).unwrap().to_string(),
        }
    }
}
