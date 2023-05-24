import React, { Component } from 'react';
import { FormGroup, Label, Input, Button } from 'reactstrap';
import axios from 'axios';

class ChatBox extends Component {
  constructor(props) {
    super(props);

    this.state = {
      objectModel: '',
      message: '',
      response: '',
      postBody: '',
      chatHistory: []
    };

    this.handleSendMessage = this.handleSendMessage.bind(this);
    this.handleObjectModelChange = this.handleObjectModelChange.bind(this);
    this.handleInputChange = this.handleInputChange.bind(this);
    this.handleInputKeyPress = this.handleInputKeyPress.bind(this);
    this.handleLoadResponse = this.handleLoadResponse.bind(this);
    this.handleLoadPostBody = this.handleLoadPostBody.bind(this);
    this.handleRenderClick = this.handleRenderClick.bind(this);
  }

  async handleSendMessage() {
    try {
      const { message, postBody, chatHistory } = this.state;
      const { backendUrl } = this.props;

      if (message === 'get') {
        const url = `${backendUrl}/get`;
        const response = await axios.get(url);
        this.setState({ response: response.data });
      } else if (message === 'post') {
        const url = `${backendUrl}/post`;
        const requestBody = postBody; // Attach postBody in the request body

        const response = await axios.post(url, requestBody, {
          headers: { 'Content-Type': 'application/json' } // Set Content-Type header to application/json
        });

        this.setState({ response: response.data });
      }

      // Add message to chat history
      const updatedChatHistory = [...chatHistory, message];
      this.setState({ chatHistory: updatedChatHistory });

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

  handleInputKeyPress(event) {
    if (event.key === 'Enter') {
      event.preventDefault();
      this.handleSendMessage();
    }
  }

  handleLoadResponse() {
    const { response } = this.state;
    this.setState({ objectModel: JSON.stringify(response, null, 2) });
  }

  handleLoadPostBody() {
    const { response } = this.state;
    this.setState({ postBody: JSON.stringify(response, null, 2) });
  }

  handleRenderClick() {
    // Pass data to AppComp
    console.log("Rendering Object");
    this.props.renderDataFunc(this.state.objectModel);
  }

  render() {
    const { objectModel, message, response, postBody, chatHistory } = this.state;

    return (
      <div>
        <div className="chatbox">Panel</div>
        <div>
          <FormGroup>
            <Label for="objectModel">Object Model:</Label>
            <Input
              type="textarea"
              name="objectModel"
              id="objectModel"
              value={objectModel}
              onChange={this.handleObjectModelChange}
            />
          </FormGroup>
        </div>
        <hr /> {/* Add a horizontal line divider */}
        <div>
          <div>
            <FormGroup>
              <Label for="chatHistory">Chat History:</Label>
              <Input
                type="textarea"
                name="chatHistory"
                id="chatHistory"
                value={chatHistory.join('\n')}
                readOnly
              />
            </FormGroup>
          </div>
          <FormGroup>
            <Label for="message">Message:</Label>
            <div className="d-flex">
              <Input
                type="text"
                name="message"
                id="message"
                value={message}
                onChange={this.handleInputChange}
                onKeyPress={this.handleInputKeyPress}
              />
              <Button color="primary" onClick={this.handleSendMessage} className="ml-2">
                Send
              </Button>
            </div>
          </FormGroup>
          {/* Render the POST call body textarea */}
          <div>
            <FormGroup>
              <Label for="postBody">POST Body:</Label>
              <Input
                type="textarea"
                name="postBody"
                id="postBody"
                value={postBody} // Show the postBody in the textarea
                readOnly
              />
            </FormGroup>
          </div>
          {/* Render the response textarea */}
          <div>
            <FormGroup>
              <Label for="response">Response:</Label>
              <Input
                type="textarea"
                name="response"
                id="response"
                value={JSON.stringify(response, null, 2)}
                readOnly
              />
            </FormGroup>
          </div>
          {/* Button to load response into the first text area */}
          <Button color="info" onClick={this.handleLoadResponse} outline>
            Load Response
          </Button>
          {/* Button to load post body into the postBody textarea */}
          <Button color="info" onClick={this.handleLoadPostBody} outline>
            Load Post Body
          </Button>
        </div>
        <hr /> {/* Add another horizontal line divider */}
        <div>
          {/* Button to perform the Render action */}
          <Button color="primary" onClick={this.handleRenderClick}>
            Render
          </Button>
        </div>
      </div>
    );
  }
}

export default ChatBox;
