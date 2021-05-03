use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

use super::*;

/// Handles POST requests to the "/" route with a `Request` payload.
#[post("/", format = "json", data = "<request>")]
fn handle_post(request: Json<Request>, map: State<ComputeMap>) -> JsonValue {
    let req = request.0;

    println!("{:?}", req);

    generate_response(req, map)
}

/// Generates a JSON response according to the type of `Request`.
/// Receives a `Request` as well as a hashmap which stores the
/// states of any in-progress comparison operations.
fn generate_response(req: Request, map: State<ComputeMap>) -> JsonValue {
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

/// Starts up the Rocket runtime, registering the POST route
/// as well as the `ComputeState` hashmap.
pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![handle_post])
        .manage(Mutex::new(HashMap::<u32, ComputeState>::new()))
}

