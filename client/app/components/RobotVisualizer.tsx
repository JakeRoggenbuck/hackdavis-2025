'use client';

import { useState, useEffect, useRef } from 'react';
import CodeMirror from '@uiw/react-codemirror'; // React wrapper for CodeMirror
//import { javascript } from '@codemirror/lang-javascript'; // JavaScript syntax highlighting
import { cpp } from '@codemirror/lang-cpp'; // C++ syntax highlighting
import {HighlightStyle} from "@codemirror/highlight"
import { EditorView } from '@codemirror/view'; // CodeMirror EditorView
import {tags} from "@lezer/highlight";
import { basicSetup } from '@codemirror/basic-setup'; // Basic setup for editor
import { Canvas } from '@react-three/fiber'; // React Three.js for 3D rendering
import { OrbitControls } from '@react-three/drei';
import { StreamLanguage } from '@codemirror/language';
import { autocompletion, CompletionContext } from '@codemirror/autocomplete';




// myCustomMode.js

// Define the custom mode

const myLanguage = StreamLanguage.define({
  startState: () => ({}),
  token: (stream) => {
    if (stream.match(/(?:mov|jal)\b/)) return 'keyword';
	if (stream.match(/(?:forward|backward|direction|wait)\b/)) return 'comment';
    if (stream.match(/"(?:[^\\"]|\\.)*"/)) return 'string';
    stream.next();
    return null;
  },
});
const myHighlightStyle = HighlightStyle.define([
  { tag: tags.keyword, color: '#ff0055', fontWeight: 'bold' },
  { tag: tags.comment, color: '#888888', fontStyle: 'italic' },
  { tag: tags.string, color: '#22863a' },
]);
  const keywords = ["mov", "jal"];
  const registerNames = ["forward", "backward", "direction", "wait"];
  import { syntaxTree } from '@codemirror/language'; // Import syntaxTree facet
  const customCompletionSource = (context: CompletionContext) => {
  const before = context.state.sliceDoc(Math.max(0, context.pos - 20), context.pos);
  const match = before.match(/(\w+)$/);
  const word = match ? match[1] : "";

  return {
    from: match ? context.pos - word.length : context.pos,
    options: [
      ...keywords.filter(keyword => keyword.startsWith(word)).map(keyword => ({
        label: keyword,
        type: 'keyword'
      })),
      ...registerNames.filter(reg => reg.startsWith(word)).map(reg => ({
        label: reg,
        type: 'registerName'
      })),
    ],
  };
};


// myCustomMode.js

interface RobotState {
  x: number;
  y: number;
  rotation: number;
}

interface AnimationState extends RobotState {
  targetX: number;
  targetY: number;
  targetRotation: number;
  isAnimating: boolean;
  currentCommand: string;
}

interface CompilationStatus {
  status: 'idle' | 'compiling' | 'success' | 'error';
  message: string;
}

interface UploadStatus {
  status: 'idle' | 'uploading' | 'success' | 'error';
  message: string;
}

interface Label {
  name: string;
  instructions: string[];
}

const ANIMATION_DURATION = 1000; // 1 second per movement
const INSTRUCTION_DELAY = 100; // 0.1 second between instructions

const Robot = ({ position, rotation }: { position: [number, number, number], rotation: number }) => {
  return (
    <mesh position={position} rotation={[0, rotation, 0]}>
      <boxGeometry args={[1, 0.5, 1.5]} />
      <meshStandardMaterial color="#646cff" />
    </mesh>
  );
};

const LoadingBar = () => {
  return (
    <div className="absolute top-0 left-0 w-full h-1 bg-gray-700 overflow-hidden">
      <div className="w-1/2 h-full bg-accent animate-[loading_2s_ease-in-out_infinite]" />
    </div>
  );
};

