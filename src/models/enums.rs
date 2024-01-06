use std::fmt;
use std::fmt::Formatter;
use serde::Deserialize;
use crate::models::structs::ValidTicketEntry;

#[derive(Debug, Clone, Deserialize)]
pub enum VisibilityEnum {
    PUBLIC,
    PRIVATE,
    MODERATED,
}

impl fmt::Display for VisibilityEnum {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            VisibilityEnum::PUBLIC => write!(f, "PUBLIC"),
            VisibilityEnum::PRIVATE => write!(f, "PRIVATE"),
            VisibilityEnum::MODERATED => write!(f, "MODERATED"),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum RoleEnum {
    SiteConsumer,
    SiteCollaborator,
    SiteContributor,
    SiteManager,
}

impl fmt::Display for RoleEnum {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            RoleEnum::SiteCollaborator => write!(f, "SiteCollaborator"),
            RoleEnum::SiteConsumer => write!(f, "SiteConsumer"),
            RoleEnum::SiteContributor => write!(f, "SiteContributor"),
            RoleEnum::SiteManager => write!(f, "SiteManager"),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum MemberTypeEnum {
    GROUP,
    PERSON,
}

impl fmt::Display for MemberTypeEnum {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            MemberTypeEnum::GROUP => write!(f, "GROUP"),
            MemberTypeEnum::PERSON => write!(f, "PERSON"),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum SubscriptionLevelEnum {
    Free,
    Standard,
    Enterprise,
}

impl fmt::Display for SubscriptionLevelEnum {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            SubscriptionLevelEnum::Free => write!(f, "Free"),
            SubscriptionLevelEnum::Standard => write!(f, "Standard"),
            SubscriptionLevelEnum::Enterprise => write!(f, "Enterprise"),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum RatingIdEnum {
    Likes,
    FiveStar,
}

impl fmt::Display for RatingIdEnum {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            RatingIdEnum::Likes => write!(f, "Likes"),
            RatingIdEnum::FiveStar => write!(f, "FiveStar"),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum RatingValueEnum {
    Bool(bool),
    Number(i32),
}

impl fmt::Display for RatingValueEnum {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            RatingValueEnum::Bool(value) => write!(f, "{}", value),
            RatingValueEnum::Number(value) => write!(f, "{}", value),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum AccessStatusEnum {
    ALLOWED,
    DENIED,
}

impl fmt::Display for AccessStatusEnum {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            AccessStatusEnum::ALLOWED => write!(f, "ALLOWED"),
            AccessStatusEnum::DENIED => write!(f, "DENIED"),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum NodeBodyLockTypeEnum {
    #[serde(rename = "ALLOW_OWNER_CHANGES")]
    AllowOwnerChanges,
    FULL,
}

impl fmt::Display for NodeBodyLockTypeEnum {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            NodeBodyLockTypeEnum::AllowOwnerChanges => write!(f, "ALLOW_OWNER_CHANGES"),
            NodeBodyLockTypeEnum::FULL => write!(f, "FULL"),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum NodeBodyLockLifetimeEnum {
    PERSISTENT,
    EPHEMERAL,
}

impl fmt::Display for NodeBodyLockLifetimeEnum {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            NodeBodyLockLifetimeEnum::PERSISTENT => write!(f, "PERSISTENT"),
            NodeBodyLockLifetimeEnum::EPHEMERAL => write!(f, "EPHEMERAL"),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum RenditionStatusEnum {
    CREATED,
    #[serde(rename = "NOT_CREATED")]
    NotCreated,
}

impl fmt::Display for RenditionStatusEnum {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            RenditionStatusEnum::CREATED => write!(f, "CREATED"),
            RenditionStatusEnum::NotCreated => write!(f, "NOT_CREATED"),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum DownloadStatusEnum {
    PENDING,
    CANCELLED,
    #[serde(rename = "IN_PROGRESS")]
    InProgress,
    DONE,
    #[serde(rename = "MAX_CONTENT_SIZE_EXCEEDED")]
    MaxContentSizeExceeded,
}

impl fmt::Display for DownloadStatusEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DownloadStatusEnum::PENDING => write!(f, "PENDING"),
            DownloadStatusEnum::CANCELLED => write!(f, "CANCELLED"),
            DownloadStatusEnum::InProgress => write!(f, "IN_PROGRESS"),
            DownloadStatusEnum::DONE => write!(f, "DONE"),
            DownloadStatusEnum::MaxContentSizeExceeded => write!(f, "MAX_CONTENT_SIZE_EXCEEDED"),
        }
    }
}

pub enum TicketValidationResult {
    Valid(ValidTicketEntry),
    InvalidTicket
}
















