import "./src/highligher/source/SyntaxHighlighter.scss";

import "./src/highligher/source/index.js";

import("./pkg").then((module) => {
  module.run_app();
});
