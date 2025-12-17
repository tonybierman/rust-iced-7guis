# Flight Booker - 7GUIs Benchmark

## Overview
An Iced GUI app presenting a flight booking interface that demonstrates constraint modeling between and within widgets.

![Screenshot of Flight Booker - 7GUIs Benchmark](screenshot.png)

## Components
- **Combobox (C)**: Options "one-way flight" and "return flight"
- **Text field (T1)**: Start date
- **Text field (T2)**: Return date
- **Button (B)**: Submit booking

## Constraints

### Widget Dependencies
- T2 is enabled only when C = "return flight"
- B is disabled when:
  - Any enabled text field contains an ill-formatted date
  - C = "return flight" AND T2's date is before T1's date

### Widget States
- Ill-formatted date fields are colored red and disable B
- Initial state:
  - C = "one-way flight"
  - T1 and T2 = same arbitrary date
  - T2 disabled

## Behavior
When B is clicked, display a message confirming the selection:
- Example: "You have booked a one-way flight on 04.04.2014."

## Focus
Demonstrate clear, succinct, and explicit constraint modeling in the source code without excessive scaffolding.
