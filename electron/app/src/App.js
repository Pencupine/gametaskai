import React, { Component } from "react";

import AppComp from "./components/AppComp";

const backendUrl = process.env.REACT_APP_BACKEND_URL || 'http://localhost:3030';
console.log(backendUrl);

export default class App extends Component {
    render() {
        return (
            <div className="bp4-dark">
              <AppComp/>
            </div>
        );
    }
}
