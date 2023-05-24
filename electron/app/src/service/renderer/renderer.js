import * as THREE from 'three';

export function setupScene() {
  return new THREE.Scene();
}

export function setupCamera() {
  // console.log("Camera Setup");
  const camera = new THREE.PerspectiveCamera(
    50,
    900 / 700,
    0.1,
    1000
  );
  camera.position.z = 5;
  return camera;
}

export function setupRenderer() {
  // console.log("Renderer setup");
  const renderer = new THREE.WebGLRenderer({ antialias: true });
  renderer.setSize(900, 700);
  return renderer;
}

export function createShape(shapeData) {
  // console.log("Creating shape");

  let geometry, material;

  // Determine the shape type from the JSON object
  switch (shapeData.qtype) {
    case "cube":
      geometry = new THREE.BoxGeometry(shapeData.width, shapeData.height, shapeData.depth);
      break;
    case "cone":
      geometry = new THREE.ConeGeometry(shapeData.radius, shapeData.height, shapeData.segments);
      break;
    default:
      console.error("Unsupported shape type:", shapeData.type);
      return null;
  }

  material = new THREE.MeshBasicMaterial({ color: shapeData.color });

  const shape = new THREE.Mesh(geometry, material);
  shape.position.set(shapeData.x, shapeData.y, shapeData.z);

  return shape;
}


export function animate(scene, camera, renderer, cube) {
  // console.log("Animating");
  const animateInternal = () => {
    requestAnimationFrame(animateInternal);

    // Rotate the cube
    cube.rotation.x += 0.01;
    cube.rotation.y += 0.01;

    // Render the scene with the camera
    renderer.render(scene, camera);
  };

  animateInternal();
}
