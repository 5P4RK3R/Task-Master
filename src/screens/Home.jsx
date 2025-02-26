import React, { useEffect } from "react";
import Task from "../components/organisms/Task";
import Input from "../components/atoms/Input";
import Button from "../components/atoms/Button";

import { useSelector, useDispatch } from "react-redux";
import {
  selectTask,
  fetchTasks,
  fetchUsers,
} from "../storeManager/slices/taskSlice";
import {
  selectUser,
  addUser,
} from "../storeManager/slices/userSlice";

const Home = ({ auth }) => {
  const { tasks } = useSelector(selectTask);
  const { users } = useSelector(selectUser);

  const dispatch = useDispatch();
  useEffect(() => {
    if (auth.companyId && auth.authToken) {
      dispatch(fetchTasks(auth.companyId, auth.authToken));
      dispatch(fetchUsers(auth.companyId, auth.authToken));
    }

    return () => {};
  }, [auth]);
  return (
    <div className="container mx-auto">
       <Input
        id="username"
        type="text"
        label="Enter Username"
        // event={setTaskDetail}
        // value={taskDetail.task_date}
        />
        <Button
          // event={saveTask}
          bgColor="bg-blue-400"
          textColor="text-white"
          text="Create User"
        />
      {tasks.map((taskDetail, index) => (
        <Task
          users={users}
          taskDetail={taskDetail}
          index={index}
          id={taskDetail.id}
          auth={auth}
        />
      ))}
    </div>
  );
};

export default Home;
