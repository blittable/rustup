# Lessons in Rust 

## Lessons

| Lesson          | Content                                             | 
| -------------   | --------------------------------------------------- | 
| Lesson One      | Setup and passing/returning String                  | 
| Lesson Two      | Standard Lib, Types, Immutability                   | 
| Lesson Three    | Introduction To Traits                              |                                 
| Lesson Four     | Enum Wrappers, Option<T>, and Matching              |                                 
| Lesson Five     | Error Handling Basics                               |                                 
| Lesson Six      | Loops and Almighty Iterators                        |                                 
| Lesson Seven    | Housekeeping (proj structure, some new types, etc.) |                                 
| Lesson Eight    | Closures and a Peek at Lifetimes                    |                                 
| Lesson Nine     | Lifetimes                                           |                                 
| Lesson Ten      | Unit Testing                                        |                                 
| Lesson Eleven   | Box<T>                                              |                                 
| Lesson Twelve   | Slices in Retrospect                                |                                 
| Lesson Thirteen | Trait Bounds                                        |                                 
| Lesson Fourteen | The dereference * operator                          |                                 
| Lesson Seventeen| Functions vs. methods and impl on structs           |                                 
| Lesson Eighteen | Slices in Retrospect                                |                                 
| Lesson Nineteen | Housekeeping: pinning runtime and rnd crate         |                                 
| Lesson 20       | The Sized Trait                                     |   
| Lesson 21       | Environment variables and the command line          |                                 
| Lesson 22       | Threads                                             |                                 
| Lesson 23       | Ref Stuff                                           |                                 
| Lesson 24       | Associated types on traits                          |   
| Lesson 25       | The Sized Trait                                     |   
| Lesson 26       | Housekeeping Part 3                                 |                                 
| Lesson 27       | Web (and other) Assembly                            |                                 
| Lesson 28       | TBD                                                 |                                 
| Lesson 29       | Futures                                             |                                 
| Lesson 30       | Pluggable Runtimes and Async/Await                  |                                 


## How to Use 

* The individual lessons are in the directories (e.g. lesson_one)  
* The README in those directories contains the lesson 
* The general format of the class requires you to commit code for each lesson.  The instructions for that process are below.

## Course Objectives
* Provide an introduction to Rust that will get you moving, quickly
* Keep the lessons small and focused, one or two topics, max
* Build a solid foundation in Rust for further development
* Provide some motiviation by looking at some interesting Rust projects (mostly during discussions) 

## Course Format
[At Mycos](https://www.mycostech.com) we'll have 20 minute discussions/presentations 30x to briefly go through topics and questions   


### Committing Homework Assignments 

* First, make sure that you are registered as a 'collaborator' in the project in github.  The course administrator will need a github ID to add you to the project.  If you don't have one, create one and then:

0) Clone this repository
1) Create a git branch with your github id as the name:
```
git branch yourgithbuid
git checkout yourgithubid
```
OR
```
git checkout -b yourgithubid
```
AND THEN
```
git push --set-upstream origin yourgithubid
```

2) Read the README for the lesson and fix/write the code (on your branch)  The 'master' branch lesson source may not compile, have masked values ****, or be empty.  The README will contain the details of the assignment. 

All of the lessons are command-line applications.  So, 1) it runs, and 2) it outputs something.  If something is confusing, ask in the chat group.  The content is challenging, but the lesson structure should not be.

3) Make sure it compiles and runs.  ;)

4) Commit and push to your remote branch.

Since this course is 'live' (lessons are being developed) you will need to update your branch from master to get the latest changes/lessons.  So, starting with Lesson 2, from *your* branch:

```
git fetch -p origin
git merge origin/master
```

The master branch is protected.

### Other Notes and Tips

* Resist the temptation to skim the documention,  jump over to github,  and start compiling.  A good foundation will get you moving faster.
* [The Rust Book] (https://doc.rust-lang.org/book/) is an excellent resource and very readable.  Chapter 4 is a must-read.  
* Realize that you're not obliged to use the 'advanced' features of the language.  There are lots of good applications and libraries that are 
light on generics, inter-thread communication```, etc
* After it is complete, you will likely not yet be a Zen master of the Rust programming language.

