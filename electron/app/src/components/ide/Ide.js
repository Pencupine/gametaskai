import React, { Component } from 'react'

export default class Ide extends Component {
    constructor(props) {
        super(props);
        this.state = {
            code: '',
            output: ''
        };
    }

    handleChange = (event) => {
        this.setState({ code: event.target.value });
    };

    handleSubmit = (event) => {
        event.preventDefault();

        // Send the Python code to your backend for execution
        // and receive the output
        const { code } = this.state;
        // Here you can make an API call to your server or use a library like Axios
        // to send the code to your backend and receive the output

        // For demonstration purposes, we'll simulate the API call with a timeout
        setTimeout(() => {
            // Simulating the output from the backend
            const output = 'Hello, World!';

            this.setState({ output });
        }, 1000); // Simulating a delay of 1 second
    };

    render() {
        const { code, output } = this.state;

        return (
            <div>
                <form onSubmit={this.handleSubmit}>
                    <label>
                        Python Code:
                        <textarea value={code} onChange={this.handleChange} />
                    </label>
                    <button type="submit">Run</button>
                </form>
                <div>
                    <h2>Output:</h2>
                    <pre>{output}</pre>
                </div>
            </div>
        );
    }
}
