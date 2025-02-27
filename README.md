### **Objective:**

The goal of this assignment is to assess your proficiency in Rust and your chosen web framework (Axum, Actix, or Rocket), with a focus on **adhering to functional programming principles**. You will be tasked with developing a small web application SaaS that demonstrates your ability to design, implement, and test a scalable, fault-tolerant, and responsive system using Rust.

### **Project Description:**

You are required to build a simplified version of a task management web application SaaS. The application should allow users to create, update, delete, and list tasks. Each task will have a title, description, due date, and status (e.g., "To Do," "In Progress," "Done"). The application should support multiple users, with each user having their own set of tasks.

### **Requirements:**

1. **Backend:**
    - Use **Rust** for all backend logic.
    - Implement the backend using your choice of the following web frameworks:
        - **Axum**
        - **Actix**
        - **Rocket**
    - Store task data in an in-memory database (e.g., using HashMap or another suitable data structure).
2. **API:**
    - Expose a RESTful API with the following endpoints:
        - `POST /users`: Create a new user.
        - `POST /users/:user_id/tasks`: Create a new task for the specified user.
        - `GET /users/:user_id/tasks`: Retrieve all tasks for the specified user.
        - `GET /users/:user_id/tasks/:task_id`: Retrieve a specific task for the specified user.
        - `PUT /users/:user_id/tasks/:task_id`: Update a specific task for the specified user.
        - `DELETE /users/:user_id/tasks/:task_id`: Delete a specific task for the specified user.
3. **Functional Programming:**
    - Ensure the application is built following functional programming principles where possible. This includes immutability, pure functions, and avoiding side effects.
4. **Concurrency:**
    - Use Rust's built-in concurrency features to manage state and ensure the application is responsive and fault-tolerant.
5. **Error Handling:**
    - Implement robust error handling and logging throughout the application.
6. **Testing:**
    - Write unit tests and integration tests to ensure the correctness of your implementation.

### **Deliverables:**

1. **Source Code:**
    - Provide the complete source code of the application in a public GitHub repository. Include clear instructions on how to build and run the application.
    - Ensure that the source code is organized with atomic commits. Each commit should represent a single, cohesive change and include a meaningful commit message.
2. **Documentation:**
    - Write a brief documentation ([README.md](http://readme.md/)) explaining the architecture, design decisions, and how to set up and use the application.
3. **Tests:**
    - Include a test suite that demonstrates the functionality of the application. Provide instructions on how to run the tests.

### **Evaluation Criteria:**

- **Code Quality:** Adherence to Rust and functional programming best practices.
- **Atomic Commits:** Clear, concise commits that represent individual changes or features.
- **Correctness:** The application should meet all the specified requirements and pass all provided tests.
- **Scalability and Performance:** Use of Rust's concurrency features to ensure the application is responsive and fault-tolerant.
- **Documentation:** Clarity and completeness of the provided documentation.
- **Testing:** Coverage and effectiveness of the provided tests.

Good luck with your project!
This project was bootstrapped with [Create React App](https://github.com/facebook/create-react-app), using the [Redux](https://redux.js.org/) and [Redux Toolkit](https://redux-toolkit.js.org/) template.

## Available Scripts
In the project directory, you can run to install packages:
### `npm i`

In the project directory, you can run:

### `npm start`

Runs the app in the development mode.<br />
Open [http://localhost:3000](http://localhost:3000) to view it in the browser.

The page will reload if you make edits.<br />
You will also see any lint errors in the console.

### `npm test`

Launches the test runner in the interactive watch mode.<br />
See the section about [running tests](https://facebook.github.io/create-react-app/docs/running-tests) for more information.

### `npm run build`

Builds the app for production to the `build` folder.<br />
It correctly bundles React in production mode and optimizes the build for the best performance.

The build is minified and the filenames include the hashes.<br />
Your app is ready to be deployed!

See the section about [deployment](https://facebook.github.io/create-react-app/docs/deployment) for more information.
