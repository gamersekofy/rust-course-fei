//! Run this file with `cargo test --test 03_state_transition_struct`.

//! This is a modified variant of the `03_state_transition` test from your home assignment.
//! Try to implement it using structs (without enums), and then later implement it using
//! enums in the assignment, and compare both approaches.

// TODO: Implement the `pc_transition` function.
// A computer can be in three states (off, running or sleeping).
// It can receive four events (turn on, turn off, pass some amount of time and mouse move).
//
// When the PC is running or sleeping, it remembers the time since it was started (`uptime`).
// When the PC is running, it also remembers `idle_time` (time since last mouse move).
// When the PC is sleeping, it also remembers `sleep_time` (time since going to sleep).
//
// Here are the rules that the computer should abide by:
// 1) When `TurnOn` happens, if the PC is off, it switches to `Running`. Otherwise nothing happens.
// 2) When `TurnOff` happens, the PC switches to `Off`.
// 3) When `MoveMouse` happens:
//   - if the PC is sleeping, the PC switches to `Running`.
//   - if the PC is running, it resets its `idle_time` to zero.
// 4) When `PassTime(time)` happens, and the PC is on, it increments its `uptime` by `time`. Then:
//   - If the PC is running and its `idle_time` is larger than 1000, it switches to `Sleeping`.
//   - If the PC is sleeping and its `sleep_time` is larger than 500, it switches to `Off`.

// Represents the state of the computer using Option fields.
// - If `uptime` is None, the computer is Off. `idle_time` and `sleep_time` must also be None.
// - If `uptime` is Some, the computer is On (Running or Sleeping).
//   - If `idle_time` is Some, it's Running. `sleep_time` must be None.
//   - If `sleep_time` is Some, it's Sleeping. `idle_time` must be None.
struct ComputerState {
    // Time since the computer was turned on. None if the computer is off.
    uptime: Option<u32>,
    // Time since the last mouse move. Only Some if the computer is Running.
    idle_time: Option<u32>,
    // Time since the computer went to sleep. Only Some if the computer is Sleeping.
    sleep_time: Option<u32>,
}

impl ComputerState {
    // Returns a computer that is turned off
    fn new_off() -> Self {
        Self {
            uptime: None,
            idle_time: None,
            sleep_time: None,
        }
    }

    // Returns a computer that is turned on (starts in Running state)
    fn new_on() -> Self {
        Self {
            uptime: Some(0),      // Uptime starts at 0
            idle_time: Some(0),   // Idle time starts at 0 (implicitly Running)
            sleep_time: None,     // Not sleeping initially
        }
    }

    // Checks if the computer is On (either Running or Sleeping)
    fn is_on(&self) -> bool {
        self.uptime.is_some()
    }

    // Checks if the computer is Sleeping
    fn is_sleeping(&self) -> bool {
        // If sleep_time is Some, it must be sleeping (based on our state representation)
        self.sleep_time.is_some()
    }

    // Returns the uptime if the computer is On, otherwise 0.
    fn uptime(&self) -> u32 {
        self.uptime.unwrap_or(0)
    }

    // Returns the idle time if the computer is Running, otherwise 0.
    fn idle_time(&self) -> u32 {
        self.idle_time.unwrap_or(0)
    }

    // Returns the sleep time if the computer is Sleeping, otherwise 0.
    fn sleep_time(&self) -> u32 {
        self.sleep_time.unwrap_or(0)
    }

    // Helper function to check if the computer is currently Running
    fn is_running(&self) -> bool {
        self.idle_time.is_some()
    }
}

// Make Event derivable for tests if needed, ensure it matches the test crate's definition
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Event {
    TurnOn,
    TurnOff,
    PassTime(u32),
    MoveMouse,
}


