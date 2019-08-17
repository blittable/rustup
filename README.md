# Lessons in Rust 

## How to Use 

* The individual lessons are in the directories (e.g. lesson_one)  
* The README in those directories contains instructions for each lesson 
* The general format of the class requires you to commit code for each lesson.  The instructions for that process are below.

## Course Objectives
* Provide a 'gentle' introduction to Rust
* Keep the lessons small and focused
* Understand the basics of the language and provide a foundation for further development
* Provide some motiviation by looking at some interesting Rust projects 

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

2) Read the README for the lesson and fix the code (on your branch)  The 'master' branch lesson, normally, will not compile.  It has some placeholders like 'xxxxxx' that need to be replaced.  That's a part of the lesson.  

All of the lessons are command-line applications.  So, 1) it runs, and 2) it outputs something.  If something is confusing, ask in the chat group.  The content is challenging, but the lesson structure should not be.

3) Make sure it compiles and runs.  ;)

4) Commit and push to your remote branch.

Since this course is 'live' (lessons are being developed) you will need to update your branch from master to get the latest changes/lessons.  So, starting with Lesson 2, from *your* branch:

```
git fetch -p origin
git merge origin/master

```

## Other Notes

* The course assumes you have some development experience
* After it is complete, you will likely not yet be a Zen master of the Rust programming language

