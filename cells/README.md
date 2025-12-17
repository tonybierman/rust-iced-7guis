# Cells - 7GUIs Benchmark

## Overview

A simple but usable Iced GUI spreadsheet application that demonstrates change propagation and widget customization in a GUI context.

## Requirements

### Grid Specification
- Scrollable spreadsheet grid
- Rows: 0-99 (100 rows)
- Columns: A-Z (26 columns)

### Cell Editing
- Double-click a cell to edit its formula
- After editing completes:
  - Parse the formula
  - Evaluate the expression
  - Display the updated value in the cell

### Change Propagation
- When a cell's value changes, automatically reevaluate all dependent cells
- Continue propagation until no cell values change
- **Optimization**: Only recompute cells that depend on changed values, not all cells

### Widget Customization
- **Do not** use existing spreadsheet widgets
- Customize a similar general-purpose widget to create a reusable spreadsheet component
- Separate domain-specific logic from GUI-specific code

## Challenges

1. **Change Propagation**: Implement intelligent dependency tracking and selective cell updates
2. **Widget Customization**: Adapt a generic widget into a specialized, reusable spreadsheet component
3. **Scalability**: Demonstrate that the approach scales to a more involved GUI application

## Success Criteria

- Change propagation requires minimal effort and performs efficiently
- Widget customization is straightforward and well-abstracted
- Clear separation between domain logic (formula parsing/evaluation) and GUI code
- Resulting spreadsheet widget is reusable in other contexts

## About 7GUIs: A GUI Programming Benchmark

There are countless GUI toolkits in different languages and with diverse approaches to GUI development. Yet, diligent comparisons between them are rare. Whereas in a traditional benchmark competing implementations are compared in terms of their resource consumption, here implementations are compared in terms of their notation. To that end, [7GUIs](https://eugenkiss.github.io/7guis/) defines seven tasks that represent typical challenges in GUI programming. In addition, 7GUIs provides a recommended set of evaluation dimensions.
