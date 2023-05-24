import React, { Component } from 'react'
import {ambientLight} from '@react-three/fiber'

export default class QStage extends Component {
  constructor(props){
    super(props)
  }
  render() {
    return (
      <ambientLight />
    )
  }
}
