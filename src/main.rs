#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use std::sync::Mutex;

use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

/// A global map that stores all of the `ComputeState`s for
/// each comparison computation.
type ComputeMap = Mutex<HashMap<u32, ComputeState>>;

#[derive(Debug)]
enum OpType {
    Max,
    Min,
}

/// The state of a single compare operation.
struct ComputeState {
    /// The type of the comparison operation, either Max or Min
    op: OpType,
    /// The index of the left element as we traverse the list.
    /// When left and right values are equal, left takes priority.
    left: usize,
    /// The index of the right element as we traverse the list.
    right: usize,
}

// This would definitely work better as an enum whose
// variants are the two different types of requests we might
// get from the client. However, this doesn't play nicely
// with Rocket's JSON parsing and would require some
// custom parsing logic.
// More generally, it doesn't help that all of the requests
// get sent to a single POST endpoint instead of being routed
// to multiple POST endpoints.
#[derive(Debug, Serialize, Deserialize)]
struct Request {
    ty: String,
    length: Option<usize>,
    request_id: Option<u32>,
    answer: Option<bool>,
}

#[post("/", format = "json", data = "<message>")]
fn handle_post(message: Json<Request>, map: State<ComputeMap>) -> JsonValue {
    let req = message.0;

    println!("{:?}", req);

    generate_response(req, map)
}

fn generate_response(req: Request, map: State<ComputeMap>) -> JsonValue {
    let done_response = |result: usize| json!({"ty": "done", "result": result});
    let compare_response = |left: usize, right: usize, id: u32| json!({
        "ty": "compare",
        "left": left,
        "right": right,
        "request_id": id,
    });

    let mut compute_map = map.lock().expect("Map lock");

    match req.ty.as_str() {
        "compute_max" => {
            // initialize a new `ComputeState` to keep track of
            // the computation and insert it into `ComputeMap`
            let id = compute_map.len() as u32 + 1;
            let left = 0;
            let right = req.length.unwrap() - 1;

            let compute_state = ComputeState {
                op: OpType::Max,
                left,
                right,
            };

            compute_map.insert(id, compute_state);

            // handle case when list is of length 1
            if left == right {
                done_response(0)
            } else {
                // send the first "compare" response with the initial
                // set of indices
                compare_response(left, right, id)
            }
        }
        "compute_min" => {
            let id = compute_map.len() as u32 + 1;
            let left = 0;
            let right = req.length.unwrap() - 1;

            let compute_state = ComputeState {
                op: OpType::Min,
                left,
                right,
            };

            compute_map.insert(id, compute_state);

            if left == right {
                done_response(0)
            } else {
                compare_response(left, right, id)
            }
        }
        "comp_result" => {
            let id = req.request_id.unwrap();
            let answer = req.answer.unwrap();

            // check to ensure that the comparison operation exists
            match compute_map.get_mut(&id) {
                Some(compute_map) => {
                    match compute_map.op {
                        OpType::Max => {
                            if answer {
                                // increment the left index
                                compute_map.left += 1;
                            } else {
                                // decrement the right index
                                compute_map.right -= 1;
                            }

                            // check if the computation has reached the end of the list
                            // otherwise, continue the computation by sending a "compare"
                            // response with the next set of indices
                            if compute_map.left == compute_map.right {
                                done_response(compute_map.left)
                            } else {
                                compare_response(compute_map.left, compute_map.right, id)
                            }
                        }
                        OpType::Min => {
                            if answer {
                                // decrement the right index
                                compute_map.right -= 1;
                            } else {
                                // increment the left index
                                compute_map.left += 1;
                            }

                            if compute_map.left == compute_map.right {
                                done_response(compute_map.left)
                            } else {
                                compare_response(compute_map.left, compute_map.right, id)
                            }
                        }
                    }
                }
                None => json!({
                    "status": "error",
                    "reason": "Attempted to fetch a non-existent comparison operation.",
                }),
            }
        }
        _ => json!({
            "status": "error",
            "reason": "Unrecognized request type.",
        }),
    }
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![handle_post])
        .manage(Mutex::new(HashMap::<u32, ComputeState>::new()))
}

fn main() {
    rocket().launch();
}
