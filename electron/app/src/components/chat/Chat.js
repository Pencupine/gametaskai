import React, { Component } from 'react';
import axios from 'axios';

class ChatBox extends Component {
  constructor(props) {
    super(props);

    this.state = {
      objectModel: '',
      message: '',
      response: ''
    };

    this.handleSendMessage = this.handleSendMessage.bind(this);
    this.handleObjectModelChange = this.handleObjectModelChange.bind(this);
    this.handleInputChange = this.handleInputChange.bind(this);
  }

  async handleSendMessage() {
    try {
      const { message, objectModel } = this.state;
      const { backendUrl } = this.props;

      if (message === 'get') {
        const url = `${backendUrl}/get`;
        const response = await axios.get(url);
        this.setState({ response: response.data });
      } else if (message === 'post') {
        const url = `${backendUrl}/post`;
        const requestBody = objectModel; // Attach objectModel in the request body
        console.log(requestBody)
        const response = await axios.post(url, requestBody, {
          headers: { 'Content-Type': 'application/json' } // Set Content-Type header to application/json
        });
        this.setState({ response: response.data });
      }
    } catch (error) {
      console.error(error);
    }
  }

  handleObjectModelChange(event) {
    this.setState({ objectModel: event.target.value });
  }

  handleInputChange(event) {
    this.setState({ message: event.target.value });
  }

  render() {
    const { objectModel, message, response } = this.state;

    return (
      <div>
        <div className="chatbox">Panel</div>
        <div>
          <label>
            Object Model:
            <textarea value={objectModel} onChange={this.handleObjectModelChange} />
          </label>
        </div>
        <div>
          <input type="text" value={message} onChange={this.handleInputChange} />
          <button onClick={this.handleSendMessage}>Send</button>
          {/* Render the response within the Chat component */}
          <div>
            <pre>{JSON.stringify(response, null, 2)}</pre>
          </div>
        </div>
      </div>
    );
  }
}

export default ChatBox;
