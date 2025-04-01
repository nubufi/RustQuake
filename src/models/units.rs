/// Units for acceleration with conversion factors to `g`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccelUnit {
    G,
    Mps2,
    Cmps2,
    Mmps2,
    Incps2,
    Ftps2,
}

impl AccelUnit {
    /// Returns the multiplier to convert from the unit to `g`.
    pub fn to_g_factor(self) -> f64 {
        match self {
            AccelUnit::G => 1.0,
            AccelUnit::Mps2 => 1.0 / 9.81,
            AccelUnit::Cmps2 => 1.0 / 981.0,
            AccelUnit::Mmps2 => 1.0 / 9810.0,
            AccelUnit::Incps2 => 0.0025900792,
            AccelUnit::Ftps2 => 1.0 / 32.17404855643,
        }
    }

    /// Parses a string into an `AccelUnit`, returning `None` if unrecognized.
    pub fn from_str(unit: &str) -> Option<Self> {
        match unit.to_lowercase().as_str() {
            "g" => Some(AccelUnit::G),
            "m/s2" => Some(AccelUnit::Mps2),
            "cm/s2" => Some(AccelUnit::Cmps2),
            "mm/s2" => Some(AccelUnit::Mmps2),
            "inc/s2" => Some(AccelUnit::Incps2),
            "ft/s2" => Some(AccelUnit::Ftps2),
            _ => None,
        }
    }
}

/// Units for velocity with conversion factors to `cm/s`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VelUnit {
    Mps,
    Cmps,
    Mmps,
    Incps,
    Ftps,
}

impl VelUnit {
    /// Returns the multiplier to convert from the unit to `cm/s`.
    pub fn to_cmps_factor(self) -> f64 {
        match self {
            VelUnit::Mps => 100.0,
            VelUnit::Cmps => 1.0,
            VelUnit::Mmps => 0.1,
            VelUnit::Incps => 2.54,
            VelUnit::Ftps => 30.48,
        }
    }

    /// Parses a string into a `VelUnit`, returning `None` if unrecognized.
    pub fn from_str(unit: &str) -> Option<Self> {
        match unit.to_lowercase().as_str() {
            "m/s" => Some(VelUnit::Mps),
            "cm/s" => Some(VelUnit::Cmps),
            "mm/s" => Some(VelUnit::Mmps),
            "inc/s" => Some(VelUnit::Incps),
            "ft/s" => Some(VelUnit::Ftps),
            _ => None,
        }
    }
}

/// Units for displacement with conversion factors to `cm`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DispUnit {
    M,
    Cm,
    Mm,
    Inc,
    Ft,
}

impl DispUnit {
    /// Returns the multiplier to convert from the unit to `cm`.
    pub fn to_cm_factor(self) -> f64 {
        match self {
            DispUnit::M => 100.0,
            DispUnit::Cm => 1.0,
            DispUnit::Mm => 0.1,
            DispUnit::Inc => 2.54,
            DispUnit::Ft => 30.48,
        }
    }

    /// Parses a string into a `DispUnit`, returning `None` if unrecognized.
    pub fn from_str(unit: &str) -> Option<Self> {
        match unit.to_lowercase().as_str() {
            "m" => Some(DispUnit::M),
            "cm" => Some(DispUnit::Cm),
            "mm" => Some(DispUnit::Mm),
            "inc" => Some(DispUnit::Inc),
            "ft" => Some(DispUnit::Ft),
            _ => None,
        }
    }
}
