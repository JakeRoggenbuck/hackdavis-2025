'use client'

import { useState } from 'react'
import Editor from '@monaco-editor/react'
import { Card, CardHeader, CardTitle, CardContent } from './ui/card'

export default function CodeEditor() {
  const [code, setCode] = useState(`// Welcome to CodeAI
// Start coding here...`)

  return (
    <Card className="h-full border-0">
      <CardHeader className="border-b">
        <CardTitle>Editor</CardTitle>
      </CardHeader>
      <CardContent className="p-0">
        <Editor
          height="calc(100vh - 200px)"
          defaultLanguage="javascript"
          theme="vs-dark"
          value={code}
          onChange={(value) => setCode(value || '')}
          options={{
            minimap: { enabled: false },
            fontSize: 14,
            lineNumbers: 'on',
            roundedSelection: false,
            scrollBeyondLastLine: false,
            automaticLayout: true
          }}
        />
      </CardContent>
    </Card>
  )
} 