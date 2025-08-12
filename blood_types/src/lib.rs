use std::fmt::{self, Debug};

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
	A,
	AB,
	B,
	O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
	Positive,
	Negative,
}

#[derive(PartialEq, Eq)]
pub struct BloodType {
	pub antigen: Antigen,
	pub rh_factor: RhFactor,
}

use std::cmp::{Ord, Ordering};

use std::str::FromStr;

impl FromStr for Antigen {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "B" => Ok(Antigen::B),
            "AB" => Ok(Antigen::AB),
            "O" => Ok(Antigen::O),
            _ => Err(()),
        }
    }
}

impl FromStr for RhFactor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err(()),
        }
    }
}

impl FromStr for BloodType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (antigen_str, rh_factor_str) = s.split_at(s.len() - 1);
        let antigen = antigen_str.parse()?;
        let rh_factor = rh_factor_str.parse()?;

        Ok(BloodType { antigen, rh_factor })
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.antigen
            .cmp(&other.antigen)
            .then(self.rh_factor.cmp(&other.rh_factor))
    }
}
impl PartialOrd for BloodType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let antigen_str = match self.antigen {
            Antigen::A => "A",
            Antigen::B => "B",
            Antigen::AB => "AB",
            Antigen::O => "O",
        };
        let rh_factor_str = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };
        write!(f, "{}{}", antigen_str, rh_factor_str)
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        let rh_compatible = match self.rh_factor {
            RhFactor::Positive => true,
            RhFactor::Negative => other.rh_factor == RhFactor::Negative,
        };

        let antigen_compatible = match self.antigen {
            Antigen::A => other.antigen == Antigen::A || other.antigen == Antigen::O,
            Antigen::B => other.antigen == Antigen::B || other.antigen == Antigen::O,
            Antigen::AB => true,
            Antigen::O => other.antigen == Antigen::O,
        };

        rh_compatible && antigen_compatible
    }

    pub fn donors(&self) -> Vec<Self> {
        all_blood_types()
            .into_iter()
            .filter(|donor| self.can_receive_from(donor))
            .collect()
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        all_blood_types()
            .into_iter()
            .filter(|recipient| recipient.can_receive_from(self))
            .collect()
    }
}

fn all_blood_types() -> Vec<BloodType> {
    vec![
        BloodType { antigen: Antigen::A, rh_factor: RhFactor::Positive },
        BloodType { antigen: Antigen::A, rh_factor: RhFactor::Negative },
        BloodType { antigen: Antigen::B, rh_factor: RhFactor::Positive },
        BloodType { antigen: Antigen::B, rh_factor: RhFactor::Negative },
        BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Positive },
        BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Negative },
        BloodType { antigen: Antigen::O, rh_factor: RhFactor::Positive },
        BloodType { antigen: Antigen::O, rh_factor: RhFactor::Negative },
    ]
}