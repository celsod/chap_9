use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    //panic!("crash and burn!");

    /* let v = vec![1, 2, 3];

    v[99]; */

    let greeting_file_result = File::open("hello.txt");

    /*The code below looks for the file hello.txt.  The first match line looks for the file and attempts to go to it.
    The Err(error) line says that if there is an error, to match the error to the kind of error it is.  In this case,
    if the file is not found, then go ahead and match that to the file create line.  If the file is successfully created,
    then proceed to the file, if the file is not created, then the Err(e) line is activated.  If the file is there but
    the program is unable to open the file, then the other_error line is activated.  Keep track of the commas and semi-colons.
    That can get confusing. */

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file {:?}", other_error);
            }
        },
    };

    /*The code below is a simpler version of the code above and uses the unwrap method on the result enumeration.
    The code below does the same thing that the code above does but is less verbose. */
    let greeting_file = File::open("hello.txt").unwrap();


    /*The code below uses the expect method to create a customized error message in the event the open method does not work.
    This was used in an earlier project.  Production quality code typically uses expect instead of unwrap to follow why
    the code is not working as expected. */
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}

/* This function will return a Result of type string and io::Error.  If the function works as expected, the return will be a
string with a username in the string.  If there is an error, then the function will return an io::Error as this is the return type
of the operations of File::open function and .read_to_string method*/

fn read_username_from_file() -> Result<String, io::Error> { 
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result { //if the file is present, place the file into username_file
        Ok(file) => file,
        Err(e) => return Err(e), //instead of panic, return the error
    };

    let mut username = String::new(); //new variable

    match username_file.read_to_string(&mut username) { //read the contents of the file and write it to the new variable
        Ok(_) => Ok(username),
        Err(e) => Err(e), //instead of panic, return the error
        //Do not need to explicitly say return due to being the last expression of the function
    }
}

/* The code below is an alternate way to write the function above.  The question mark (?) does the same thing that the
match statements do in the function above.  They return the result if ok or return the error if not ok. The question mark operator
converts the error type received from the call into the error type defined in the return type of the function. */
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

/*The code below is an even shorter form of the code above.  Note how the File::open method is chained together with the read_to_string
and a question mark is at the end.  The same result occurs as with the code examples above. */
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

/*The shortest possible function code for the same thing as shown above.  Due to the commonality of reading a string from a file, Rust 
has made the function call very easy. */
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
    /*fs::read_to_string opens the file, creates a new string, reads the contents of the file, puts the contents into that string and returns it. */
}

/*The code below is an example of how to use the ? operator inside a function and in the middle of method calls.  From the Rust tutorial:

 This function returns Option<char> because it’s possible that there is a character there, but it’s also possible that there isn’t. 
 This code takes the text string slice argument and calls the lines method on it, which returns an iterator over the lines in the string. 
 Because this function wants to examine the first line, it calls next on the iterator to get the first value from the iterator. If text is the empty string, 
 this call to next will return None, in which case we use ? to stop and return None from last_char_of_first_line. If text is not the empty string, 
 next will return a Some value containing a string slice of the first line in text.

The ? extracts the string slice, and we can call chars on that string slice to get an iterator of its characters. 
We’re interested in the last character in this first line, so we call last to return the last item in the iterator. This is an Option because it’s 
possible that the first line is the empty string, for example if text starts with a blank line but has characters on other lines, as in "\nhi". 
However, if there is a last character on the first line, it will be returned in the Some variant. 
The ? operator in the middle gives us a concise way to express this logic, allowing us to implement the function in one line. 
If we couldn’t use the ? operator on Option, we’d have to implement this logic using more method calls or a match expression.*/
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}