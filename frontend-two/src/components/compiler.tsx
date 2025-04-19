'use client'

import { Card, CardHeader, CardTitle, CardContent } from './ui/card'
import { Button } from './ui/button'

export default function Compiler() {
  return (
    <Card className="h-full border-0">
      <CardHeader className="border-b flex flex-row items-center justify-between">
        <CardTitle>Output</CardTitle>
        <Button variant="outline" size="sm">
          Run Code
        </Button>
      </CardHeader>
      <CardContent className="p-4">
        <div className="font-mono text-sm bg-muted p-4 rounded-lg h-[calc(100vh-250px)] overflow-auto">
          <p className="text-muted-foreground">Output will appear here...</p>
        </div>
      </CardContent>
    </Card>
  )
} 