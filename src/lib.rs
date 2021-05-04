#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use std::sync::Mutex;

use rocket_contrib::json::JsonValue;

pub mod server;

/// A global map that stores all of the `ComputeState`s for
/// each comparison computation.
pub struct ComputeMap {
    /// A monotonically-increasing counter for assigning new request IDs.
    uid: u32,
    /// HashMap mapping request IDs to `ComputeState` instances.
    mapping: HashMap<u32, ComputeState>,
}

/// The types of operations that the server should be able
/// to handle.
#[derive(Debug)]
pub enum OpType {
    /// Max comparison operation.
    Max,
    /// Min comparison operation.
    Min,
}

/// The state of a single compare operation.
#[derive(Debug)]
pub struct ComputeState {
    /// The type of the comparison operation, either Max or Min.
    op: OpType,
    /// The index of the left element as we traverse the list.
    left: usize,
    /// The index of the right element as we traverse the list.
    right: usize,
}

/// Represents the possible fields that may be passed in from a
/// JSON request.
#[derive(Debug, Deserialize)]
pub struct Request {
    /// The type of the Request.
    ty: String,
    /// The length that is present when the Request type is "compute".
    length: Option<usize>,
    /// The request ID that is present when the Request type is "comp_result".
    request_id: Option<u32>,
    /// The answer that is present when the Request type is "comp_result".
    answer: Option<bool>,
}

/// Constructs a "done" JSON Response from an input `result` usize.
pub fn done_response(result: usize) -> JsonValue {
    json!({ "ty": "done", "result": result })
}

/// Constructs a "compare" JSON Response from `left`, `right`, and `id`
/// parameters.
pub fn compare_response(left: usize, right: usize, id: u32) -> JsonValue {
    json!({
        "ty": "compare",
        "left": left,
        "right": right,
        "request_id": id,
    })
}
