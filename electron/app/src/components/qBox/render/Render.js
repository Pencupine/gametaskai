import React, { Component } from 'react';
import * as THREE from 'three';

class Render extends Component {
  componentDidMount() {
    // Set up the scene
    const scene = new THREE.Scene();

    // Set up the camera
    const camera = new THREE.PerspectiveCamera(
      50,
      700 / window.innerHeight,
      0.1,
      1000
    );
    camera.position.z = 5;

    // Set up the renderer
    const renderer = new THREE.WebGLRenderer({ antialias: true });
    renderer.setSize(700, 800);
    this.mount.appendChild(renderer.domElement);

    // Create a geometry
    const geometry = new THREE.BoxGeometry(1, 1, 1);

    // Create a material
    const material = new THREE.MeshBasicMaterial({ color: 0x00ff00 });

    // Create a mesh
    const cube = new THREE.Mesh(geometry, material);

    // Add the mesh to the scene
    scene.add(cube);

    // Animation loop
    const animate = () => {
      requestAnimationFrame(animate);

      // Rotate the cube
      cube.rotation.x += 0.01;
      cube.rotation.y += 0.01;

      // Render the scene with the camera
      renderer.render(scene, camera);
    };

    animate();
  }

  render() {
    return <div ref={(ref) => (this.mount = ref)} />;
  }
}

export default Render;
