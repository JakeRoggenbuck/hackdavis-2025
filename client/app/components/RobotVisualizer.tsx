'use client';

import { useState, useEffect } from 'react';
import CodeMirror from '@uiw/react-codemirror';
import { javascript } from '@codemirror/lang-javascript';
import { Canvas } from '@react-three/fiber';
import { OrbitControls } from '@react-three/drei';

interface RobotState {
  x: number;
  y: number;
  rotation: number;
}

const Robot = ({ position, rotation }: { position: [number, number, number], rotation: number }) => {
  return (
    <mesh position={position} rotation={[0, rotation, 0]}>
      <boxGeometry args={[1, 0.5, 1.5]} />
      <meshStandardMaterial color="#646cff" />
    </mesh>
  );
};

export default function RobotVisualizer() {
  const [code, setCode] = useState(`// Write your robot commands here
// Example:
// add forward, 5
// add left, 10
// add backward, 100
// add right, 20`);

  const [robotState, setRobotState] = useState<RobotState>({
    x: 0,
    y: 0,
    rotation: 0
  });

  const executeCommand = (command: string) => {
    const parts = command.trim().split(',').map(part => part.trim());
    if (parts.length !== 2) return;

    const [action, valueStr] = parts[0].split(' ');
    const value = parseFloat(valueStr);

    switch (action) {
      case 'add forward':
        setRobotState(prev => ({ ...prev, y: prev.y + value }));
        break;
      case 'add backward':
        setRobotState(prev => ({ ...prev, y: prev.y - value }));
        break;
      case 'add left':
        setRobotState(prev => ({ ...prev, x: prev.x - value }));
        break;
      case 'add right':
        setRobotState(prev => ({ ...prev, x: prev.x + value }));
        break;
    }
  };

  const handleCodeChange = (value: string) => {
    setCode(value);
    // Reset robot state
    setRobotState({ x: 0, y: 0, rotation: 0 });
    // Execute each command
    value.split('\n').forEach(line => {
      if (line.trim().startsWith('add')) {
        executeCommand(line);
      }
    });
  };

  return (
    <div className="flex w-full h-screen">
      <div className="w-1/2 p-4 bg-secondary">
        <CodeMirror
          value={code}
          height="100%"
          theme="dark"
          extensions={[javascript()]}
          onChange={handleCodeChange}
          className="h-full rounded-lg overflow-hidden"
        />
      </div>
      <div className="w-1/2 bg-primary">
        <Canvas camera={{ position: [10, 10, 10] }}>
          <ambientLight intensity={0.5} />
          <pointLight position={[10, 10, 10]} />
          <Robot 
            position={[robotState.x / 10, 0, robotState.y / 10]} 
            rotation={robotState.rotation}
          />
          <gridHelper args={[20, 20]} />
          <OrbitControls />
        </Canvas>
      </div>
    </div>
  );
} 