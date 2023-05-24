import React, { Component } from 'react';
import Render from './render/Render'

export default class QBox extends Component {
  constructor(props){
    super(props)

  }
  render() {
    return (
      <div>
        <Render renderData={this.props.data} /> 
      </div>
    )
  }
}
