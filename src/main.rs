use largest::hospital::{Patient, HospitalStaff, Hospital, Occupant};
use std::cmp::Ord;

// Can ask for generics that satisfy certain traits.
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
	let mut big = list[0];
	for &i in list {
		if i > big {
			big = i;
		}
	}
	big
}

fn sort_asc_slice<T: Ord>(list: &mut [T]) -> &[T] {
	list.sort();
	list
}

fn sort_desc_slice<T: Ord>(list: &mut [T]) -> &[T] {
	list.sort();
	list.reverse();
	list
}

fn to_occupent_vec<'a>(p: Vec<Patient<'a>>,
					   s: Vec<HospitalStaff<'a>>) -> Vec<Box<dyn Occupant + 'a>> {
	let mut o: Vec<Box<dyn Occupant>> = Vec::new();
	for i in p {
		o.push(Box::new(i));
	}
	for j in s {
		o.push(Box::new(j));
	}
	o
}

fn main() {
	for arr in vec![vec![1, 5, 3], vec![5, 3, 2]] {
		println!("{}", largest(&arr));
	}

	let np = Patient::new;
	let ns = HospitalStaff::new;

	let patients = &mut vec![
		np("May", "Susie", 54, 2),
		np("Porsche", "Megan", 22, 1),
		np("Mars", "Tommy", 65,3)
	];
	let staff = &mut vec![
		ns("Jackie", "Robinson", 28, 1),
		ns("House", "McCabin", 45, 2),
		ns("Martha", "Stewart", 40, 3),
		ns("Morgan", "Williams", 30, 2)
	];
	let mut hos = Hospital::new("Saint Anna", 
		to_occupent_vec(patients.to_vec(), staff.to_vec()));

	println!("{}", largest(&vec![5.8, 6.4, 4.5]));
	println!("{}", largest(&vec!["Hello", "World", "Elmo", "Wand"]));
	println!("{}", largest(patients));
	println!("{:?}", sort_desc_slice(patients));
	println!("{:?}", sort_asc_slice(staff));
	println!("{}", hos);
	println!("Checking in new patient.");
	hos.check_in(Box::new(Patient::new("Maria", "Dennis", 27, 1)));
	println!("{}", hos);
	hos.curr_occupants.sort();
	hos.print_occupants();
}