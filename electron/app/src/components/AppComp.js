import React, { Component } from "react";
import { Container, Row, Col } from 'reactstrap';
import { Intent } from '@blueprintjs/core';
import ChatBox from './chat/Chat';
import QBox from "./qBox/QBox";

const backendUrl = process.env.REACT_APP_BACKEND_URL || 'http://localhost:3030';
console.log(backendUrl);

export default class AppComp extends Component {
    constructor(props) {
        super(props);
        this.state = {
            dataToQBox: null
        };
        this.handleRenderData = this.handleRenderData.bind(this);
    }

    handleRenderData(data) {
        // console.log(data);
        this.setState({ dataToQBox: data });
    }

    render() {
        console.log("DataObj : ",this.state.dataToQBox)
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

        return (
            <div style={containerStyle} className="">
                <Container fluid>
                    <Row>
                        <Col sm={12} md={9} lg={9} style={qBoxStyle}>
                            <QBox data={this.state.dataToQBox} />
                        </Col>
                        <Col sm={12} md={3} lg={3}>
                            <Row>
                                <Col xs={12} style={chatStyle}>
                                    <ChatBox backendUrl={backendUrl} renderDataFunc={this.handleRenderData} />
                                </Col>
                            </Row>
                        </Col>
                    </Row>
                </Container>
            </div>
        );
    }
}
