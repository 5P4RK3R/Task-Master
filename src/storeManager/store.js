import { configureStore } from "@reduxjs/toolkit";
import authReducer from "./slices/authSlice";
import taskReducer from "./slices/taskSlice";
import userReducer from "./slices/userSlice";
import logger from "redux-logger";
import thunk from "redux-thunk";
export const store = configureStore({
  reducer: {
    auth: authReducer,
    task: taskReducer,
    user: userReducer,
  },
  middleware: [thunk, logger],
});
