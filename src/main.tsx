import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { ShowProvider } from "./state/show";
import "./index.css";
import { BrowserRouter } from "react-router";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ShowProvider>
      <BrowserRouter>
        <App />
      </BrowserRouter>
    </ShowProvider>
  </React.StrictMode>,
);
