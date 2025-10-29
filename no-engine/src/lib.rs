pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

struct MWindow;

trait WindowSys {
    fn create() -> MWindow;
}

// # Capabilities required for 1-D games:
// 1. Ability to create a 1xX window
// 2. Ability to color a pixel
// 3. Ability to handle touch input
// 4. Ability to handle keyboard input
// 5. Ability to load a sound effect
// 6. Ability to play a sound effect
