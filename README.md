# Oso Work Sample

## Approach

This `max-server` implementation is a Rocket webserver that exposes a single POST endpoint where all client requests are received.

When the server receives either a `compute_max` or `compute_min` request, a new `ComputeState` instance is initialized in order to keep track of the ongoing state of the comparison operation as the client and server trade messages back and forth. Each `ComputeState` instance is stored in a global `ComputeMap` along with the computation's `request_id`. This global map is handled as a piece of [State](https://api.rocket.rs/v0.4/rocket/struct.State.html) by Rocket.

When the server receives a `comp_result` request with the result of comparing two list values, it fetches the `ComputeState` using the `request_id` included in the request and updates the computation's left or right index according to the type of the operation and the comparison result. The max (or min) value is found when the left and right indices meet; that's the server's cue to send the "done" response.

## How to Run

First, build and run the server by executing `cargo run` in the project's root. Then run the client with `python client.py` to have it start sending requests to the server.

## Changes Made to the Client

The only notable changes made to `client.py` file is renaming each message's "type" field to "ty" (since "type" is a keyword in Rust). Additionally, a few more assertions were added as end-to-end tests for the server.
