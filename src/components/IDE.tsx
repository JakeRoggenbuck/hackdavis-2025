'use client';

import { useState, useEffect, useRef } from 'react';
import dynamic from 'next/dynamic';
import { FaSpinner, FaCheck } from 'react-icons/fa';

// Dynamically import Monaco editor with no SSR
const Editor = dynamic(() => import('@monaco-editor/react'), {
  ssr: false,
  loading: () => <div className="w-full h-full bg-gray-800 flex items-center justify-center">Loading editor...</div>
});

// Create a client-side only terminal component
const TerminalComponent = dynamic(() => Promise.resolve(({ terminalRef }: { terminalRef: React.RefObject<HTMLDivElement | null> }) => {
  const terminal = useRef<any>(null);
  const fitAddon = useRef<any>(null);

  useEffect(() => {
    if (terminalRef.current) {
      // Dynamically import XTerm.js and its addons
      Promise.all([
        import('@xterm/xterm'),
        import('@xterm/addon-fit'),
        import('@xterm/addon-web-links')
      ]).then(([xterm, fit, webLinks]) => {
        // Initialize terminal
        terminal.current = new xterm.Terminal({
          cursorBlink: true,
          theme: {
            background: '#1e1e1e',
            foreground: '#ffffff',
          },
        });

        fitAddon.current = new fit.FitAddon();
        terminal.current.loadAddon(fitAddon.current);
        terminal.current.loadAddon(new webLinks.WebLinksAddon());

        terminal.current.open(terminalRef.current);
        fitAddon.current.fit();

        // Add welcome message
        terminal.current.writeln('Welcome to the IDE Terminal!');
        terminal.current.writeln('Type your commands here...');

        // Handle window resize
        const handleResize = () => {
          fitAddon.current?.fit();
        };

        window.addEventListener('resize', handleResize);

        return () => {
          window.removeEventListener('resize', handleResize);
          terminal.current?.dispose();
        };
      });
    }
  }, []);

  return (
    <div
      ref={terminalRef}
      className="h-full bg-gray-800 rounded-lg overflow-hidden"
    />
  );
}), {
  ssr: false,
  loading: () => <div className="w-full h-full bg-gray-800 flex items-center justify-center">Loading terminal...</div>
});

const IDE = () => {
  const [code, setCode] = useState<string>('// Start coding here...');
  const [isCompiling, setIsCompiling] = useState<boolean>(false);
  const [isCompiled, setIsCompiled] = useState<boolean>(false);
  const terminalRef = useRef<HTMLDivElement>(null);

  const handleEditorChange = (value: string | undefined) => {
    if (value !== undefined) {
      setCode(value);
    }
  };

  const simulateCompilation = () => {
    setIsCompiling(true);
    setIsCompiled(false);
    
    // Simulate compilation time
    setTimeout(() => {
      setIsCompiling(false);
      setIsCompiled(true);
      
      // Clear the success state after 2 seconds
      setTimeout(() => {
        setIsCompiled(false);
      }, 2000);
    }, 1500);
  };

  return (
    <div className="flex flex-col h-screen bg-gray-900 text-white">
      {/* Header */}
      <div className="flex items-center justify-between p-4 bg-gray-800 border-b border-gray-700">
        <h1 className="text-xl font-bold">Custom IDE</h1>
        <div className="flex items-center space-x-4">
          <button
            onClick={simulateCompilation}
            className="px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded-md transition-colors"
          >
            Compile
          </button>
          <div className="flex items-center space-x-2">
            {isCompiling ? (
              <FaSpinner className="animate-spin text-blue-500" />
            ) : isCompiled ? (
              <FaCheck className="text-green-500" />
            ) : null}
          </div>
        </div>
      </div>

      {/* Main Content */}
      <div className="flex flex-1 overflow-hidden">
        {/* Editor */}
        <div className="w-1/2 border-r border-gray-700">
          <Editor
            height="100%"
            defaultLanguage="javascript"
            theme="vs-dark"
            value={code}
            onChange={handleEditorChange}
            options={{
              minimap: { enabled: true },
              fontSize: 14,
              wordWrap: 'on',
              automaticLayout: true,
            }}
          />
        </div>

        {/* Terminal */}
        <div className="w-1/2 p-4">
          <TerminalComponent terminalRef={terminalRef} />
        </div>
      </div>
    </div>
  );
};

export default IDE; 