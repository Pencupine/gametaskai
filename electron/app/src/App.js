import React, { Component } from "react";
import { Container, Row, Col } from 'reactstrap';
import { Intent } from '@blueprintjs/core';
import Chat from './components/chat/Chat';
import QBox from "./components/qBox/QBox";
import Ide from "./components/ide/Ide";

export default class App extends Component {
  render() {
    const containerStyle = {
      margin: "0 auto",
      padding: "20px",
      backgroundColor: "#F5F8FA",
    };

    const qBoxStyle = {
      backgroundColor: "#FFFFFF",
      boxShadow: "0 2px 4px rgba(16,22,26,.1)",
      borderRadius: "4px",
      padding: "20px",
      marginBottom: "20px",
    };

    const ideStyle = {
      backgroundColor: "#FFFFFF",
      boxShadow: "0 2px 4px rgba(16,22,26,.1)",
      borderRadius: "4px",
      padding: "20px",
      marginBottom: "20px",
      height: "calc(100% - 160px)", // Adjust height based on other components
    };

    const chatStyle = {
      backgroundColor: "#FFFFFF",
      boxShadow: "0 2px 4px rgba(16,22,26,.1)",
      borderRadius: "4px",
      padding: "20px",
      height: "calc(100% - 220px)", // Adjust height based on other components
    };

    const customStyle={
      backgroundColor: "blue"
    }

    return (
      <div style={ containerStyle} className="">
        <Container fluid>
          <Row>
            <Col sm={12} md={8} lg={9} style={qBoxStyle}>
              <QBox />
            </Col>
            <Col sm={12} md={4} lg={3}>
              <Row>
                <Col xs={12} className="mb-3" style={ideStyle}>
                  <Ide />
                </Col>
                <Col xs={12} style={chatStyle}>
                  <Chat />
                </Col>
              </Row>
            </Col>
          </Row>
        </Container>
      </div>
    );
  }
}
