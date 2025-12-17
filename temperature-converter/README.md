# Temperature Converter

## Overview
Build a temperature conversion application with bidirectional data flow between Celsius and Fahrenheit text fields.

## Requirements

### UI Components
- Two text input fields labeled TC (Celsius) and TF (Fahrenheit)
- Both fields initially empty

### Behavior
- When user enters a numerical value in TC, TF updates automatically
- When user enters a numerical value in TF, TC updates automatically
- Non-numerical input in either field does not trigger updates
- Conversion happens in real-time as user types

### Conversion Formulas
- Celsius to Fahrenheit: `F = C * (9/5) + 32`
- Fahrenheit to Celsius: `C = (F - 32) * (5/9)`

## Challenges
- **Bidirectional data flow**: Changes in either field must update the other
- **User-provided text input**: Must validate numerical input and handle invalid entries gracefully

## Success Criteria
A good solution will:
- Make the bidirectional dependency clear and explicit
- Minimize boilerplate code
- Handle input validation elegantly
