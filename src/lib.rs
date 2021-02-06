pub mod hospital {
	use std::cmp::Ordering;
	use std::fmt;
	use std::boxed::Box;

	pub trait Occupant {
		fn info(&self) -> String;
		fn occ_type(&self) -> String;
		fn occ_code(&self) -> u8;
	}
	impl Ord for dyn Occupant {
		fn cmp(&self, other: &Self) -> Ordering {
			self.occ_code().cmp(&other.occ_code())
		}
	}
	impl Eq for dyn Occupant { }
	impl PartialOrd for dyn Occupant {
		fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
			self.occ_code().partial_cmp(&other.occ_code())
		}
	}
	impl PartialEq for dyn Occupant {
		fn eq(&self, other: &Self) -> bool {
			self.occ_code() == other.occ_code()
		}
	}

	#[derive(Copy, Clone, Debug)]
	pub struct Patient<'a> {
		pub last_name: &'a str,
		pub first_name: &'a str,
		pub age: u8,
		pub condition: u8,
	}
	impl Occupant for Patient<'_> {
		fn info(&self) -> String {
			format!("Name: {}, {} - age: {} - condition: {}",
				self.last_name, self.first_name, self.age, self.get_con_string())
		}
		fn occ_type(&self) -> String {
			"Patient".to_string()
		}
		fn occ_code(&self) -> u8 {
			2
		}
	}
	impl Ord for Patient<'_> {
		fn cmp(&self, other: &Self) -> Ordering {
			if self.condition == other.condition {
				self.age.cmp(&other.age)
			} else {
				self.condition.cmp(&other.condition)
			}
		}
	}
	impl Eq for Patient<'_> { }
	impl PartialOrd for Patient<'_> {
		fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
			if self.condition == other.condition {
				self.age.partial_cmp(&other.age)
			} else {
				self.condition.partial_cmp(&other.condition)
			}
		}
	}
	impl PartialEq for Patient<'_> {
		fn eq(&self, other: &Self) -> bool {
			self.condition == other.condition && self.age == other.age
		}
	}
	impl fmt::Display for Patient<'_> {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			write!(f, "Name: {}, {} - age: {} - condition: {}",
				self.last_name, self.first_name, self.age, self.get_con_string())
		}
	}
	impl<'a> Patient<'a> {
		pub fn new(f_name: &'a str, l_name: &'a str, a: u8, con: u8) -> Self {
			Patient {
				first_name: f_name,
				last_name: l_name,
				age: a,
				condition: con
			}
		}
		pub fn get_con_string(&self) -> &str {
			match self.condition {
				1 => "Stable",
				2 => "Unstable",
				3 => "Critical",
				_ => "[invalid condition code]"
			}
		}
	}

	#[derive(Clone, Copy, Debug)]
	pub struct HospitalStaff<'a> {
		pub first_name: &'a str,
		pub last_name: &'a str,
		pub age: u8,
		pub position: u8
	}
	impl Occupant for HospitalStaff<'_> {
		fn info(&self) -> String {
			format!("Name: {}, {} - Position: {} - Age: {}",
				self.last_name, self.first_name, self.get_pos_string(), self.age)
		}
		fn occ_type(&self) -> String {
			"HospitalStaff".to_string()
		}
		fn occ_code(&self) -> u8 {
			1
		}
	}
	impl Ord for HospitalStaff<'_> {
		fn cmp(&self, other: &Self) -> Ordering {
			if self.last_name == other.last_name {
				self.first_name.cmp(&other.first_name)
			} else {
				self.last_name.cmp(&other.last_name)
			}
		}
	}
	impl Eq for HospitalStaff<'_> { }
	impl PartialOrd for HospitalStaff<'_> {
		fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
			if self.last_name == other.last_name {
				self.first_name.partial_cmp(&other.first_name)
			} else {
				self.last_name.partial_cmp(&other.last_name)
			}
		}
	}
	impl PartialEq for HospitalStaff<'_> {
		fn eq(&self, other: &Self) -> bool {
			self.first_name == other.first_name && self.last_name == other.last_name
		}
	}
	impl fmt::Display for HospitalStaff<'_> {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			write!(f, "Name: {}, {} - Position: {} - Age: {}",
				self.last_name, self.first_name, self.get_pos_string(), self.age)
		}
	}
	impl<'a> HospitalStaff<'a> {
		pub fn new(f_name: &'a str, l_name: &'a str, a: u8, pos: u8) -> Self {
			HospitalStaff {
				first_name: f_name,
				last_name: l_name,
				age: a,
				position: pos
			}
		}
		pub fn get_pos_string(&self) -> &str {
			match self.position {
				1 => "Nurse",
				2 => "Doctor",
				3 => "Office Staff",
				_ => "other"
			}
		}
	}

	pub struct Hospital<'a> {
		pub name: &'a str,
		pub curr_occupants: Vec<Box<dyn Occupant>>
	}
	impl fmt::Display for Hospital<'_> {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			write!(f, "Name: {}, Current Occupants: {}",
				self.name, self.curr_occupants.len())
		}
	}
	impl<'a> Hospital<'a> {
		pub fn new(n: &'a str, occs: Vec<Box<dyn Occupant>>) -> Self {
			Hospital {
				name: n,
				curr_occupants: occs
			}
		}
		pub fn check_in(&mut self, person: Box<dyn Occupant>) {
			self.curr_occupants.push(person);
		}
		pub fn print_occupants(&self) {
			for o in self.curr_occupants.iter() {
				println!("{}", o.info());
			}
		}
	}
}