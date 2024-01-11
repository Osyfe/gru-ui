An embeddable and lightweight ui library intended for usage in games written in Rust. It draws inspiration from three Rust ui libraries and combines their design strengths:

- "egui" for its embeddable nature: The library consumes events as input and produces renderable geometry as output, leaving complete control over its execution and integration into the application to the user.
- "iced" for the event system: Widgets can be configured to produce ui events (e.g. button presses) that can be reacted to anywhere and anytime in the code, decoupling ui construction and its effects.
- "druid" for the lensing mechanic: The ui does not own any data, instead each widget knows how to navigate to its relevant data and operates directly on that, no synchronisation is needed.

The behaviour of the library is very simple and straight forward. As a trade off the user is required to trigger some necessary updates manually, e.g. when some widget needs to recompute itself due to changed data.
