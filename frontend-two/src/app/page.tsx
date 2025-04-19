import { ResizablePanelGroup, ResizablePanel } from "@/components/ui/resizable"
import { Card } from "@/components/ui/card"
import CodeEditor from "@/components/code-editor"
import Compiler from "@/components/compiler"

export default function Home() {
  return (
    <main className="min-h-screen bg-background">
      <div className="container mx-auto p-4">
        <h1 className="text-4xl font-bold mb-6 text-foreground">Code Editor</h1>
        <ResizablePanelGroup direction="horizontal" className="min-h-[600px] rounded-lg border">
          <ResizablePanel defaultSize={50}>
            <Card className="h-full rounded-none border-0">
              <CodeEditor />
            </Card>
          </ResizablePanel>
          <ResizablePanel defaultSize={50}>
            <Card className="h-full rounded-none border-0">
              <Compiler />
            </Card>
          </ResizablePanel>
        </ResizablePanelGroup>
      </div>
    </main>
  )
}
