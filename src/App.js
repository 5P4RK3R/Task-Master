import React, { useEffect } from "react";
import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import Home from "./screens/Home";
// import Login from "./screens/Login";
import "./App.css";
import { useSelector, useDispatch } from "react-redux";
import { selectAuth, fetchAuth } from "./storeManager/slices/authSlice";
const App = () => {
  const auth = useSelector(selectAuth);
  const dispatch = useDispatch();
  useEffect(() => {
    dispatch(fetchAuth());
    return () => {};
  }, []);
  return (
    <div className="App w-20 flex flex-col justify-center">
       {/* <Router>
      <Routes>
        <Route path="/" element={<Home auth={auth}  />} />
        {/* <Route path="/home" element={<Home auth={auth} />} /> */}
        {/* <Route path="*" element={<NotFound />} /> Catch-all for 404 */}
      {/* </Routes> */}
    {/* </Router> */}
      {/* {/* <Login auth={auth} /> */}
      <Home auth={auth} /> 
    </div>
  );
};

export default App;
