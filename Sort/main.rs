extern crate libc;
use libc::{c_char, printf};
use std::ffi::{CString, CStr};

#[link(name = "library", kind = "static")]
extern "C" {
    fn add_book(title: *const c_char, author: *const c_char);
    fn remove_book(title: *const c_char) -> i32;
    fn display_books();
}

fn main() {
    println!("Library System");

    // Add books using FFI
    let title1 = CString::new("The Rust Programming Language").unwrap();
    let author1 = CString::new("Steve Klabnik and Carol Nichols").unwrap();
    unsafe {
        add_book(title1.as_ptr(), author1.as_ptr());
    }

    let title2 = CString::new("The C Programming Language").unwrap();
    let author2 = CString::new("Brian W. Kernighan and Dennis M. Ritchie").unwrap();
    unsafe {
        add_book(title2.as_ptr(), author2.as_ptr());
    }

    // Display books
    unsafe {
        display_books();
    }

    // Remove a book
    let title_to_remove = CString::new("The C Programming Language").unwrap();
    unsafe {
        let result = remove_book(title_to_remove.as_ptr());
        if result == 1 {
            println!("Book removed successfully.");
        } else {
            println!("Failed to remove the book.");
        }
    }

    // Display books again
    unsafe {
        display_books();
    }
}
