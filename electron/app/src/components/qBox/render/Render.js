import React, { Component } from 'react'
import { Canvas } from '@react-three/fiber';
import { Box } from '@react-three/drei';

export default class Render extends Component {
  render() {
    console.log("RENDER")
    return (
      <div>
        <Canvas>
          <ambientLight intensity={0.5} />
          <pointLight position={[10, 10, 10]} />
          <Box>
            <meshStandardMaterial attach="material" color="orange" />
          </Box>
        </Canvas>
      </div>
    )
  }
}