const HelpMenu = () => {
  const [isOpen, setIsOpen] = useState(true);

  return (
    <div className="mt-4">
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="flex items-center gap-2 text-gray-300 hover:text-white transition-colors"
      >
        <span className="text-sm">Assembly Help</span>
        <svg
          className={`w-4 h-4 transform transition-transform ${isOpen ? 'rotate-180' : ''}`}
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
        </svg>
      </button>
      
      {isOpen && (
        <div className="mt-2 p-4 bg-[#1e1e1e] rounded-lg border border-[#333] text-sm text-gray-300">
          <h3 className="text-white font-medium mb-2">Available Commands:</h3>
          <ul className="space-y-2">
            <li>
              <code className="bg-[#2a2a2a] px-2 py-1 rounded">mov direction, 0</code> - Move straight
            </li>
            <li>
              <code className="bg-[#2a2a2a] px-2 py-1 rounded">mov direction, 1</code> - Turn left
            </li>
            <li>
              <code className="bg-[#2a2a2a] px-2 py-1 rounded">mov direction, 2</code> - Turn right
            </li>
            <li>
              <code className="bg-[#2a2a2a] px-2 py-1 rounded">mov forward, X</code> - Move forward X units
            </li>
            <li>
              <code className="bg-[#2a2a2a] px-2 py-1 rounded">jal label</code> - Jump to label and return
            </li>
          </ul>
          
          <h3 className="text-white font-medium mt-4 mb-2">Writing Tips:</h3>
          <ul className="space-y-2">
            <li>• Always start with a <code className="bg-[#2a2a2a] px-2 py-1 rounded">main:</code> label</li>
            <li>• Use labels to organize your code into reusable sections</li>
            <li>• Comments start with <code className="bg-[#2a2a2a] px-2 py-1 rounded">#</code></li>
            <li>• Use <code className="bg-[#2a2a2a] px-2 py-1 rounded">jal</code> for function-like behavior</li>
            <li>• Be careful with movement values - too large values may cause the robot to move off-screen</li>
          </ul>
        </div>
      )}
    </div>
  );
};

