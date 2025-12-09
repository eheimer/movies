# Implementation Plan

- [x] 1. Update Edit mode header logic in draw_header function
  - Modify the `Mode::Edit` match arm in `draw_header` function (src/display.rs, ~line 230)
  - Change the conditional logic to append ", [F2] save" when `is_dirty` is true
  - Current code: `if is_dirty { instruction.push_str(", [F2] save changes"); }`
  - Update to: `if is_dirty { instruction.push_str(", [F2] save"); }`
  - _Requirements: 1.1, 1.2, 1.3_
