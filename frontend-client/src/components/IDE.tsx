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
  const [code, setCode] = useState<string>('// Welcome to CodeAI\n// Start coding here...');
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
    <div className="flex flex-col h-[calc(100vh-7rem)] border border-white/10 rounded-sm overflow-hidden">
      {/* Header */}
      <div className="flex items-center justify-between px-6 py-3 bg-[#1e1e1e] border-b border-white/10">
        <h1 className="text-xl font-serif text-white">Code Editor</h1>
        <button
          onClick={simulateCompilation}
          className="px-4 py-1.5 text-sm bg-white/10 hover:bg-white/20 text-white rounded-sm transition-colors"
        >
          {isCompiling ? (
            <div className="flex items-center space-x-2">
              <FaSpinner className="animate-spin" />
              <span>Compiling...</span>
            </div>
          ) : (
            <div className="flex items-center space-x-2">
              <span>Compile & Run</span>
              {isCompiled && <FaCheck className="text-green-500" />}
            </div>
          )}
        </button>
      </div>

      {/* Main Content */}
      <div className="flex flex-1 divide-x divide-white/10">
        {/* Editor */}
        <div className="w-1/2">
          <Editor
            height="100%"
            defaultLanguage="javascript"
            theme="vs-dark"
            value={code}
            onChange={handleEditorChange}
            options={{
              minimap: { enabled: false },
              fontSize: 14,
              lineHeight: 1.6,
              padding: { top: 12 },
              wordWrap: 'on',
              automaticLayout: true,
              scrollBeyondLastLine: false,
              renderLineHighlight: 'none',
              folding: false,
              lineNumbers: 'on',
              lineDecorationsWidth: 0,
              glyphMargin: false,
              scrollbar: {
                vertical: 'visible',
                horizontal: 'visible',
                verticalScrollbarSize: 8,
                horizontalScrollbarSize: 8
              }
            }}
          />
        </div>

        {/* Terminal */}
        <div className="w-1/2">
          <div className="h-full">
            <TerminalComponent terminalRef={terminalRef} />
          </div>
        </div>
      </div>
    </div>
  );
};

export default IDE; 