export default function RobotVisualizer() {
  const [code, setCode] = useState(`circle:
    mov direction, 1
    mov forward, 4
    mov direction, 0

main:
    jal circle
    mov forward, 2
    jal circle
    
    

    



    
    `);

  const [cppCode, setCppCode] = useState<string>('// Generated Arduino C++ code will appear here');
  const [showCppCode, setShowCppCode] = useState(false);
  const [uploadStatus, setUploadStatus] = useState<UploadStatus>({
    status: 'idle',
    message: 'Ready to upload'
  });

  const [robotState, setRobotState] = useState<AnimationState>({
    x: 0,
    y: 0,
    rotation: 0,
    targetX: 0,
    targetY: 0,
    targetRotation: 0,
    isAnimating: false,
    currentCommand: ''
  });

  const [compilationStatus, setCompilationStatus] = useState<CompilationStatus>({
    status: 'idle',
    message: 'Ready to compile'
  });

  const animationFrame = useRef<number>();
  const startTime = useRef(0);
  const isAnimating = useRef(false);

  useEffect(() => {
    return () => {
      if (animationFrame.current) {
        cancelAnimationFrame(animationFrame.current);
      }
    };
  }, []);

  const animate = (timestamp: number) => {
    if (!isAnimating.current) return;

    const progress = Math.min(1, (timestamp - startTime.current) / ANIMATION_DURATION);
    const easeProgress = 1 - Math.pow(1 - progress, 2); // Smoother easing

    setRobotState(prev => {
      const newX = prev.x + (prev.targetX - prev.x) * easeProgress;
      const newY = prev.y + (prev.targetY - prev.y) * easeProgress;
      const newRotation = prev.rotation + (prev.targetRotation - prev.rotation) * easeProgress;

      return {
        ...prev,
        x: newX,
        y: newY,
        rotation: newRotation,
        isAnimating: progress < 1
      };
    });

    if (progress < 1) {
      animationFrame.current = requestAnimationFrame(animate);
    } else {
      isAnimating.current = false;
      setRobotState(prev => ({
        ...prev,
        x: prev.targetX,
        y: prev.targetY,
        rotation: prev.targetRotation,
        isAnimating: false
      }));
    }
  };

  const parseLabels = (code: string): Map<string, Label> => {
    const labels = new Map<string, Label>();
    let currentLabel: Label | null = null;
    
    code.split('\n').forEach(line => {
      const trimmedLine = line.trim();
      
      // Skip empty lines and comments
      if (!trimmedLine || trimmedLine.startsWith('#')) return;
      
      // Check if line is a label
      if (trimmedLine.endsWith(':')) {
        const labelName = trimmedLine.slice(0, -1);
        currentLabel = { name: labelName, instructions: [] };
        labels.set(labelName, currentLabel);
      } 
      // Add instruction to current label
      else if (currentLabel && trimmedLine) {
        const instruction = trimmedLine.split('#')[0].trim(); // Remove comments
        if (instruction) {
          currentLabel.instructions.push(instruction);
        }
      }
    });
    
    return labels;
  };

  const executeInstruction = async (instruction: string) => {
    return new Promise<void>((resolve) => {
      const [cmd, ...params] = instruction.split(' ').filter(Boolean);
      const args = params.join(' ').split(',').map(p => p.trim());

      // Update current command display
      setRobotState(prev => ({ ...prev, currentCommand: instruction }));

      switch (cmd.toLowerCase()) {
        case 'mov': {
          const [type, value] = args;
          const numValue = parseFloat(value);
          
          if (type === 'direction') {
            if (numValue === 1) { // Turn left
              setRobotState(prev => ({
                ...prev,
                targetRotation: prev.rotation + Math.PI / 2,
                targetX: prev.x,
                targetY: prev.y,
                isAnimating: true
              }));
            } else if (numValue === 0) { // Straight
              setRobotState(prev => ({
                ...prev,
                targetRotation: prev.rotation,
                targetX: prev.x,
                targetY: prev.y,
                isAnimating: true
              }));
            }
          } else if (type === 'forward') {
            setRobotState(prev => {
              const angle = prev.rotation;
              return {
                ...prev,
                targetX: prev.x + Math.cos(angle) * numValue,
                targetY: prev.y + Math.sin(angle) * numValue,
                targetRotation: prev.rotation,
                isAnimating: true
              };
            });
          }
          
          isAnimating.current = true;
          startTime.current = performance.now();
          animationFrame.current = requestAnimationFrame(animate);

          // Wait for animation to complete
          const checkAnimation = () => {
            if (isAnimating.current) {
              setTimeout(checkAnimation, 50);
            } else {
              resolve();
            }
          };
          setTimeout(checkAnimation, 50);
          break;
        }
        default:
          resolve();
      }
    });
  };

  const executeLabel = async (label: Label, labels: Map<string, Label>, executedLabels: Set<string> = new Set()) => {
    if (executedLabels.has(label.name)) {
      throw new Error(`Recursive call detected: ${label.name}`);
    }
    
    executedLabels.add(label.name);
    
    for (const instruction of label.instructions) {
      if (instruction.startsWith('jal')) {
        const targetLabel = instruction.split(' ')[1];
        const target = labels.get(targetLabel);
        if (target) {
          await executeLabel(target, labels, new Set(executedLabels));
        } else {
          throw new Error(`Label not found: ${targetLabel}`);
        }
      } else {
        await executeInstruction(instruction);
        // Add a small delay between instructions
        await new Promise(resolve => setTimeout(resolve, INSTRUCTION_DELAY));
      }
    }
  };

  const handleCompile = async () => {
    setCompilationStatus({ status: 'compiling', message: 'Analyzing code structure...' });
    
    try {
      // Call the backend API to compile to Arduino C++
      const arduinoResponse = await fetch('http://localhost:8080/api/compile/arduino', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ code }),
      });

      if (!arduinoResponse.ok) {
        const errorData = await arduinoResponse.json();
        throw new Error(`Backend error: ${errorData.error}`);
      }

      const arduinoData = await arduinoResponse.json();
      setCppCode(arduinoData.output);
      console.log('Generated Arduino C++ code:', arduinoData.output);

      // Call the backend API to compile to IR
      const irResponse = await fetch('http://localhost:8080/api/compile', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ code }),
      });

      if (!irResponse.ok) {
        const errorData = await irResponse.json();
        throw new Error(`Backend error: ${errorData.error}`);
      }

      const irData = await irResponse.json();
      console.log('Generated IR:', irData.output);

      // Reset robot state
      setRobotState({
        x: 0,
        y: 0,
        rotation: 0,
        targetX: 0,
        targetY: 0,
        targetRotation: 0,
        isAnimating: false,
        currentCommand: ''
      });
      
      // Clear any existing animation
      if (animationFrame.current) {
        cancelAnimationFrame(animationFrame.current);
      }
      isAnimating.current = false;
      
      // Parse code first
      const labels = parseLabels(code);
      
      setCompilationStatus({ status: 'compiling', message: 'Validating program structure...' });
      await new Promise(resolve => setTimeout(resolve, 400));
      
      // Validate main exists
      const mainLabel = labels.get('main');
      if (!mainLabel) {
        throw new Error('Program must contain a main label');
      }

      setCompilationStatus({ status: 'compiling', message: 'Executing program...' });
      await new Promise(resolve => setTimeout(resolve, 400));
      
      // Execute the program
      await executeLabel(mainLabel, labels);
      
      setCompilationStatus({ 
        status: 'success', 
        message: `Program compiled and executed successfully ✓`
      });
    } catch (error) {
      setCompilationStatus({ 
        status: 'error', 
        message: error instanceof Error ? error.message : 'Error compiling code' 
      });
    }
  };

  const handleCodeChange = (value: string) => {
    setCode(value);
    setCompilationStatus({ status: 'idle', message: 'Ready to compile' });
  };
    const extensions = [
    myLanguage, // Add your custom language mode
	    autocompletion({ override: [customCompletionSource] }), // Correct function name!

    EditorView.theme({
      '&': {
        color: '#abb2bf', // Example default text color for dark theme
        backgroundColor: '#282c34', // Example dark background color
      },
      '.cm-content': {
        caretColor: '#c6c6c6',
      },
      '.cm-line': {
        paddingLeft: '10px',
        paddingRight: '10px',
      },
      '&.cm-focused .cm-selectionBackground, ::selection': {
        backgroundColor: '#3e4451',
      },
      '.cm-gutters': {
        backgroundColor: '#282c34',
        color: '#6b829e',
        border: 'none',
      },
      '.cm-activeLine': {
        backgroundColor: '#363b46',
      },
      '.cm-activeLineGutter': {
        backgroundColor: '#363b46',
      },
      ...myHighlightStyle.module, // Apply your custom highlight style
    }),
  ];

  const handleDownload = () => {
    const element = document.createElement('a');
    const file = new Blob([cppCode], { type: 'text/plain' });
    element.href = URL.createObjectURL(file);
    element.download = 'robot_sketch.ino';
    document.body.appendChild(element);
    element.click();
    document.body.removeChild(element);
  };

  const handleUpload = async () => {
    setUploadStatus({ status: 'uploading', message: 'Uploading to Arduino...' });
    
    try {
      const response = await fetch('http://localhost:8080/api/upload/arduino', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          code: cppCode,
          port: '/dev/ttyUSB0' // You might want to make this configurable
        }),
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.error);
      }

      const data = await response.json();
      setUploadStatus({ 
        status: 'success', 
        message: 'Upload successful! ✓' 
      });
    } catch (error) {
      setUploadStatus({ 
        status: 'error', 
        message: error instanceof Error ? error.message : 'Error uploading to Arduino' 
      });
    }
  };

  return (
      <div className="flex w-full h-full overflow-visible">
      <div className="w-1/2 p-4 bg-white flex flex-col h-screen">
	  <div className="h-[60px] bg-[#1e1e1e] rounded-t-lg border-b border-[#333] flex items-center px-4 gap-4 mb-0">
          <button
            onClick={handleCompile}
            disabled={compilationStatus.status === 'compiling' || robotState.isAnimating}
            className={`px-6 py-2 rounded-md font-medium transition-all duration-200 flex items-center gap-2
              ${(compilationStatus.status === 'compiling' || robotState.isAnimating)
                ? 'bg-accent/50 cursor-not-allowed'
                : 'bg-accent hover:bg-accent/80'}`}
          >
            {compilationStatus.status === 'compiling' ? (
              <>
                <div className="w-4 h-4 border-2 border-white/20 border-t-white rounded-full animate-spin" />
                Compiling...
              </>
            ) : (
              'Compile'
            )}
          </button>
          <div className="flex items-center gap-2">
            <div className={`w-2 h-2 rounded-full ${
              compilationStatus.status === 'idle' ? 'bg-gray-400' :
              compilationStatus.status === 'compiling' ? 'bg-yellow-400' :
              compilationStatus.status === 'success' ? 'bg-green-400' :
              'bg-red-400'
            }`} />
            <span className="text-sm text-gray-300">{compilationStatus.message}</span>
          </div>
          {robotState.currentCommand && (
            <div className="ml-auto text-sm text-gray-300">
              Executing: <span className="text-white font-mono">{robotState.currentCommand}</span>
            </div>
          )}
        </div>

        <div className="flex-1 flex flex-col h-full">
          <div className="h-full">
           { <div className="h-[30px] bg-[#1e1e1e] border-b border-[#333] flex items-center px-4">
              <span className="text-sm text-gray-300">Assembly Code</span>
            </div>}
            <CodeMirror
              value={code}
              height="calc(100% - 30px)"

              theme="dark"

              //extensions={[highlighting]}
              extensions={extensions}
              onChange={handleCodeChange}
              className="rounded-b-lg"
            />
          </div>
          <div className="flex-1 relative">
            {<div className="h-[30px] bg-[#1e1e1e] border-b border-[#333] flex items-center px-4 justify-between">
              <span className="text-sm text-gray-300">Generated Arduino C++</span>
              <div className="flex items-center gap-2">
                <button
                  onClick={() => setShowCppCode(!showCppCode)}
                  className="text-sm text-gray-300 hover:text-white transition-colors"
                >
                  {showCppCode ? 'Hide' : 'Show'} Code
                </button>
                <button
                  onClick={handleDownload}
                  className="px-3 py-1 text-sm rounded-md bg-blue-600 hover:bg-blue-700 transition-colors"
                >
                  Download
                </button>
                <button
                  onClick={handleUpload}
                  disabled={uploadStatus.status === 'uploading' || compilationStatus.status !== 'success'}
                  className={`px-3 py-1 text-sm rounded-md transition-colors flex items-center gap-2
                    ${uploadStatus.status === 'uploading' ? 'bg-yellow-600 cursor-not-allowed' :
                      compilationStatus.status !== 'success' ? 'bg-gray-600 cursor-not-allowed' :
                      'bg-green-600 hover:bg-green-700'}`}
                >
                  {uploadStatus.status === 'uploading' ? (
                    <>
                      <div className="w-3 h-3 border-2 border-white/20 border-t-white rounded-full animate-spin" />
                      Uploading...
                    </>
                  ) : (
                    'Upload to Arduino'
                  )}
                </button>
                <div className={`w-2 h-2 rounded-full ${
                  uploadStatus.status === 'idle' ? 'bg-gray-400' :
                  uploadStatus.status === 'uploading' ? 'bg-yellow-400' :
                  uploadStatus.status === 'success' ? 'bg-green-400' :
                  'bg-red-400'
                }`} />
              </div>
            </div>}
            {compilationStatus.status === 'compiling' && <LoadingBar />}
            {showCppCode && (
              <CodeMirror
                value={cppCode}
                height="calc(100% - 30px)"
                theme="dark"
                extensions={[cpp()]}
                readOnly={true}
                className="rounded-b-lg"
                basicSetup={{
                  lineNumbers: true,
                  highlightActiveLine: false,
                  highlightActiveLineGutter: false,
                  foldGutter: false,
                  dropCursor: false,
                  allowMultipleSelections: false,
                  indentOnInput: false,
                  syntaxHighlighting: true,
                }}
              />
            )}
            <HelpMenu />
          </div>
        </div>
      </div>


      <div className="w-1/2 bg-primary">
        <Canvas camera={{ position: [10, 10, 10] }}>
          <ambientLight intensity={0.5} />
          <pointLight position={[10, 10, 10]} />
          <Robot 
            position={[robotState.x, 0, robotState.y]} 
            rotation={robotState.rotation}
          />
          <gridHelper args={[20, 20]} />
          <OrbitControls />
        </Canvas>
      </div>
    </div>
  );
} 
