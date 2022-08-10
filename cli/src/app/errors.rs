use std::fmt;

//A specifier error is thrown when the user inputs an invalid specifier
//A specifier is a word that is passed during task creation that contains a ':' or one of the defined special symbols such as %. Ex: due:tue or %personal
pub struct SpecifierError<'a>(pub &'a str);

impl std::error::Error for SpecifierError<'_> { }

impl fmt::Display for SpecifierError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SpecifierError: \"{}\"", self.0)
    }
}

impl fmt::Debug for SpecifierError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SpecifierError: \"{}\". file: {}, line: {}", self.0, file!(), line!())
    }
}


pub struct TaskNotFoundError(pub u16);

impl std::error::Error for TaskNotFoundError { }

impl fmt::Display for TaskNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TaskNotFoundError: \"{}\" is an invalid task_id", self.0)
    }
}

impl fmt::Debug for TaskNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TaskNotFoundError: \"{}\" is an invalid task_id. file: {}, line: {}", self.0, file!(), line!())
    }
}


pub struct UncoveredError<'a>(pub &'a str);

impl std::error::Error for UncoveredError<'_> { }

impl fmt::Display for UncoveredError <'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UncoveredError: \"{}\" is uncovered", self.0)
    }
}

impl fmt::Debug for UncoveredError <'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UnvoceredError: \"{}\" is uncovered. file: {}, line: {}", self.0, file!(), line!())
    }
}


//TODO: test register error
pub struct RegisterError<'a>(pub &'a str);

impl std::error::Error for RegisterError<'_> { }

impl fmt::Display for RegisterError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RegisterError: \"{}\"", self.0)
    }
}

impl fmt::Debug for RegisterError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RegisterError: \"{}\". file: {}, line: {}", self.0, file!(), line!())
    }
}
