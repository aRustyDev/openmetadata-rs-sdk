# TODOs

## Macros

Not sure how I want to do this, but it would be really nice to make these things easily implementable via macros

## Schemas
- [ ] Settings
- [ ] Configuration
- [ ] Security
- [ ] Auth
- [ ] Entity
- [ ] Tests
- [ ] MetadataIngestion
- [ ] System
- [ ] Type
- [ ] Governance
- [ ] Api
- [ ] DataInsight
- [ ] Jobs
- [ ] Monitoring
- [ ] Events
- [ ] Email
- [ ] Analytics

## [API](https://docs.open-metadata.org/latest/main-concepts/metadata-standard/apis) lib

- [ ] Data Asset: support operations related to data asset entities
  - [ ] databases
  - [ ] tables
  - [ ] metrics
  - [ ] dashboards
  - [ ] reports
  - [ ] pipelines
  - [ ] topics
- [ ] Service: support operations related to services from which metadata is collected
  - [ ] databaseService
  - [ ] dashboardService
  - [ ] messagingService
- [ ] Teams & Users:
  - [ ] teams
  - [ ] users
- [ ] Search & Suggest:
  - [ ] search
    - [ ] query
    - [ ] suggest
- [ ] Other
  - [ ] tags
  - [ ] feeds
  - [ ] usage

## Roadmap

- [ ] Auto-generated models from JSON schemas
- [ ] Builder patterns for all entities
- [ ] Pagination support
- [ ] Async/streaming for large result sets
- [ ] Entity-specific helper methods
- [ ] Integration tests
- [ ] Complete entity coverage
- [ ] Lineage operations
- [ ] Search functionality
- [ ] Bulk operations
- [ ] Caching layer

Implementation Approach

Start with manual struct definitions for core entities
Use reqwest for HTTP client with tokio for async
Use serde and serde_json for serialization
Use thiserror for error handling
Consider using derive_builder crate for builder patterns
