import PropTypes from 'prop-types'
import React, { Component } from 'react'

export default class QToolBar extends Component {
  constructor(props) {
    super(props)

    this.state = {
        verticalCamAngle: 0,
    }
  }

  handleSliderChange = (value) => {
    // Handle the slider value change here
    console.log('Slider value:', value);
    this.props.updateCamAngle(value);
    this.setState({verticalCamAngle: value})
  };

  render() {
    return (
      <div>
        <div style={leftColumnStyle(4)}>
            <Slider style={sliderStyle}
            vertical
            min={-180}
            max={180}
            stepSize={1}
            labelRenderer={true}
            labelStepSize={180}
            showTrackFill={true}
            value={this.state.verticalCamAngle}
            onChange={this.handleSliderChange}
            />
        </div>
      </div>
    )
  }
}
