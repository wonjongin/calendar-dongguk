import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./App.tsx";
import { BrowserRouter, Route } from "react-router";
import { Routes } from "react-router";
import Subscribe from "./Subscribe.tsx";

createRoot(document.getElementById("root")!).render(
  <BrowserRouter>
    <Routes>
      <Route path="/calendar" element={<App />} />
      <Route path="/subscribe" element={<Subscribe />} />
    </Routes>
  </BrowserRouter>,
);
