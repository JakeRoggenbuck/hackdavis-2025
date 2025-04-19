import IDE from '@/components/IDE';

export default function Home() {
  return (
    <main className="min-h-screen bg-black">
      <div className="h-screen p-4">
        {/* Header */}
        <header className="flex items-center justify-end space-x-4 mb-4">
          <button className="px-4 py-1.5 text-sm text-gray-400 hover:text-white transition-colors">
            Documentation
          </button>
          <button className="px-4 py-1.5 text-sm bg-white/10 hover:bg-white/20 text-white rounded-md transition-colors">
            Sign In
          </button>
        </header>
        
        {/* IDE Component */}
        <IDE />
      </div>
    </main>
  );
}
