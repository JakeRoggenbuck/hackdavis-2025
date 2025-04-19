import { useState, useEffect, useRef } from 'react';
import Editor from '@monaco-editor/react';
import { Terminal } from 'xterm';
import { FitAddon } from 'xterm-addon-fit';
import { WebLinksAddon } from 'xterm-addon-web-links';
import { motion } from 'framer-motion';
import 'xterm/css/xterm.css';

function App() {
  const [code, setCode] = useState('// Write your code here...');
  const terminalRef = useRef<HTMLDivElement>(null);
  const terminal = useRef<Terminal | null>(null);
  const fitAddon = useRef<FitAddon | null>(null);

  useEffect(() => {
    if (terminalRef.current) {
      terminal.current = new Terminal({
        theme: {
          background: '#2d2d2d',
          foreground: '#ffffff',
        },
        cursorBlink: true,
      });

      fitAddon.current = new FitAddon();
      terminal.current.loadAddon(fitAddon.current);
      terminal.current.loadAddon(new WebLinksAddon());
      terminal.current.open(terminalRef.current);
      fitAddon.current.fit();

      // Add some initial terminal output
      terminal.current.writeln('Welcome to the Code Editor!');
      terminal.current.writeln('Type your code and click "Run" to execute it.');
      terminal.current.writeln('');

      const handleResize = () => {
        fitAddon.current?.fit();
      };

      window.addEventListener('resize', handleResize);
      return () => {
        window.removeEventListener('resize', handleResize);
        terminal.current?.dispose();
      };
    }
  }, []);

  const handleRun = () => {
    if (terminal.current) {
      terminal.current.writeln('\r\n$ Running code...');
      terminal.current.writeln('> ' + code);
      terminal.current.writeln('Code execution completed.');
      terminal.current.writeln('');
    }
  };

  const handleClear = () => {
    if (terminal.current) {
      terminal.current.clear();
      terminal.current.writeln('Terminal cleared.');
      terminal.current.writeln('');
    }
  };

  const handleDownload = () => {
    const blob = new Blob([code], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'code.txt';
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  };

  return (
    <div className="min-h-screen p-4">
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        className="max-w-7xl mx-auto"
      >
        <h1 className="text-3xl font-bold mb-6 text-center">Code Editor</h1>
        
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-4 mb-4">
          <motion.div
            className="editor-panel h-[500px]"
            whileHover={{ scale: 1.01 }}
            transition={{ duration: 0.2 }}
          >
            <Editor
              height="100%"
              defaultLanguage="javascript"
              value={code}
              onChange={(value) => setCode(value || '')}
              theme="vs-dark"
              options={{
                minimap: { enabled: false },
                fontSize: 14,
                lineNumbers: 'on',
                roundedSelection: false,
                scrollBeyondLastLine: false,
                automaticLayout: true,
              }}
            />
          </motion.div>

          <motion.div
            className="terminal-panel h-[500px]"
            whileHover={{ scale: 1.01 }}
            transition={{ duration: 0.2 }}
          >
            <div ref={terminalRef} className="h-full" />
          </motion.div>
        </div>

        <div className="flex justify-center gap-4">
          <motion.button
            whileHover={{ scale: 1.05 }}
            whileTap={{ scale: 0.95 }}
            className="control-button"
            onClick={handleRun}
          >
            Run
          </motion.button>
          <motion.button
            whileHover={{ scale: 1.05 }}
            whileTap={{ scale: 0.95 }}
            className="control-button"
            onClick={handleClear}
          >
            Clear
          </motion.button>
          <motion.button
            whileHover={{ scale: 1.05 }}
            whileTap={{ scale: 0.95 }}
            className="control-button"
            onClick={handleDownload}
          >
            Download
          </motion.button>
        </div>
      </motion.div>
    </div>
  );
}

export default App;
