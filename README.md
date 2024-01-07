# Alfresco Rust SDK

This tool is currently in development to address my personal needs. While I intend to eventually make it cover all of Alfresco's REST API endpoints, I do not plan to put on any significant effort in thoroughly testing it to ensure full functionality—at least not yet. My testing will be limited to what is required for my current projects, focusing on ensuring it works for my specific use case. Consequently, it may not function for different use cases.

**This is my first time writing Rust, so it will 100% contain spaghetti code.**


## Compilation

If you haven't already, download rust:

```bash
curl -sSf https://static.rust-lang.org/rustup.sh | sh
```

```rust
// TODO: Document compilation steps
```

## API coverage

### [Authentication API](https://api-explorer.alfresco.com/api-explorer/?urls.primaryName=Authentication%20API#/)

- [x] Models
- [x] Authentication
  - [x] POST /tickets
  - [x] GET /tickets/-me-
  - [x] DELETE /tickets/-me-
- [ ] Documentation

### [Core API](https://api-explorer.alfresco.com/api-explorer/?urls.primaryName=Core%20API)

- [x] Models
- [x] Audit
  - [x]   GET /audit-applications
  - [x]   GET /audit-applications/{auditApplicationId}
  - [x]   PUT /audit-applications/{auditApplicationId}
  - [x]   GET /audit-applications/{auditApplicationId}/audit-entries
  - [x] DELETE /audit-applications/{auditApplicationId}/audit-entries
  - [x]   GET /audit-applications/{auditApplicationId}/audit-entries/{auditEntryId}
  - [x] DELETE /audit-applications/{auditApplicationId}/audit-entries/{auditEntryId}
  - [x]   GET /nodes/{nodeId}/audit-entries
- [ ] Actions
  - [ ]   GET /nodes/{nodeId}/action-definitions
  - [ ]   GET /action-definitions
  - [ ]   GET /action-definitions/{actionDefinitionId}
  - [ ] POST /action-executions
- [ ] Activities
  - [ ] GET /people/{personId}/activities
- [ ] Comments
  - [ ]   GET /nodes/{nodeId}/comments
  - [ ]   POST /nodes/{nodeId}/comments
  - [ ]   PUT /nodes/{nodeId}/comments/{commentId}
  - [ ] DELETE /nodes/{nodeId}/comments/{commentId}
- [ ] Downloads
  - [ ]   POST /downloads
  - [ ]   GET /downloads/{downloadId}
  - [ ] DELETE /downloads/{downloadId}
- [ ] Favorites
  - [ ]   GET /people/{personId}/favorites
  - [ ]   POST /people/{personId}/favorites
  - [ ]   GET /people/{personId}/favorites/{favoriteId}
  - [ ] DELETE /people/{personId}/favorites/{favoriteId}
- [ ] Networks
  - [ ] GET /people/{personId}/networks
  - [ ] GET /people/{personId}/networks/{networkId}
  - [ ] GET /networks/{networkId}
- [ ] Nodes
  - [ ]    GET /nodes/{nodeId}
  - [ ]    PUT /nodes/{nodeId}
  - [ ] DELETE /nodes/{nodeId}
  - [ ]    GET /nodes/{nodeId}/children
  - [ ]   POST /nodes/{nodeId}/children
  - [ ]   POST /nodes/{nodeId}/copy
  - [ ]   POST /nodes/{nodeId}/lock
  - [ ]   POST /nodes/{nodeId}/unlock
  - [ ]   POST /nodes/{nodeId}/move
  - [ ]    GET /nodes/{nodeId}/content
  - [ ]    PUT /nodes/{nodeId}/content
  - [ ]   POST /nodes/{nodeId}/secondary-children
  - [ ]    GET /nodes/{nodeId}/secondary-children
  - [ ] DELETE /nodes/{nodeId}/secondary-children/{childId}
  - [ ]    GET /nodes/{nodeId}/parents
  - [ ]   POST /nodes/{nodeId}/targets
  - [ ]    GET /nodes/{nodeId}/targets
  - [ ] DELETE /nodes/{nodeId}/targets/{targetId}
  - [ ]    GET /nodes/{nodeId}/sources
- [ ] People
  - [ ]   POST /people
  - [ ]    GET /people
  - [ ]    GET /people/{personId}
  - [ ]    PUT /people/{personId}
  - [ ]   POST /people/{personId}/request-password-reset
  - [ ]   POST /people/{personId}/reset-password
  - [ ]    GET /people/{personId}/avatar
  - [ ]    PUT /people/{personId}/avatar
  - [ ] DELETE /people/{personId}/avatar
- [ ] Groups
  - [ ]    GET /people/{personId}/groups
  - [ ]    GET /groups
  - [ ]   POST /groups
  - [ ]    GET /groups/{groupId}
  - [ ]    PUT /groups/{groupId}
  - [ ] DELETE /groups/{groupId}
  - [ ]   POST /groups/{groupId}/members
  - [ ]    GET /groups/{groupId}/members
  - [ ] DELETE /groups/{groupId}/members/{groupMemberId}
- [ ] Preferences
  - [ ] GET /people/{personId}/preferences
  - [ ] GET /people/{personId}/preferences/{preferenceName}
