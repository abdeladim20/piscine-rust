// #[derive(Debug, Eq, PartialEq)]
// pub struct FormError {
//     // expected public fields
//     pub 
// }

// impl FormError {
//     pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
//         todo!()
//     }
// }

// #[derive(Debug, Eq, PartialEq)]
// pub struct Form {
//     // expected public fields
// }

// impl Form {
//     pub fn validate(&self) -> Result<(), FormError> {
//         todo!()
//     }
// }

// /* 
// Form { name: "Lee", password: "qwqwsa1dty_" }
// Ok(())
// Err(FormError { form_values: ("name", ""), date: "2022-10-17 12:09:25", err: "Username is empty" })
// Err(FormError { form_values: ("password", "dty_1"), date: "2022-10-17 12:09:25", err: "Password should be at least 8 characters long" })
// Err(FormError { form_values: ("password", "asdasASd(_"), date: "2022-10-17 12:09:25", err: "Password should be a combination of ASCII numbers, letters and symbols" })
// Err(FormError { form_values: ("password", "asdasASd123SA"), date: "2022-10-17 12:09:25", err: "Password should be a combination of ASCII numbers, letters and symbols" })
// */