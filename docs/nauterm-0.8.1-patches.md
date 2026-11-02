# Nauterm foundation extensions on 0.8.1

This branch starts at `v0.8.1` and carries the existing Nauterm foundation
patches previously kept in its repository-local `gpui-base` copy. Product
components, themes and terminal rendering remain in Nauterm.

- Committed single-line ASCII digit filtering runs before edit history and caret
  updates, including full/accessibility replacements and IME commits. Exact
  programmatic values, undo, preedit and multiline editing retain their behavior.
- Auto-growing textareas expose their intrinsic minimum inside forms.
- Positioner can follow current trigger bounds during prepaint and keep its
  requested side while retaining viewport clamping.
- Markdown can opt into preserving soft line breaks in prose without changing
  fenced code or the default CommonMark behavior.
- Plain selectable text can join rich-text selection in paint order, while
  explicit document order remains the default.
- The branch retains Nauterm's two-logical-pixel input caret baseline, including
  its existing upstream snapping and blink behavior.

## Public API

All additions are in `gpui-base`:

- `input::InputFilter::{None, AsciiDigits}`: committed single-line edit policy.
- `InputBaseState::set_input_filter(&mut self, filter: InputFilter)`: opt into
  filtering without applying it to programmatic loads or multiline edits.
- `Positioner::tracked_trigger_bounds(self, bounds: Rc<Cell<Bounds<Pixels>>>) -> Self`:
  resolve the trigger geometry from the current frame.
- `Positioner::fixed_placement(self) -> Self`: preserve the requested side.
- `MarkdownExtensions::soft_line_breaks(self, preserve: bool) -> Self`: opt into
  chat prose line breaks.
- `SelectableText::in_paint_order(self) -> Self`: participate in displayed
  selection order alongside rich text.

The former caret snapping, focus blink and editor wheel-boundary patches are
already upstream in 0.8.1 and are not reintroduced. Strict Base library Clippy
and formatting checks cover the migration; actual UI verification remains with
the consuming application.