// Processes an event and returns the new state of the computer.
fn pc_transition(mut computer: ComputerState, event: Event) -> ComputerState {
    match event {
        Event::TurnOn => {
            // Rule 1: If off, switch to Running. Otherwise, no change.
            if !computer.is_on() {
                computer = ComputerState::new_on(); // Reinitialize to the base "On" state
            }
            // If already on, do nothing. State and times remain unchanged.
        }
        Event::TurnOff => {
            // Rule 2: Switch to Off, regardless of the current state.
            computer = ComputerState::new_off();
        }
        Event::MoveMouse => {
            // Rule 3:
            if computer.is_sleeping() {
                // Rule 3a: If sleeping, switch to Running.
                // Uptime is preserved. Idle time resets. Sleep time is cleared.
                let current_uptime = computer.uptime.expect("Sleeping state must have uptime");
                // No need to re-assign uptime, it's preserved.
                computer.idle_time = Some(0); // Reset idle time, indicating Running state
                computer.sleep_time = None;   // No longer sleeping
            } else if computer.is_running() {
                // Rule 3b: If running, reset idle_time to zero.
                computer.idle_time = Some(0);
            }
            // If Off, MoveMouse does nothing.
        }
        Event::PassTime(time) => {
            // Rule 4: Only applies if the computer is On.
            if let Some(current_uptime) = computer.uptime {
                let new_uptime = current_uptime + time;
                computer.uptime = Some(new_uptime); // Update uptime regardless of sub-state

                // Check if was RUNNING at the start of the event
                if let Some(current_idle_time) = computer.idle_time {
                    let new_idle_time = current_idle_time + time;
                    if new_idle_time > 1000 {
                        // --- Transition to Sleeping ---
                        computer.idle_time = None;
                        // Correction 1: Initial sleep time is excess idle time
                        let initial_sleep_time = new_idle_time - 1000;
                        computer.sleep_time = Some(initial_sleep_time);

                        // Correction 2: Immediately check if the new sleep state triggers shutdown
                        if initial_sleep_time > 500 {
                            computer = ComputerState::new_off(); // Turn off
                            // State changed to Off, no further processing needed for this event
                        }
                        // If it didn't turn off, it's now sleeping. End of processing for this PassTime.

                    } else {
                        // --- Still Running ---
                        computer.idle_time = Some(new_idle_time); // Just update idle time
                    }
                }
                // Check if was SLEEPING at the start of the event
                // Use else if because it cannot be Running and Sleeping simultaneously.
                // This block is NOT entered if it just transitioned from Running to Sleeping above.
                else if let Some(current_sleep_time) = computer.sleep_time {
                    let new_sleep_time = current_sleep_time + time;
                    if new_sleep_time > 500 {
                        // --- Transition to Off from Sleep ---
                        computer = ComputerState::new_off(); // Turn off
                        // State changed to Off, no further processing needed
                    } else {
                        // --- Still Sleeping ---
                        computer.sleep_time = Some(new_sleep_time); // Just update sleep time
                    }
                }
                // If the state was somehow invalid (On but neither Running nor Sleeping), do nothing more.

            } // End if computer.is_on()
            // If Off, PassTime does nothing.
        }
    }
    computer // Return the (potentially) modified state
}

