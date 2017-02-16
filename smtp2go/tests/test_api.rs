extern crate smtp2go;

#[test]
fn test_instantiation() {
	let s = smtp2go::Smtp2goApi::new(String::from("api-06AA9CCC552B11E68E9F90B11C30B754"));
}

#[test]
fn test_send() {
	let s = smtp2go::Smtp2goApi::new(String::from("api-06AA9CCC552B11E68E9F90B11C30B754"));
	let r = s.send(
		&String::from("matthew.lj.smith@gmail.com"), 
		&vec![String::from("matthew.lj.smith@gmail.com")], 
		&String::from("This is the Subject"),
		&String::from("This is the message")
	);
}