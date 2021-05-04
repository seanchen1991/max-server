use super::*;

use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

/// Handles POST requests to the "/" route with a `Request` payload.
#[post("/", format = "json", data = "<request>")]
fn handle_post(request: Json<Request>, map: State<Mutex<ComputeMap>>) -> JsonValue {
    let req = request.0;

    println!("{:?}", req);

    generate_response(req, map)
}

/// Generates a JSON response according to the type of `Request`.
/// Receives a `Request` as well as a hashmap which stores the
/// states of any in-progress comparison operations.
fn generate_response(req: Request, map: State<Mutex<ComputeMap>>) -> JsonValue {
    let mut compute_map = map.lock().expect("Map lock");
    let req_str = req.ty.as_str();

    match req_str {
        "compute_max" | "compute_min" => {
            // initialize a new `ComputeState` to keep track of
            // the computation and insert it into `ComputeMap`
            let id = compute_map.uid + 1;
            let left = 0;
            let right = req.length.unwrap() - 1;
            let op = if req_str == "compute_max" {
                OpType::Max
            } else {
                OpType::Min
            };

            let compute_state = ComputeState { op, left, right };

            // handle case when list is of length 1
            if left == right {
                done_response(0)
            } else {
                // send the first "compare" response with the initial
                // set of indices
                compute_map.mapping.insert(id, compute_state);
                compute_map.uid = id;
                compare_response(left, right, id)
            }
        }
        "comp_result" => {
            let id = req.request_id.unwrap();
            let answer = req.answer.unwrap();

            // check to ensure that the comparison operation is a
            // pre-existing one
            match compute_map.mapping.get_mut(&id) {
                Some(compute_state) => {
                    match compute_state.op {
                        OpType::Max => {
                            if answer {
                                // increment the left index
                                compute_state.left += 1;
                            } else {
                                // decrement the right index
                                compute_state.right -= 1;
                            }

                            // check if the computation has reached the end of the list
                            // otherwise, continue the computation by sending a "compare"
                            // response with the next set of indices
                            if compute_state.left == compute_state.right {
                                done_response(compute_state.left)
                            } else {
                                compare_response(compute_state.left, compute_state.right, id)
                            }
                        }
                        OpType::Min => {
                            if answer {
                                // decrement the right index
                                compute_state.right -= 1;
                            } else {
                                // increment the left index
                                compute_state.left += 1;
                            }

                            if compute_state.left == compute_state.right {
                                done_response(compute_state.left)
                            } else {
                                compare_response(compute_state.left, compute_state.right, id)
                            }
                        }
                    }
                }
                None => json!({
                    "status": "error",
                    "reason": "Attempted to fetch a non-existent comparison computation.",
                }),
            }
        }
        _ => json!({
            "status": "error",
            "reason": "Unrecognized request type.",
        }),
    }
}

/// Starts up the Rocket server, registering the POST route
/// as well as the `ComputeState` hashmap.
pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![handle_post])
        .manage(Mutex::new(ComputeMap {
            uid: 0,
            mapping: HashMap::<u32, ComputeState>::new(),
        }))
}
