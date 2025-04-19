'use client';

import { useState, useEffect, useRef } from 'react';
import Editor from '@monaco-editor/react';
import { Terminal } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import { WebLinksAddon } from '@xterm/addon-web-links';
import { FaSpinner, FaCheck } from 'react-icons/fa';

export default function IDE() {
  const [code, setCode] = useState('// Start coding here...');
  const [isCompiling, setIsCompiling] = useState(false);
  const [isCompiled, setIsCompiled] = useState(false);
  const terminalRef = useRef<HTMLDivElement>(null);
  const terminal = useRef<Terminal | null>(null);
  const fitAddon = useRef<FitAddon | null>(null);

  useEffect(() => {
    if (terminalRef.current && !terminal.current) {
      // Initialize terminal
      terminal.current = new Terminal({
        cursorBlink: true,
        theme: {
          background: '#1e1e1e',
          foreground: '#ffffff',
        },
      });

      // Initialize addons
      fitAddon.current = new FitAddon();
      const webLinksAddon = new WebLinksAddon();

      // Load addons
      terminal.current.loadAddon(fitAddon.current);
      terminal.current.loadAddon(webLinksAddon);

      // Open terminal
      terminal.current.open(terminalRef.current);
      fitAddon.current.fit();

      // Handle resize
      const resizeObserver = new ResizeObserver(() => {
        fitAddon.current?.fit();
      });
      resizeObserver.observe(terminalRef.current);

      return () => {
        resizeObserver.disconnect();
        terminal.current?.dispose();
      };
    }
  }, []);

  const handleCompile = () => {
    setIsCompiling(true);
    setIsCompiled(false);

    // Simulate compilation
    setTimeout(() => {
      setIsCompiling(false);
      setIsCompiled(true);
      
      // Write to terminal
      if (terminal.current) {
        terminal.current.write('\r\n$ Compilation successful!\r\n');
      }
    }, 2000);
  };

  return (
    <div className="flex h-screen bg-gray-900 text-white">
      {/* Editor Section */}
      <div className="w-1/2 h-full">
        <div className="h-full flex flex-col">
          <div className="p-4 bg-gray-800 flex justify-between items-center">
            <h2 className="text-xl font-bold">Editor</h2>
            <button
              onClick={handleCompile}
              className="px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded-md transition-colors"
            >
              Compile
            </button>
          </div>
          <div className="flex-1">
            <Editor
              height="100%"
              defaultLanguage="javascript"
              value={code}
              onChange={(value) => setCode(value || '')}
              theme="vs-dark"
              options={{
                minimap: { enabled: true },
                fontSize: 14,
                lineNumbers: 'on',
                roundedSelection: false,
                scrollBeyondLastLine: false,
                readOnly: false,
              }}
            />
          </div>
        </div>
      </div>

      {/* Terminal Section */}
      <div className="w-1/2 h-full flex flex-col">
        <div className="p-4 bg-gray-800 flex justify-between items-center">
          <h2 className="text-xl font-bold">Terminal</h2>
          <div className="flex items-center space-x-2">
            {isCompiling && (
              <div className="flex items-center text-yellow-400">
                <FaSpinner className="animate-spin mr-2" />
                <span>Compiling...</span>
              </div>
            )}
            {isCompiled && (
              <div className="flex items-center text-green-400">
                <FaCheck className="mr-2" />
                <span>Compiled</span>
              </div>
            )}
          </div>
        </div>
        <div ref={terminalRef} className="flex-1 bg-gray-900 p-2" />
      </div>
    </div>
  );
} 