// --- Tests ---
// Use the tests provided by the user. Ensure this code is in `src/lib.rs` or similar
// and the tests are in `tests/03_state_transition_struct.rs`.

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::{pc_transition, ComputerState, Event};

    #[test]
    fn turn_off_when_off() {
        // The matches!(<variable>, <pattern>) macro returns `true` if <variable> matches the
        // given <pattern>.
        // We could have nicer error messages with `assert_eq!`, but for that we would need to know
        // about traits first :) Stay tuned.

        let pc = ComputerState::new_off();
        let pc = pc_transition(pc, Event::TurnOff);
        assert!(!pc.is_on());
        assert!(!pc.is_sleeping());
        assert_eq!(pc.uptime(), 0);
        assert_eq!(pc.idle_time(), 0);
        assert_eq!(pc.sleep_time(), 0);
    }

    #[test]
    fn turn_off_when_running() {
        let pc = ComputerState::new_on();
        let pc = pc_transition(pc, Event::TurnOff);
        assert!(!pc.is_on());
        assert!(!pc.is_sleeping());
        assert_eq!(pc.uptime(), 0);
        assert_eq!(pc.idle_time(), 0);
        assert_eq!(pc.sleep_time(), 0);
    }

    #[test]
    fn turn_off_when_sleeping() {
        let pc = ComputerState::new_on();
        let pc = pc_transition(pc, Event::PassTime(1000));
        let pc = pc_transition(pc, Event::TurnOff);
        assert!(!pc.is_on());
        assert!(!pc.is_sleeping());
        assert_eq!(pc.uptime(), 0);
        assert_eq!(pc.idle_time(), 0);
        assert_eq!(pc.sleep_time(), 0);
    }

    #[test]
    fn turn_on_when_off() {
        let pc = ComputerState::new_off();
        let pc = pc_transition(pc, Event::TurnOn);

        assert!(pc.is_on());
        assert!(!pc.is_sleeping());
        assert_eq!(pc.uptime(), 0);
        assert_eq!(pc.idle_time(), 0);
        assert_eq!(pc.sleep_time(), 0);
    }

    #[test]
    fn turn_on_when_running() {
        let pc = ComputerState::new_on();
        let pc = pc_transition(pc, Event::TurnOn);
        let pc = pc_transition(pc, Event::TurnOn);

        assert!(pc.is_on());
        assert!(!pc.is_sleeping());
        assert_eq!(pc.uptime(), 0);
        assert_eq!(pc.idle_time(), 0);
        assert_eq!(pc.sleep_time(), 0);
    }

    #[test]
    fn turn_on_when_sleeping() {
        let pc = ComputerState::new_on();
        let pc = pc_transition(pc, Event::TurnOn);
        let pc = pc_transition(pc, Event::PassTime(1100));
        let pc = pc_transition(pc, Event::TurnOn);

        assert!(pc.is_on());
        assert!(pc.is_sleeping());
        assert_eq!(pc.uptime(), 1100);
        assert_eq!(pc.idle_time(), 0);
        assert_eq!(pc.sleep_time(), 100);
    }

    #[test]
    fn pass_time_off() {
        let pc = ComputerState::new_off();
        let pc = pc_transition(pc, Event::PassTime(1100));

        assert!(!pc.is_on());
        assert!(!pc.is_sleeping());
        assert_eq!(pc.uptime(), 0);
        assert_eq!(pc.idle_time(), 0);
        assert_eq!(pc.sleep_time(), 0);
    }

    #[test]
    fn pass_time_running() {
        let pc = ComputerState::new_on();
        let pc = pc_transition(pc, Event::PassTime(20));
        let pc = pc_transition(pc, Event::MoveMouse);
        let pc = pc_transition(pc, Event::PassTime(120));
        let pc = pc_transition(pc, Event::PassTime(123));

        assert!(pc.is_on());
        assert!(!pc.is_sleeping());
        assert_eq!(pc.uptime(), 263);
        assert_eq!(pc.idle_time(), 243);
        assert_eq!(pc.sleep_time(), 0);
    }

    #[test]
    fn pass_time_go_to_sleep() {
        let pc = ComputerState::new_on();
        let pc = pc_transition(pc, Event::PassTime(800));
        let pc = pc_transition(pc, Event::PassTime(320));

        assert!(pc.is_on());
        assert!(pc.is_sleeping());
        assert_eq!(pc.uptime(), 1120);
        assert_eq!(pc.idle_time(), 0);
        assert_eq!(pc.sleep_time(), 120);
    }

    #[test]
    fn pass_time_sleeping() {
        let pc = ComputerState::new_on();
        let pc = pc_transition(pc, Event::PassTime(1100));
        let pc = pc_transition(pc, Event::PassTime(320));

        assert!(pc.is_on());
        assert!(pc.is_sleeping());
        assert_eq!(pc.uptime(), 1420);
        assert_eq!(pc.idle_time(), 0);
        assert_eq!(pc.sleep_time(), 420);
    }

    #[test]
    fn pass_time_shutdown() {
        let pc = ComputerState::new_on();
        let pc = pc_transition(pc, Event::PassTime(800));
        let pc = pc_transition(pc, Event::PassTime(10000));

        assert!(!pc.is_on());
        assert!(!pc.is_sleeping());
        assert_eq!(pc.uptime(), 0);
        assert_eq!(pc.idle_time(), 0);
        assert_eq!(pc.sleep_time(), 0);
    }

    #[test]
    fn pass_time_sleeping_turn_off() {
        let pc = ComputerState::new_on();
        let pc = pc_transition(pc, Event::PassTime(800));
        let pc = pc_transition(pc, Event::PassTime(120));
        let pc = pc_transition(pc, Event::PassTime(700));

        assert!(!pc.is_on());
        assert!(!pc.is_sleeping());
        assert_eq!(pc.uptime(), 0);
        assert_eq!(pc.idle_time(), 0);
        assert_eq!(pc.sleep_time(), 0);
    }

    #[test]
    fn mouse_move_off() {
        let pc = ComputerState::new_off();
        let pc = pc_transition(pc, Event::PassTime(800));
        let pc = pc_transition(pc, Event::TurnOff);
        let pc = pc_transition(pc, Event::MoveMouse);

        assert!(!pc.is_on());
        assert!(!pc.is_sleeping());
        assert_eq!(pc.uptime(), 0);
        assert_eq!(pc.idle_time(), 0);
        assert_eq!(pc.sleep_time(), 0);
    }

    #[test]
    fn mouse_move_running() {
        let pc = ComputerState::new_on();
        let pc = pc_transition(pc, Event::PassTime(500));
        let pc = pc_transition(pc, Event::PassTime(100));
        let pc = pc_transition(pc, Event::MoveMouse);

        assert!(pc.is_on());
        assert!(!pc.is_sleeping());
        assert_eq!(pc.uptime(), 600);
        assert_eq!(pc.idle_time(), 0);
        assert_eq!(pc.sleep_time(), 0);
    }

    #[test]
    fn mouse_move_wake() {
        let pc = ComputerState::new_on();
        let pc = pc_transition(pc, Event::PassTime(500));
        let pc = pc_transition(pc, Event::PassTime(600));
        let pc = pc_transition(pc, Event::MoveMouse);

        assert!(pc.is_on());
        assert!(!pc.is_sleeping());
        assert_eq!(pc.uptime(), 1100);
        assert_eq!(pc.idle_time(), 0);
        assert_eq!(pc.sleep_time(), 0);
    }

    #[test]
    fn complex_transition_1() {
        let mut pc = ComputerState::new_off();
        let events = [
            Event::TurnOn,
            Event::PassTime(100),
            Event::PassTime(50),
            Event::MoveMouse,
            Event::PassTime(500),
            Event::PassTime(600),
            Event::PassTime(100),
            Event::MoveMouse,
            Event::PassTime(20),
            Event::PassTime(100),
        ];
        for event in events {
            pc = pc_transition(pc, event);
        }
        assert!(pc.is_on());
        assert!(!pc.is_sleeping());
        assert_eq!(pc.uptime(), 1470);
        assert_eq!(pc.idle_time(), 120);
        assert_eq!(pc.sleep_time(), 0);
    }

    #[test]
    fn complex_transition_2() {
        let mut pc = ComputerState::new_off();
        let events = [
            Event::TurnOn,
            Event::PassTime(100),
            Event::PassTime(50),
            Event::MoveMouse,
            Event::PassTime(500),
            Event::PassTime(600),
            Event::TurnOff,
            Event::MoveMouse,
            Event::PassTime(600),
            Event::TurnOn,
            Event::PassTime(100),
            Event::MoveMouse,
            Event::PassTime(20),
            Event::PassTime(100),
            Event::PassTime(1000),
            Event::TurnOn,
            Event::PassTime(150),
        ];
        for event in events {
            pc = pc_transition(pc, event);
        }
        assert!(pc.is_on());
        assert!(pc.is_sleeping());
        assert_eq!(pc.uptime(), 1370);
        assert_eq!(pc.idle_time(), 0);
        assert_eq!(pc.sleep_time(), 270);
    }
}
