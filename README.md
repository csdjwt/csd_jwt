#### CSD-JWT 
This repository contains the implementation associated with the paper "Compact and Selective Disclosure for Verifiable Credentials". 
In particular, it provides a proof of concept implementation of CSD-JWT and an extensive comparison against SD-JWT, BBS+, and Merkle Trees.

The key performance metrics included in the benchmark against the amount of claims included in the Verifiable Credential
are:

-  Verifiable Credential size.
-  Verifiable Credential generation latency.
-  Verifiable Credential verification latency.

Conversely, the key performance metrics included in the benchmark against the amount of disclosed claims included in
the Verifiable Presentation:

-  Verifiable Presentation size.
-  Verifiable Presentation generation latency.
-  Verifiable Presentation verification latency.

To run all the available tests in the library, execute in the project directory `cargo test`.
To run the benchmark, execute in the project directory `cargo run -r`.

External libraries 

- [Openssl](httpsopenssl-library.org) 
