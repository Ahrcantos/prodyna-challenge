# Prodyna Challenge

## Setup

To run this project clone the repository and execute `cargo run`.
For this to work you will need to have rust and `cargo` installed.
If you do not have it installed already you can follow the
instructions here: <https://www.rust-lang.org/learn/get-started>.

Depending on your development environment you might also
need additional dependencies to compile `raylib`.
For this checkout the "Building from source" section here: <https://crates.io/crates/raylib>.


## Functionality

Renders static candidate information.
Displays a candidates history of experiences, skill level for indivudual skills and
additional notes about the candidate. For these information different sections
can be opened and closed by clicking on them.


## Design for loading candidate data

The rendering of the candidate's information is already generalized to
display the information contained in the `Candidate` struct.
To load data from an additional data source the data will therefor have to be
read into this struct.

When loading the data from a json file one approach would be to
use the `serde` and `serde_json` crates to make the struct deserializable
into json. The contents of a json file could then be directly deserialized
into the struct.

The approach for loading the data from an external database would depend on
the database technology in question.

If the data can be fetched from a singular place (e.g. as document in a NoSql)
database then the library used to read from the database might allow directly
deserializing into the struct provided that `serde::Deserialize` has been implemented
for the `Candidate` struct.

If the data needs to first be collected from multiple sources (e.g. multiple tables in a relational database) then additional functionality is needed to collect all needed data and to combine it into the `Candidate` struct.

Regardless of datasource once the data is read into the `Candidate` struct
the rendering process can begin.


## Current Limitations

Currently the application still has some limitations when it comes to
displaying a candidates data in a generalized way.

First the width of an element is always static meaning that once
dynamic data can be read into the system the layouting of certain elements
will break by rendering text/shapes across certain boundaries.

Another limitation with the current element structure is that events (like clicking)
are only evaluated for top-level elements. This would for example prevent the
possibility of having an expandable section contain another expandable section.
