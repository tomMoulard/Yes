"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var react_1 = require("react");
var react_2 = require("@testing-library/react");
var App_1 = require("../src/App");
test('renders Bidding App heading', function () {
    (0, react_2.render)(<App_1.default />);
    var headingElement = react_2.screen.getByText(/Bidding App/i);
    expect(headingElement).toBeInTheDocument();
});
test('renders welcome message', function () {
    (0, react_2.render)(<App_1.default />);
    var welcomeElement = react_2.screen.getByText(/Welcome to the Bidding App!/i);
    expect(welcomeElement).toBeInTheDocument();
});
