use crate::utils::LibError;
use crate::utils::LibError::*;
use std::ops::AddAssign;

// ----------------------------------------------------
// Energy
pub(crate) const MAX_ENERGY_LEVEL: usize = 100; // Maximum available energy, so if we decide to change is easier

/// Represents the energy quantity.
///
/// The `Energy` struct is used to define the energy level of a robot.
///
/// # Fields
///
/// - `energy_level`: An `usize` that holds the energy level of the robot.
///
/// # Usage
///
/// ```
/// use robotics_lib::energy::Energy;
/// fn energy_use(energy: &mut Energy) {
///     let energy_needed = 10;
///     if energy.has_enough_energy(energy_needed) {
///        print!("I have enough energy to do the action");
///     }
///     else {
///         print!("Oh no, I don't have enough energy 😴")}
/// }
/// ```
///
/// #Remarks
/// - The energy level is set to 0 by default
/// - Consume energy is pub(crate) because it should be used only by the robot
#[derive(Clone, Default)]
pub struct Energy {
    energy_level: usize,
}

impl Energy {
    pub(crate) fn new(energy_level: usize) -> Self {
        Energy {
            energy_level: std::cmp::min(energy_level, MAX_ENERGY_LEVEL),
        }
    }

    pub fn get_energy_level(&self) -> usize {
        self.energy_level
    }

    /// Sets the energy level
    ///
    /// # Arguments
    ///
    /// * `energy_level`: The energy level to set
    ///
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    ///
    /// # Remarks
    /// - The energy level is set to the energy level passed as argument
    /// - If the energy level passed as argument is greater than MAX_ENERGY_LEVEL, the energy level is set to MAX_ENERGY_LEVEL
    pub(crate) fn set_energy_level(&mut self, energy_level: usize) {
        self.energy_level = std::cmp::min(energy_level, MAX_ENERGY_LEVEL);
    }

    /// Consumes the energy needed
    ///
    /// # Arguments  
    /// * `energy_needed`: The energy needed to do the action
    ///
    /// # Returns
    /// returns: Result<(), LibError>
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    ///
    /// # Remarks
    /// - The energy level is decreased by the energy needed
    /// - If the energy needed is greater than the energy level, returns an error
    /// - If the energy needed is less than the energy level, returns Ok(())
    pub(crate) fn consume_energy(&mut self, energy_needed: usize) -> Result<(), LibError> {
        if !self.has_enough_energy(energy_needed) {
            return Err(NotEnoughEnergy);
        }
        self.energy_level -= energy_needed;
        Ok(())
    }

    pub fn has_enough_energy(&self, energy_needed: usize) -> bool {
        self.energy_level > energy_needed
    }

    /// Recharges the energy
    ///
    /// # Arguments
    ///
    /// * `energy_to_add`: The energy to add to the main energy level
    ///
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    ///
    /// # Remarks
    /// - The energy level is increased by the energy to add
    /// - If the energy level is greater than MAX_ENERGY_LEVEL, the energy level is set to MAX_ENERGY_LEVEL
    pub(crate) fn recharge_energy(&mut self, energy_to_add: usize) {
        self.energy_level = std::cmp::min(MAX_ENERGY_LEVEL, self.energy_level + energy_to_add);
    }

    // Merges the energy
    //
    // # Arguments
    //
    // * `energy`: The energy to merge with the main energy level
    //
    // # Examples
    //
    // ```
    //
    // ```
    //
    // # Remarks
    // -The energy level will be merged to the main energy level
    // -If the energy level is greater than MAX_ENERGY_LEVEL, the energy level is set to MAX_ENERGY_LEVEL
    // Alternative to the AddAssign trait

    // pub(crate) fn merge_energy(&mut self, energy: Energy) {
    //     self.recharge_energy(energy.get_energy_level());
    // }
}

impl AddAssign for Energy {
    fn add_assign(&mut self, other: Self) {
        if MAX_ENERGY_LEVEL - self.energy_level < other.energy_level {
            self.energy_level = MAX_ENERGY_LEVEL;
        } else {
            self.energy_level += other.energy_level;
        }
    }
}