- [ ] Probes
  - [ ] GET /probes/{probeId}
- [ ] Queries
  - [ ] GET /queries/nodes
  - [ ] GET /queries/sites
  - [ ] GET /queries/people
- [ ] Ratings
  - [ ]    GET /nodes/{nodeId}/ratings
  - [ ]   POST /nodes/{nodeId}/ratings
  - [ ]    GET /nodes/{nodeId}/ratings/{ratingId}
  - [ ] DELETE /nodes/{nodeId}/ratings/{ratingId}
- [ ] Renditions
  - [ ] POST /nodes/{nodeId}/renditions
  - [ ]   GET /nodes/{nodeId}/renditions
  - [ ]   GET /nodes/{nodeId}/renditions/{renditionId}
  - [ ]   GET /nodes/{nodeId}/renditions/{renditionId}/content
- [ ] Shared Links
  - [ ]   POST /shared-links
  - [ ]    GET /shared-links
  - [ ]    GET /shared-links/{shareId}
  - [ ] DELETE /shared-links/{shareId}
  - [ ]    GET /shared-links/{shareId}/content
  - [ ]    GET /shared-links/{shareId}/renditions
  - [ ]    GET /shared-links/{shareId}/renditions/{renditionId}
  - [ ]    GET /shared-links/{shareId}/renditions/{renditionId}/content
  - [ ]   POST /shared-links/{shareId}/email
- [ ] Sites
  - [ ]    GET /people/{personId}/site-membership-requests
  - [ ]   POST /people/{personId}/site-membership-requests
  - [ ]    GET /people/{personId}/site-membership-requests/{siteId}
  - [ ]    PUT /people/{personId}/site-membership-requests/{siteId}
  - [ ] DELETE /people/{personId}/site-membership-requests/{siteId}
  - [ ]    GET /people/{personId}/sites
  - [ ]    GET /people/{personId}/sites/{siteId}
  - [ ] DELETE /people/{personId}/sites/{siteId}
  - [ ]    GET /sites
  - [ ]   POST /sites
  - [ ]    GET /sites/{siteId}
  - [ ]    PUT /sites/{siteId}
  - [ ] DELETE /sites/{siteId}
  - [ ]    GET /sites/{siteId}/containers
  - [ ]    GET /sites/{siteId}/containers/{containerId}
  - [ ]    GET /sites-membership-requests
  - [ ]   POST /sites/{siteId}/site-membership-requests/{inviteeId}/approve
  - [ ]   POST /sites/{siteId}/site-membership-requests/{inviteeId}/reject
  - [ ]    GET /sites/{siteId}/members
  - [ ]   POST /sites/{siteId}/members
  - [ ]    GET /sites/{siteId}/members/{personId}
  - [ ]    PUT /sites/{siteId}/members/{personId}
  - [ ] DELETE /sites/{siteId}/members/{personId}
  - [ ]    GET /sites/{siteId}/group-members
  - [ ]   POST /sites/{siteId}/group-members
  - [ ]    GET /sites/{siteId}/group-members/{groupId}
  - [ ]    PUT /sites/{siteId}/group-members/{groupId}
  - [ ] DELETE /sites/{siteId}/group-members/{groupId}
- [ ] Tags
  - [ ]    GET /nodes/{nodeId}/tags
  - [ ]   POST /nodes/{nodeId}/tags
  - [ ] DELETE /nodes/{nodeId}/tags/{tagId}
  - [ ]    GET /tags
  - [ ]    GET /tags/{tagId}
  - [ ]    PUT /tags/{tagId}
- [ ] Trashcan
  - [ ]    GET /deleted-nodes
  - [ ]    GET /deleted-nodes/{nodeId}
  - [ ] DELETE /deleted-nodes/{nodeId}
  - [ ]    GET /deleted-nodes/{nodeId}/content
  - [ ]   POST /deleted-nodes/{nodeId}/restore
  - [ ]    GET /deleted-nodes/{nodeId}/renditions
  - [ ]    GET /deleted-nodes/{nodeId}/renditions/{renditionId}
  - [ ]    GET /deleted-nodes/{nodeId}/renditions/{renditionId}/content
- [ ] Versions
  - [ ]    GET /nodes/{nodeId}/versions
  - [ ]    GET /nodes/{nodeId}/versions/{versionId}
  - [ ] DELETE /nodes/{nodeId}/versions/{versionId}
  - [ ]    GET /nodes/{nodeId}/versions/{versionId}/content
  - [ ]   POST /nodes/{nodeId}/versions/{versionId}/revert
  - [ ]   POST /nodes/{nodeId}/versions/{versionId}/renditions 
  - [ ]    GET /nodes/{nodeId}/versions/{versionId}/renditions
  - [ ]    GET /nodes/{nodeId}/versions/{versionId}/renditions/{renditionId}
  - [ ]    GET /nodes/{nodeId}/versions/{versionId}/renditions/{renditionId}/content

## .env configuration

The .env file has to be configured like this:

```dotenv
ALFRESCO_URL=https://alfresco-base-url.com
ALFRESCO_USERNAME=username
ALFRESCO_PASSWORD=password
```