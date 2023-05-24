import React, { Component, Suspense } from 'react';
import { Canvas } from '@react-three/fiber';
import { OrbitControls } from '@react-three/drei';
import { Box, Plane } from '@react-three/drei';

import QStage from './qStage/QStage'

function QObj(data) {
    // console.log(data)
    const width = box.width
    const height = box.height
    const depth = box.depth
    const color = box.color
    const x=box.x
    const y=box.y
    const z=box.z
    return (
        <Box/>
    )
}

export default class QCanvas extends Component {
  constructor(props) {
    super(props);
  }

  render() {
    const screenWidth = this.props.screenSizeData.width;
    const ratio_string = this.props.screenSizeData.ratio;
    const ratio_numbers = ratio_string.split(':');
    const screenRatio = parseInt(ratio_numbers[0]) / parseInt(ratio_numbers[1]);
    const screenHeight = screenWidth / screenRatio;

    // let Item = null
    // console.log(this.props.renderData)


    return (
      <div >
        <Canvas
          style={{
            width: screenWidth,
            height: screenHeight,
            background: 'blue' // Set the background color to blue
          }}
          gl={{ antialias: true }}
        >
          {/* <Item /> */}
          {/* <Box 
            args={[2, 3, 5]}
            position={[1,1,1]}
          /> */}
          {/* <Plane args={[10, 10]} rotation={[-Math.PI / 2, 0, 0]} receiveShadow /> */}
          <Suspense fallback={null}>
            
          </Suspense>
        </Canvas>
      </div>
    );
  }
}
