// library.c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Define a Book struct
typedef struct Book {
    char title[100];
    char author[100];
    struct Book* next;
} Book;

// Create a linked list of books
Book* head = NULL;

// Add a new book to the library
void add_book(const char* title, const char* author) {
    Book* new_book = (Book*)malloc(sizeof(Book));
    strncpy(new_book->title, title, sizeof(new_book->title) - 1);
    strncpy(new_book->author, author, sizeof(new_book->author) - 1);
    new_book->next = head;
    head = new_book;
}

// Remove book by title
int remove_book(const char* title) {
    Book* temp = head;
    Book* prev = NULL;

    // If the head contains the book to be removed!!!
    if (temp != NULL && strcmp(temp->title, title) == 0) {
        head = temp->next;
        free(temp);
        return 1;
    }

    // Search for the book title in the list
    while (temp != NULL && strcmp(temp->title, title) != 0) {
        prev = temp;
        temp = temp->next;
    }

    // If book wasn't found...
    if (temp == NULL) return 0;

    prev->next = temp->next;
    free(temp);
    return 1;
}

// Display all books in the library!
void display_books() {
    Book* temp = head;
    if (temp == NULL) {
        printf("No books in the library.\n");
        return;
    }

    while (temp != NULL) {
        printf("Title: %s, Author: %s\n", temp->title, temp->author);
        temp = temp->next;
    }
}
