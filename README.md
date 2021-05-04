# Oso Work Sample

## How to Run

First, build and run the server by executing `cargo run` in the project's root. Then run the client with `python client.py` to have it start sending requests to the server.

## Approach

This `max-server` implementation is a [Rocket](https://rocket.rs) webserver that exposes a single POST endpoint where all client requests are received.

When the server receives either a `compute_max` or `compute_min` request, a new `ComputeState` instance is initialized in order to keep track of the ongoing state of the comparison operation as the client and server trade messages back and forth. Each `ComputeState` instance is stored in a global `ComputeMap` along with the computation's `request_id`. This global map is handled as a piece of [State](https://api.rocket.rs/v0.4/rocket/struct.State.html) by Rocket.

When the server receives a `comp_result` request with the result of comparing two list values, it fetches the `ComputeState` using the `request_id` included in the request and updates the computation's left or right index according to the type of the operation and the comparison result. The max (or min) value is found when the left and right indices meet; that's the server's cue to send the "done" response.

## Changes Made to the Client

Notable changes made to `client.py`: 
 - Each "type" field on each message has been renamed to "ty" (since "type" is a keyword in Rust).
 - A few more assertions have been added to test additional edge cases.

## Missing Features

This implementation is currently missing, among other things:
 - Unit tests
 - Proper error handling
