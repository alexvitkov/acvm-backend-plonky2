# acvm-backend-plonky2
WIP noir backend for plonky2.


## Copy Pasta from Noir
acvm-repo copy-pasted from the Noir repo. The crates.io packages for acvm are way out of date.

In the root Cargo.toml, workspace.dependencies are copy-pasted from Noir Cargo.toml 

## Some modifications to Noir needed!!!
There's currently some barretenberg-specific assumptions in Noir.
To get this to run, find the following code in Noir and comment it out:

```rust
//proof_system.rs:72:
let proof = bb_abstraction_leaks::remove_public_inputs(
    circuit.public_inputs().0.len(),
    &proof_with_public_inputs,
);
```