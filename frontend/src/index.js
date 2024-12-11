"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var react_1 = require("react");
var client_1 = require("react-dom/client");
var App_1 = require("./App");
(0, client_1.default)(document.getElementById('app'))
    .render(<App_1.default tab="home"/>);
