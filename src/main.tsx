import React from "react";
import ReactDOM from "react-dom/client";
import "./styles.css";

const root = ReactDOM.createRoot(document.getElementById("root") as HTMLElement);

async function mountRoot() {
  if (import.meta.env.VITE_LIFE_OS_ANDROID_M2A === "1") {
    const { AndroidM2AApp } = await import("./android-m2a/AndroidM2AApp");
    await import("./android-m2a/android-m2a.css");
    root.render(
      <React.StrictMode>
        <AndroidM2AApp />
      </React.StrictMode>,
    );
    return;
  }

  if (import.meta.env.VITE_LIFE_OS_ANDROID_M1 === "1") {
    const { AndroidM1App } = await import("./android-m1/AndroidM1App");
    await import("./android-m1/android-m1.css");
    root.render(
      <React.StrictMode>
        <AndroidM1App />
      </React.StrictMode>,
    );
    return;
  }

  if (import.meta.env.VITE_LIFE_OS_ANDROID_FEASIBILITY_M0 === "1") {
    const { AndroidFeasibilityApp } = await import("./android-m0/AndroidFeasibilityApp");
    await import("./android-m0/android-feasibility-m0.css");
    root.render(
      <React.StrictMode>
        <AndroidFeasibilityApp />
      </React.StrictMode>,
    );
    return;
  }

  const { App } = await import("./app/App");
  root.render(
    <React.StrictMode>
      <App />
    </React.StrictMode>,
  );
}

void mountRoot();
