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

## Codegen for `x as u1`
```rust
fn main(x : Field)  {
    assert(x as u1 == 1);
}
```
This emits ~500 ACIR instructions, most of them regular constraints, but also two black-box function calls to `ToLeRadix` as well as a Brillig chain. Ideally for this code, we'd just like to emit `builder.not(builder.is_equal(x, 1))`.
```bash
DIR::TORADIX (_x2, [_8..._260] )

# For N in 8..256, wN*wN - wN = 0: 
EXPR [ (1, _8, _8) (-1, _8) 0 ]
EXPR [ (1, _9, _9) (-1, _9) 0 ]
...
EXPR [ (1, _260, _260) (-1, _260) 0 ]

EXPR [ (1, _2) (-1, _8) (-1, _263) 0 ]
EXPR [ (-2, _9) (-4, _10) (-1, _264) 0 ]
EXPR [ (-8, _11) (-2⁴, _12) (-1, _265) 0 ]
# ... 250 more constraints ...
EXPR [ (-2²⁵¹, _259) (-2²⁵², _260) (-1, _389) 0 ]

EXPR [ (1, _263) (1, _264) (-1, _390) 0 ]
EXPR [ (1, _265) (1, _266) (-1, _391) 0 ]
# ... 250 more constraints...
EXPR [ (1, _511) (1, _512) (1, _513) 0 ]

EXPR [ (1, _2) (-1, _4) 10944121435919637611123202872628637544274182200208017171849102093287904247809 ]
BRILLIG: {
    inputs: [Single(Expression { mul_terms: [], linear_combinations: [(1, Witness(4))], q_c: 0 })]
    outputs: [Simple(Witness(5))]
    [
        JumpIfNot { condition: RegisterIndex(0), location: 3 }, 
        Const { destination: RegisterIndex(1), value: Value { inner: 1 } }, 
        BinaryFieldOp { destination: RegisterIndex(0), op: Div, lhs: RegisterIndex(1), rhs: RegisterIndex(0) }, 
        Stop
    ]
}

EXPR [ (1, _4, _5) (1, _6) -1 ]
EXPR [ (1, _4, _6) 0 ]
EXPR [ (-1, _3, _6) (-1, _7) 0 ]
DIR::TORADIX (_x7, [_261..._262] )
EXPR [ (1, _261, _261) (-1, _261) 0 ]
EXPR [ (1, _262, _262) (-1, _262) 0 ]
EXPR [ (1, _7) (-1, _261) (-2, _262) 0 ]
EXPR [ (1, _3) -1 ]
```

## Codegen for !=

```rust
fn main(x: Field, y: pub Field) {
    assert(x != y);
}
```
The circuit above emits the following ACIR, which includes a Brillig division.
```bash
EXPR [ (-1, _1) (1, _2) (-1, _3) 0 ]

BRILLIG: {
    inputs: [Single(Expression { mul_terms: [], linear_combinations: [(1, Witness(3))], q_c: 0 })]
    outputs: [Simple(Witness(4))]
    [
        JumpIfNot { condition: RegisterIndex(0), location: 3 },
        Const { destination: RegisterIndex(1), value: Value { inner: 1 } },
        BinaryFieldOp {
            destination: RegisterIndex(0),
            op: Div, lhs: RegisterIndex(1),
            rhs: RegisterIndex(0)
        },
        Stop
    ]
}

EXPR [ (1, _3, _4) (1, _5) -1 ]
EXPR [ (1, _3, _5) 0 ]
EXPR [ (-1, _5) 0 ]
```

If the circuit instead asserted `x == y`, we only get one constraint as expected:
```
EXPR [ (1, _1) (-1, _2) 0 ]
```

## Goldilocks curve not supported
ACVM supports two curves:
```
pub enum FieldOptions {
    BN254,
    BLS12_381,
}
```
Implementations used in ACVM are from https://github.com/arkworks-rs/curves are used. The repo doesn't have an implementation for Goldilocks either. 
