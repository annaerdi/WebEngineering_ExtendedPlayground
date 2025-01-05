# Web Engineering Coding Playground Template

This repository is designed as the foundation for coding playgrounds in the Web Engineering course. It offers a structured space for experimenting with and mastering various web development technologies and practices. 
The project is based on [this](https://developer.mozilla.org/en-US/docs/Learn/Accessibility/Accessibility_troubleshooting) repository from MDN.

The project introduces a lot of code smells for you to tackle. 
**Lets get coding!**

## Submission Details and Deadlines
* Coding playgrounds are **individual** work
* There will be 2 serparate submissions:
  * [Base Playgrounds](#base-coding-playgrounds): Submission Deadline **03.11.2024**
  * [Extended Playgrounds](#extended-coding-playgrounds): Submission Deadline **16.01.2025**
* The playgrounds will be guided through in our sessions - still there will be distance work!
* Use this base template to create your project repository.
* Each playground is linked in the corresponding course section.
* You can find the submissions at the bottom of the Moodle course.
  

## Features

- Wonderful UI-design :heart_eyes:
- Loads bear data using [Wikipedia API](https://www.mediawiki.org/wiki/API:Main_page) :bear:
  - Original Wikipedia Page can be found [here](https://en.wikipedia.org/wiki/List_of_ursids)
- Worst JS coding practices :cold_sweat:
- No Build and Dependency Management at all :fire:



# Extended Coding Playgrounds
Please create a new independent Repository for these playgrounds and submit a link to it in the Moodle submission. 
Additionally, provide a description of how to start your frontend and backend services inside the `README.md`.

## Submission
Submit your coding repository link in Moodle. Send me an invitation to your repository if it is set to private:
> GitHub: leonardo1710
> 
> GitLab: leon.freudenthaler@fh-campuswien.ac.at

## 5. Migrate to a Frontend Framework (10 pts.)
In this playground you will migrate your application to a frontend framework of your choice.

**Tasks**:
* Migrate your application to a frontend framework of your choice (e.g. React, Angular, Vue.js, Svelte,...)
  * All previous features should still work
  * The application still should use build and dependency management
* Adapt your `npm scripts` if necessary

Framework of choice: **Vue.js**

To install the frontend dependencies and start the application, run the following commands:
```bash
npm i
npm run dev
```

## 6. Integrate a Backend Framework (10 pts.)
In this playground you will use a backend framework of your choice and connect it with an API to your frontend application. 

**Tasks**:
* (3) Setup a backend framework of your choice
* (2) Create an API your frontend will be connected to (REST, GraphQL, gRPC, you choose...)
* (2) Your backend should now request the bear data from presented Wikipedia API
* (3) Replace the frontend Wikipedia API calls with calls to your backend - the functionality of your frontend should work as before!
* (Optional): you may want to introduce some sort of caching layer for Wikipedia API requests

Framework of choice: **axum (Rust)**

To start the backend, make sure you have [Rust](https://www.rust-lang.org/tools/install) installed and run the following:
```bash
cd backend
cargo run
```

## 7. Containerize your application (10 pts.)
Dockerize your frontend and backend applications. It should be possible to start all services in the corresponding mode (development, production) with a single command (e.g. use Docker Compose for this).

**Tasks**:
* (6) Create **multi-stage Dockerfiles** for your applications (depending on your frameworks):
  * The frontend Dockerfile should: 1. run the app in a development environment 2. build the app 3. serve build artefacts over Nginx
  * The backend Dockerfile should: 1. run the app in a development environment 2. build the app if there is a build step in your framework (optional) 3. serve the app 
* (4) Create two docker-compose files to orchestrate you applications in ``development`` and ``production`` mode:
  * Define ports and dependencies
  * Define corresponding stage (development, production)
  * Use environment variables if possible
* Your application should start with the following commands:
  * Development: `docker-compose -f docker-compose.yml up --build`
  * Production: `docker-compose -f docker-compose.prod.yml up --build`
