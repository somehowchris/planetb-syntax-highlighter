import "./src/highlighter/source/SyntaxHighlighter.css";
import "./src/assets/css/material-kit.css";

import "./src/highlighter/source/index.js";
import './src/assets/js/material-kit.min.js';
import './src/assets/js/core/bootstrap.min.js';

import("./pkg").then((module) => {
  module.run_app();
});
