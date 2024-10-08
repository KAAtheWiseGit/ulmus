# Ulmus

Ulmus is a TUI framework, inspired by [Charm's][charm] [Bubble Tea][bt].
Like Bubble Tea, it uses the [Elm architecture][ea].


## Architecture

To use Ulmus a user needs to create a type which implements the [`Model`
trait][Model].  It will be called using a [`Program`], via the [`run`]
method.

`Program` will repeatedly call the model, passing crossterm messages via
model's [`update` method][update].  After each update the [`view`]
method will be called, and the results will be draw on the terminal.

For performing I/O see [`Command`] and [`Subroutine`].


## [Examples]

- [Inline picker][examples-ip] — a TODO list picker which uses the
  inline `Program` mode.


## Comparison

### Bubble Tea

Ulmus interface is based on that of Bubble Tea, so they are very
similar.  The model is mutated in-place in `update`, since it can be
ensured that it doesn't change when `view` is called.


### [Ratatui]

- Ratatui is a library.  The application calls Ratatui's functions.

- Ulmus is a framework.  Ulmus calls application's code.

Additionally, Ulmus does less than Ratatui.  It doesn't support multiple
terminal backends, styling (the application has to do it itself, for
example using [`colored`]), or complex widgets (Ratatui renders each
cell individually, Ulmus renders one string).

Finally, Ratatui is a mature project with a [large ecosystem][re] and
Ulmus is my small weekend project.


[charm]: https://charm.sh/
[bt]: https://github.com/charmbracelet/bubbletea
[ea]: https://guide.elm-lang.org/architecture/
[Model]: https://docs.rs/ulmus/latest/ulmus/trait.Model.html
[`Program`]: https://docs.rs/ulmus/latest/ulmus/struct.Program.html
[`run`]: https://docs.rs/ulmus/latest/ulmus/struct.Program.html#method.run
[update]: https://docs.rs/ulmus/latest/ulmus/trait.Model.html#tymethod.update
[`view`]: https://docs.rs/ulmus/latest/ulmus/trait.Model.html#tymethod.view
[`Command`]: https://docs.rs/ulmus/latest/ulmus/enum.Command.html
[`Subroutine`]: https://docs.rs/ulmus/latest/ulmus/type.Subroutine.html
[Examples]: https://codeberg.org/kaathewise/ulmus/src/branch/trunk/examples
[examples-ip]: https://codeberg.org/kaathewise/ulmus/src/branch/trunk/examples/inline-picker.rs
[Ratatui]: https://ratatui.rs/
[`colored`]: https://github.com/colored-rs/colored
[re]: https://ratatui.rs/showcase/third-party-widgets/
