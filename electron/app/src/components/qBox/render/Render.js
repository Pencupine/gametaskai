import React, { Component, createRef } from 'react';
import {Col, Row} from 'reactstrap';
import { Canvas } from '@react-three/fiber';
import { Container, HTMLSelect, Slider, Divider } from '@blueprintjs/core';

import QCanvas from './qCanvas/QCanvas';

import { spanContainerStyle, spanLeftPanelStyle, spanRightPanelStyle } from './RenderStyle';
class Render extends Component {
  constructor(props) {
    super(props);

    this.containerRef = createRef();

    this.state = {
      screenWidth: 0,
      screenRatio: '16:9', // Default screen ratio
      toolToCanvas:5,
      verticalCamAngle: 0,
      horizontalCamAngle: 0
    };
  }

  componentDidMount() {
    this.updateScreenSize();
    window.addEventListener('resize', this.updateScreenSize);
  }

  componentWillUnmount() {
    window.removeEventListener('resize', this.updateScreenSize);
  }

  updateScreenSize = () => {
    const { clientWidth } = this.containerRef.current;

    this.setState({
      screenWidth: clientWidth
    });
  };

  handleScreenRatioChange = (event) => {
    console.log("Event value", event.target.value)
    this.setState({
      screenRatio: event.target.value,
    });
  };

  handleVerticalSliderChange = (value) => {
    console.log('Vertical Slider value:', value);
    this.setState({verticalCamAngle: value})
  };

  handleHorizontalSliderChange = (value) => {
    console.log('Horizontal Slider value:', value);
    this.setState({horizontalCamAngle: value})
  };
  
  render() {
    const col_1 = 1;
    const col_2 = 12-col_1;
    
    const { renderData } = this.props;
    const { screenWidth, screenRatio } = this.state;

    const screenSizeData = {"width": screenWidth, "ratio": screenRatio};

    const leftColumnWidth = `${this.state.toolToCanvas}%`; // Define the desired width for the left column
    const rightColumnWidth = `calc(100% - ${leftColumnWidth})`; // Calculate the width for the right column  

    const rowContainerStyle = {
      display: 'flex',
      flexDirection: 'row',
    };

    const leftColumnStyle = {
      flex: leftColumnWidth,
    };

    const rightColumnStyle = {
      flex: rightColumnWidth,
    };

    return (
      <div>

        <div>
          <Row>
            <Col sm={col_1} md={col_1} lg={col_1}/>
            <Col sm={col_2} md={col_2} lg={col_2}>
              <HTMLSelect value={screenRatio} onChange={this.handleScreenRatioChange}>
                <option value="4:3">4:3</option>
                <option value="16:9">16:9</option>
              </HTMLSelect>
            </Col>
          </Row>
          <hr />
          <Row>
            <div style={rowContainerStyle}>
              <div style={leftColumnStyle}>
                <Slider 
                  vertical
                  min={-180}
                  max={180}
                  stepSize={1}
                  labelRenderer={true}
                  labelStepSize={180}
                  showTrackFill={true}
                  value={this.state.verticalCamAngle}
                  onChange={this.handleVerticalSliderChange}
                />
              </div>
              <div style={rightColumnStyle}>
                <div ref={this.containerRef}>
                  <QCanvas screenSizeData={screenSizeData} renderData={renderData} />
                </div>
              </div>
            </div>
          </Row>
          <hr/>
          <Row>
            <Col sm={col_1} md={col_1} lg={col_1}></Col>
            <Col sm={col_2} md={col_2} lg={col_2}>
              <Slider
                min={-180}
                max={180}
                stepSize={1}
                labelRenderer={true}
                labelStepSize={180}
                showTrackFill={true}
                value={this.state.horizontalCamAngle}
                onChange={this.handleHorizontalSliderChange}
              />
            </Col>
          </Row>
        </div>
      </div>
    );
  }
}

export default Render;
