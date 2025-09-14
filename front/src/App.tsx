import Markdown from "react-markdown";
import "./App.css";
import { useEffect, useState } from "react";
import remarkGfm from "remark-gfm";
import rehypeRaw from "rehype-raw";

function App() {
  const [readme, setReadme] = useState("");
  useEffect(() => {
    fetch(
      "https://raw.githubusercontent.com/wonjongin/calendar-dongguk/main/README.md",
    )
      .then((res) => res.text())
      .then((text) => setReadme(text));
  }, []);

  return (
    <div className="container markdown mt-0 sm:mt-10 p-8 sm:p-10 bg-white dark:bg-gray-800 rounded-lg shadow-lg">
      <Markdown remarkPlugins={[remarkGfm]} rehypePlugins={[rehypeRaw]}>
        {readme}
      </Markdown>
    </div>
  );
}

export default App;
