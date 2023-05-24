import React, { useEffect, useRef } from 'react';
import * as THREE from 'three';

const Item = ({ objectModel }) => {
  const mountRef = useRef(null);

  useEffect(() => {
    // Set up the scene
    const scene = new THREE.Scene();

    // Set up the camera
    const camera = new THREE.PerspectiveCamera(
      50,
      window.innerWidth / window.innerHeight,
      0.1,
      1000
    );
    camera.position.z = 5;

    // Set up the renderer
    const renderer = new THREE.WebGLRenderer({ antialias: true });
    renderer.setSize(900, 700);
    mountRef.current.appendChild(renderer.domElement);

    // Create a geometry
    const geometry = new THREE.BoxGeometry(1, 1, 1);

    // Create a material
    const material = new THREE.MeshBasicMaterial({ color: 0x00ff00 });

    // Create a mesh
    const cube = new THREE.Mesh(geometry, material);

    // Add the mesh to the scene
    scene.add(cube);


    renderer.render(scene, camera);

    // animate();

    return () => {
      // Clean up Three.js objects and release resources
      renderer.dispose();
      geometry.dispose();
      material.dispose();
    };
  }, []);

  return <div ref={mountRef} />;
};

export default Item;
