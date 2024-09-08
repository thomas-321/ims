pub mod article;
pub mod role;
pub mod user;

// Now in each `mod.rs`, only the model.rs is included.
// For example, in api/api-models/src/article/mod.rs:
// pub mod model;