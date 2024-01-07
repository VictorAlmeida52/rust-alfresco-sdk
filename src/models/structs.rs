use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::models::enums::{AccessStatusEnum, DownloadStatusEnum, MemberTypeEnum, NodeBodyLockLifetimeEnum, NodeBodyLockTypeEnum, RatingIdEnum, RatingValueEnum, RenditionStatusEnum, RoleEnum, SubscriptionLevelEnum, VisibilityEnum};

type AnyObj = serde_json::Value;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlfError {
    pub error: AlfErrorProps
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlfErrorProps {
    pub error_key: Option<String>,
    pub status_code: u16,
    pub brief_summary: String,
    pub stack_trace: String,
    pub description_url: String,
    pub log_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Capabilities {
    pub is_admin: bool,
    pub is_guest: bool,
    pub is_mutable: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub count: u64,
    pub has_more_items: bool,
    pub total_items: u64,
    pub skip_count: u64,
    pub max_items: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteBodyCreate {
    pub id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub visibility: VisibilityEnum,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SitePaging {
    pub list: SitePagingList
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SitePagingList {
    pub pagination: Pagination,
    pub entries: Vec<SiteEntry>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteEntry {
    pub entry: Site
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Site {
    pub id: String,
    pub guid: String,
    pub title: String,
    pub description: Option<String>,
    pub visibility: VisibilityEnum,
    pub preset: Option<String>,
    pub role: Option<RoleEnum>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteBodyUpdate {
    pub title: String,
    pub description: String,
    pub visibility: VisibilityEnum,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteContainerPaging {
    pub list: SiteContainerPagingList,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteContainerPagingList {
    pub pagination: Pagination,
    pub entries: Vec<SiteContainerEntry>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteContainerEntry {
    pub entry: SiteContainer,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteContainer {
    pub id: String,
    pub folder_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteMembershipBodyCreate {
    pub role: RoleEnum,
    pub id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteMembershipBodyUpdate {
    pub role: RoleEnum,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteMemberPaging {
    pub list: SiteMemberPagingList,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteMemberPagingList {
    pub pagination: Pagination,
    pub entries: Vec<SiteMemberEntry>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteMemberEntry {
    pub entry: SiteMember,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteMember {
    pub id: String,
    pub person: Person,
    pub role: RoleEnum,
    pub is_member_of_group: Option<bool>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub id: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub avatar_id: Option<String>,
    pub email: String,
    pub skype_id: Option<String>,
    pub google_id: Option<String>,
    pub instant_message_id: Option<String>,
    pub job_title: Option<String>,
    pub location: Option<String>,
    pub company: Option<Company>,
    pub mobile: Option<String>,
    pub telephone: Option<String>,
    pub status_updated_at: Option<String>, // date-time
    pub user_status: Option<String>,
    pub enabled: bool, // default = true
    pub email_notifications_enabled: Option<bool>, // default = true
    pub aspect_names: Option<Vec<String>>,
    pub properties: Option<AnyObj>,
    pub capabilities: Option<Capabilities>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Company {
    pub organization: Option<String>,
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub address3: Option<String>,
    pub postcode: Option<String>,
    pub telephone: Option<String>,
    pub fax: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteGroupPaging {
    pub list: SiteGroupPagingList,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteGroupPagingList {
    pub pagination: Pagination,
    pub entries: Vec<SiteGroupEntry>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteGroupEntry {
    pub entry: SiteGroup,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteGroup {
    pub id: String,
    pub group: GroupMember,
    pub role: RoleEnum,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMember {
    pub id: String,
    pub display_name: String,
    pub member_type: MemberTypeEnum,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteRolePaging {
    pub list: SiteRolePagingList,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteRolePagingList {
    pub pagination: Pagination,
    pub entries: Vec<SiteRoleEntry>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteRoleEntry {
    pub entry: SiteRole,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteRole {
    pub site: Site,
    pub id: String,
    pub guid: String,
    pub role: RoleEnum,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonBodyCreate {
    pub id: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub description: Option<String>,
    pub email: String,
    pub skype_id: Option<String>,
    pub google_id: Option<String>,
    pub instant_message_id: Option<String>,
    pub job_title: Option<String>,
    pub location: Option<String>,
    pub company: Option<Company>,
    pub mobile: Option<String>,
    pub telephone: Option<String>,
    pub user_status: Option<String>,
    pub enabled: Option<bool>, // default = true
    pub email_notifications_enabled: Option<bool>, // default = true
    pub password: String,
    pub aspect_names: Option<Vec<String>>,
    pub properties: Option<AnyObj>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonBodyUpdate {
    pub id: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub description: Option<String>,
    pub email: Option<String>,
    pub skype_id: Option<String>,
    pub google_id: Option<String>,
    pub instant_message_id: Option<String>,
    pub job_title: Option<String>,
    pub location: Option<String>,
    pub company: Option<Company>,
    pub mobile: Option<String>,
    pub telephone: Option<String>,
    pub user_status: Option<String>,
    pub enabled: Option<bool>, // default = true
    pub email_notifications_enabled: Option<bool>, // default = true
    pub password: Option<String>,
    pub aspect_names: Option<Vec<String>>,
    pub properties: Option<AnyObj>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonPaging {
    pub list: PersonPagingList,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonPagingList {
    pub pagination: Pagination,
    pub entries: PersonEntry,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonEntry {
    pub entry: Person,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupPaging {
    pub list: GroupPagingList,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupPagingList {
    pub pagination: Pagination,
    pub entries: GroupEntry,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupEntry {
    pub entry: Group,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub id: String,
    pub display_name: String,
    pub is_root: bool, // default = true
    pub parent_ids: Vec<String>,
    pub zones: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMemberPaging {
    pub list: GroupMemberPagingList,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMemberPagingList {
    pub pagination: Pagination,
    pub entries: Vec<GroupMemberEntry>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMemberEntry {
    pub entry: GroupMember,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMembershipBodyCreate {
    pub id: String,
    pub member_type: MemberTypeEnum,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupBodyCreate {
    pub id: String,
    pub display: String,
    pub parent_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupBodyUpdate {
    pub display_name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentBody {
    pub content: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentPaging {
    pub list: CommentPagingList,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentPagingList {
    pub pagination: Pagination,
    pub entries: CommentEntry,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentEntry {
    pub entry: Comment,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_by: Person,
    pub created_at: String, // date-time
    pub edited: bool,
    pub modified_by: Person,
    pub modified_at: String, // date-time
    pub can_edit: bool,
    pub can_delete: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TagBody {
    pub tag: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TagPaging {
    pub list: TagPagingList,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TagPagingList {
    pub pagination: Pagination,
    pub entries: Vec<TagEntry>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TagEntry {
    pub entry: Tag
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: String,
    pub tag: String,
    pub count: Option<u64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonNetworkPaging {
    pub list: PersonNetworkPagingList,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonNetworkPagingList {
    pub pagination: Pagination,
    pub entries: Vec<PersonNetworkEntry>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonNetworkEntry {
    pub entry: PersonNetwork,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonNetwork {
    pub id: String,
    pub home_network: Option<bool>,
    pub is_enabled: bool,
    pub created_at: Option<String>, // date-time
    pub paid_network: Option<bool>,
    pub subscription_level: Option<SubscriptionLevelEnum>,
    pub quotas: Vec<NetworkQuota>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkQuota {
    pub id: String,
    pub limit: u64,
    pub usage: u64,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RatingBody {
    pub id: RatingIdEnum,
    pub my_rating: RatingValueEnum,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RatingPaging {
    pub list: RatingPagingList,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RatingPagingList {
    pub pagination: Pagination,
    pub entries: Vec<RatingEntry>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RatingEntry {
    pub entry: Rating,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Rating {
    pub id: String,
    pub aggregate: RatingAggregate,
    pub rated_at: Option<String>, // date-time
    pub my_rating: Option<RatingValueEnum>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RatingAggregate {
    pub number_of_ratings: u64,
    pub average: Option<u64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FavoriteBodyCreate {
    pub target: AnyObj,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FavoritePaging {
    pub list: FavoritePagingList,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FavoritePagingList {
    pub pagination: Pagination,
    pub entries: Vec<FavoriteEntry>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FavoriteEntry {
    pub entry: Favorite,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Favorite {
    pub target_guid: String,
    pub created_at: Option<String>, // date-time,
    pub target: AnyObj,
    pub properties: Option<AnyObj>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FavoriteSiteBodyCreate {
    pub id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FavoriteSiteEntry {
    pub entry: FavoriteSite,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FavoriteSite {
    pub id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityPaging {
    pub list: ActivityPagingList,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityPagingList {
    pub pagination: Pagination,
    pub entries: ActivityEntry,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityEntry {
    pub entry: Activity,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub post_person_id: String,
    pub id: u64,
    pub site_id: Option<String>,
    pub posted_at: Option<String>, // date-time
    pub feed_person_id: String,
    pub activity_summary: Option<HashMap<String, String>>,
    pub activity_type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreferencePaging {
    pub list: PreferencePagingList,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreferencePagingList {
    pub pagination: Pagination,
    pub entries: PreferenceEntry,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreferenceEntry {
    pub entry: Preference,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Preference {
    pub id: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteMembershipRequestBodyCreate {
    pub message: Option<String>,
    pub id: String,
    pub title: Option<String>,
    pub client: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteMembershipRequestBodyUpdate {
    pub message: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteMembershipRequestPaging {
    pub list: SiteMembershipRequestPagingList,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteMembershipRequestPagingList {
    pub pagination: Pagination,
    pub entries: Vec<SiteMembershipRequestEntry>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteMembershipRequestEntry {
    pub entry: SiteMembershipRequest,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteMembershipRequest {
    pub id: String,
    pub created_at: String, // date-time
    pub site: Site,
    pub message: Option<String>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteMembershipRequestWithPersonPaging {
    pub list: SiteMembershipRequestWithPersonPagingList,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteMembershipRequestWithPersonPagingList {
    pub pagination: Pagination,
    pub entries: Vec<SiteMembershipRequestWithPersonEntry>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteMembershipRequestWithPersonEntry {
    pub entry: SiteMembershipRequestWithPerson,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteMembershipRequestWithPerson {
    pub id: String,
    pub created_at: String, // date-time
    pub site: Site,
    pub person: Person,
    pub message: Option<String>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteMembershipApprovalBody {
    pub role: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteMembershipRejectionBody {
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub display_name: String,
    pub id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentInfo {
    pub mime_type: String,
    pub mime_type_name: Option<String>,
    pub size_in_bytes: Option<u64>,
    pub encoding: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssociationInfo {
    pub assoc_type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssociationBody {
    pub target_id: String,
    pub assoc_type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildAssociationInfo {
    pub assoc_type: String,
    pub is_primary: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildAssociationBody {
    pub child_id: String,
    pub assoc_type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PathElement {
    pub id: Option<String>,
    pub name: Option<String>,
    pub node_type: Option<String>,
    pub aspect_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PathInfo {
    pub elements: Option<PathElement>,
    pub name: Option<String>,
    pub is_complete: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionElement {
    pub authority_id: Option<String>,
    pub name: Option<String>,
    pub access_status: Option<AccessStatusEnum>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionsInfo {
    pub is_inheritance_enabled: Option<bool>,
    pub inherited: Option<Vec<PermissionElement>>,
    pub locally_set: Option<Vec<PermissionElement>>,
    pub settable: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionsBody {
    pub is_inheritance_enabled: Option<bool>,
    pub locally_set: Option<Vec<PermissionElement>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseAssociation {
    pub assoc_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeBodyCreate {
    pub name: String,
    pub node_type: String,
    pub aspect_names: Option<Vec<String>>,
    pub properties: Option<AnyObj>,
    pub permissions: Option<PermissionsBody>,
    pub definition: Option<Definition>,
    pub relative_path: Option<String>,
    pub association: Option<BaseAssociation>,
    pub secondary_children: Option<Vec<ChildAssociationBody>>,
    pub targets: Option<Vec<AssociationBody>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Definition {
    pub properties: Vec<Property>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub default_value: Option<String>,
    pub data_type: Option<String>,
    pub is_multi_valued: Option<bool>,
    pub is_mandatory: Option<bool>,
    pub is_mandatory_enforced: Option<bool>,
    pub is_protected: Option<bool>,
    pub constraints: Option<Vec<Constraint>>,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Constraint {
    pub id: String,
    #[serde(alias = "type")]
    pub constraint_type: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub parameters: Option<HashMap<String, AnyObj>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeBodyUpdate {
    pub name: Option<String>,
    pub node_type: Option<String>,
    pub aspect_names: Option<Vec<String>>,
    pub properties: Option<HashMap<String, String>>,
    pub permissions: Option<PermissionsBody>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeBodyCopy {
    pub target_parent_id: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeBodyMove {
    pub target_parent_id: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeBodyLock {
    pub time_to_expire: Option<u64>,
    #[serde(alias = "type")]
    pub node_body_lock_type: Option<NodeBodyLockTypeEnum>,
    pub lifetime: Option<NodeBodyLockLifetimeEnum>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: String,
    pub name: String,
    pub node_type: String,
    pub is_folder: bool,
    pub is_file: bool,
    pub is_locked: bool,
    pub modified_at: String, // date-time
    pub modified_by_user: UserInfo,
    pub created_at: String, // date-time
    pub created_by_user: UserInfo,
    pub parent_id: Option<String>,
    pub is_link: Option<bool>,
    pub is_favorite: Option<bool>,
    pub content: Option<ContentInfo>,
    pub aspect_names: Option<Vec<String>>,
    pub properties: Option<AnyObj>,
    pub allowable_operations: Option<Vec<String>>,
    pub path: Option<PathInfo>,
    pub permissions: Option<PermissionsInfo>,
    pub definition: Option<Definition>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeEntry {
    pub entry: Node,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodePaging {
    pub list: Option<NodePagingList>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodePagingList {
    pub pagination: Option<Pagination>,
    pub entries: Option<Vec<NodeEntry>>,
    pub source: Option<Node>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeAssociationPaging {
    pub list: Option<NodeAssociationPagingList>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeAssociationPagingList {
    pub pagination: Option<Pagination>,
    pub entries: Option<Vec<NodeAssociationEntry>>,
    pub source: Option<Node>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeChildAssociationPaging {
    pub list: Option<NodeChildAssociationPagingList>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeChildAssociationPagingList {
    pub pagination: Option<Pagination>,
    pub entries: Option<Vec<NodeChildAssociationEntry>>,
    pub source: Option<Node>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeAssociationEntry {
    pub entry: NodeAssociation,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeAssociation {
    pub id: String,
    pub name: String,
    pub node_type: String,
    pub is_folder: bool,
    pub is_file: bool,
    pub is_locked: bool,
    pub modified_at: String, // date-time
    pub modified_by_user: UserInfo,
    pub created_at: String, // date-time
    pub created_by_user: UserInfo,
    pub parent_id: Option<String>,
    pub is_link: Option<bool>,
    pub is_favorite: Option<bool>,
    pub content: Option<ContentInfo>,
    pub aspect_names: Option<Vec<String>>,
    pub properties: Option<AnyObj>,
    pub allowable_operations: Option<Vec<String>>,
    pub path: Option<PathInfo>,
    pub permissions: Option<PermissionsInfo>,
    pub definition: Option<Definition>,
    pub association: Option<AssociationInfo>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeChildAssociationEntry {
    pub entry: NodeChildAssociation,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeChildAssociation {
    pub id: String,
    pub name: String,
    pub node_type: String,
    pub is_folder: bool,
    pub is_file: bool,
    pub is_locked: bool,
    pub modified_at: String, // date-time
    pub modified_by_user: UserInfo,
    pub created_at: String, // date-time
    pub created_by_user: UserInfo,
    pub parent_id: Option<String>,
    pub is_link: Option<bool>,
    pub is_favorite: Option<bool>,
    pub content: Option<ContentInfo>,
    pub aspect_names: Option<Vec<String>>,
    pub properties: Option<AnyObj>,
    pub allowable_operations: Option<Vec<String>>,
    pub path: Option<PathInfo>,
    pub permissions: Option<PermissionsInfo>,
    pub definition: Option<Definition>,
    pub association: Option<ChildAssociationInfo>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssociationEntry {
    pub entry: Association,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Association {
    pub target_id: String,
    pub assoc_type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildAssociationEntry {
    pub entry: ChildAssociation,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildAssociation {
    pub child_id: String,
    pub assoc_type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProbeEntry {
    pub entry: Probe,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Probe {
    pub message: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectAccessUrlBodyCreate {
    pub expires_at: Option<String>, // date-time
    pub valid_for: Option<u64>, // Length of time in seconds that the url is valid for.
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedLinkBodyCreate {
    pub node_id: String,
    pub expired_at: Option<String>, // date-time
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedLinkBodyEmail {
    pub client: Option<String>,
    pub message: Option<String>,
    pub locale: Option<String>,
    pub recipient_emails: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedLinkPaging {
    pub list: SharedLinkPagingList,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedLinkPagingList {
    pub pagination: Pagination,
    pub entries: Vec<SharedLinkEntry>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedLinkEntry {
    pub entry: SharedLink
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedLink {
    pub id: Option<String>,
    pub expired_at: Option<String>, // date-time
    pub node_id: Option<String>,
    pub name: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub modified_at: Option<String>, // date-time
    pub modified_by_user: Option<UserInfo>,
    pub shared_by_user: Option<UserInfo>,
    pub content: Option<ContentInfo>,
    pub allowable_operations: Option<Vec<String>>,
    pub allowable_operations_on_target: Option<Vec<String>>,
    pub is_favorite: Option<bool>,
    pub properties: Option<AnyObj>,
    pub aspect_names: Option<Vec<String>>,
    pub path: Option<PathInfo>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RenditionBodyCreate {
    pub id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RenditionEntry {
    pub entry: Rendition,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Rendition {
    pub id: Option<String>,
    pub content: Option<ContentInfo>,
    pub status: Option<RenditionStatusEnum>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RenditionPaging {
    pub list: Option<RenditionPagingList>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RenditionPagingList {
    pub pagination: Option<Pagination>,
    pub entries: Option<Vec<RenditionEntry>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeletedNodesPaging {
    pub list: Option<DeletedNodesPagingList>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeletedNodesPagingList {
    pub pagination: Option<Pagination>,
    pub entries: Option<Vec<DeletedNodeEntry>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeletedNodeEntry {
    pub entry: DeletedNode,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeletedNode {
    pub id: String,
    pub name: String,
    pub node_type: String,
    pub is_folder: bool,
    pub is_file: bool,
    pub is_locked: Option<bool>,
    pub modified_at: String, // date-time
    pub modified_by_user: UserInfo,
    pub created_at: String, // date-time
    pub created_by_user: UserInfo,
    pub parent_id: Option<String>,
    pub is_link: Option<bool>,
    pub is_favorite: Option<bool>,
    pub content: Option<ContentInfo>,
    pub aspect_names: Option<Vec<String>>,
    pub properties: Option<AnyObj>,
    pub allowable_operations: Option<Vec<String>>,
    pub path: Option<PathInfo>,
    pub permissions: Option<PermissionsInfo>,
    pub definition: Option<Definition>,
    pub archived_by_user: UserInfo,
    pub archived_at: String, // date-time
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeletedNodeRestore {
    pub target_parent_id: Option<String>,
    pub assoc_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RevertBody {
    pub major_version: Option<bool>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionPaging {
    pub list: Option<VersionPagingList>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionPagingList {
    pub pagination: Option<Pagination>,
    pub entries: Option<Vec<VersionEntry>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionEntry {
    pub entry: Option<Version>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub id: String,
    pub version_comment: Option<String>,
    pub name: String,
    pub node_type: String,
    pub is_folder: bool,
    pub is_file: bool,
    pub modified_at: String, // date-time
    pub modified_by_user: UserInfo,
    pub content: Option<ContentInfo>,
    pub aspect_names: Option<Vec<String>>,
    pub properties: Option<AnyObj>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadBodyCreate {
    pub node_ids: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadEntry {
    pub entry: Download,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Download {
    pub filed_added: Option<u64>,
    pub bytes_added: Option<u64>,
    pub id: Option<String>,
    pub total_files: Option<u64>,
    pub total_bytes: Option<u64>,
    pub status: Option<DownloadStatusEnum>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientBody {
    pub client: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordResetBody {
    pub password: String,
    pub id: String,
    pub key: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditAppPaging {
    pub list: Option<AuditAppPagingList>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditAppPagingList {
    pub pagination: Option<Pagination>,
    pub entries: Option<Vec<AuditAppEntry>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditAppEntry {
    pub entry: AuditApp,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditApp {
    pub id: String,
    pub name: Option<String>,
    pub is_enabled: Option<bool>, // default = true
    pub max_entry_id: Option<u64>,
    pub min_entry_id: Option<u64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditBodyUpdate {
    pub is_enabled: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditEntryPaging {
    pub list: Option<AuditEntryPagingList>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditEntryPagingList {
    pub pagination: Option<Pagination>,
    pub entries: Option<Vec<AuditEntryEntry>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditEntryEntry {
    pub entry: AuditEntry,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditEntry {
    pub id: String,
    pub audit_application_id: String,
    pub created_by_user: UserInfo,
    pub created_at: String, // date-time,
    pub values: Option<AnyObj>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionDefinitionList {
    pub list: Option<ActionDefinitionListList>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionDefinitionListList {
    pub pagination: Option<Pagination>,
    pub entries: Option<Vec<ActionDefinition>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionDefinitionEntry {
    pub entry: ActionDefinition,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionDefinition {
    pub id: String,
    pub name: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub applicable_types: Vec<String>,
    pub track_status: bool,
    pub parameter_definitions: Option<Vec<ActionParameterDefinition>>,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionParameterDefinition {
    pub name: Option<String>,
    #[serde(alias = "type")]
    pub action_parameter_type: Option<String>,
    pub multi_valued: Option<bool>,
    pub mandatory: Option<bool>,
    pub display_label: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionBodyExec {
    pub action_definition_id: String,
    pub target_id: Option<String>,
    pub params: Option<AnyObj>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionExecResultEntry {
    pub entry: ActionExecResult,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionExecResult {
    pub id: String,
}

// Authentication API
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketBodyCreate {
    pub user_id: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketEntry {
    pub entry: Ticket,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticket {
    pub id: Option<String>,
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidTicketEntry {
    pub entry: ValidTicket,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidTicket {
    pub id: Option<String>,
}
































