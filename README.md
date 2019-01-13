# stars-reborn_game-engine #

This is the game engine for stars-reborn.  It provides the backend services
needed to operate the game and provide game-specific logic and information.

This includes:

  * Game and universe generation
  * Turn generation and retrieval
  * Turn action submission
  * Race validation
  * Game information queries
  * Historical game information generation and retrieval

The idea is that anything specific to the logic of stars-reborn itself lives in
the game engine and there is a separate view frontend that provides the player
with the information and controls to generate the actions needed for the turn.

Any authentication, record of games, races associated to the player, etc. is
done in the frontend layer.

Performing actions/queries to the service is done via HTTP on the port the
service is running on.  The requests and responses to the service must conform 
to the JSON schemas in the schemas folder, otherwise they are rejected.

# Building #

Use cargo to build the project.

    cargo build

or:

    cargo build --release

# Running #

You can provide environment parameters with a .env file or providing
environment variables on the command line.

Examples:

    RUST_BACKTRACE=1 SERVER_ADDR="127.0.0.1:8200" RUST_LOG=debug target/debug/stars-reborn_game-engine

# Programming Language Choice #

The engine started in Python.  I have moved it to Rust:
  * To have more control over types
  * To have a compiled binary for clean deployment to the production system
  * To learn Rust
  
Because I am learning, the code may need... help.  And I welcome suggestions!

# Thanks! #

  * Thank you to all of the authors and contributors to the Rust crates that I am
using.  <https://crates.io> is a great ecosystem.
  * The design of this service was influenced by: The Twelve-Factor App <https://12factor.net/>
  * I used this example for hyper for the service as a starting point: <http://www.goldsborough.me/rust/web/tutorial/2018/01/20/17-01-11-writing_a_microservice_in_rust>
