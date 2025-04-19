import { ChatSession } from '../types/chat';
import { PlusIcon } from '@heroicons/react/24/outline';

interface SidebarProps {
  sessions: ChatSession[];
  currentSessionId: string | null;
  onNewSession: () => void;
  onSelectSession: (sessionId: string) => void;
}

export default function Sidebar({
  sessions,
  currentSessionId,
  onNewSession,
  onSelectSession,
}: SidebarProps) {
  return (
    <div className="w-64 h-screen bg-chat-sidebar p-4 flex flex-col">
      <button
        onClick={onNewSession}
        className="flex items-center gap-2 p-3 rounded-md border border-chat-border hover:bg-chat-input transition-colors"
      >
        <PlusIcon className="w-5 h-5" />
        <span>New Chat</span>
      </button>

      <div className="mt-4 flex-1 overflow-y-auto">
        {sessions.map((session) => (
          <button
            key={session.id}
            onClick={() => onSelectSession(session.id)}
            className={`w-full p-3 rounded-md text-left mb-2 transition-colors ${
              session.id === currentSessionId
                ? 'bg-chat-input'
                : 'hover:bg-chat-input'
            }`}
          >
            <div className="truncate">{session.title}</div>
            <div className="text-xs text-gray-400">
              {new Date(session.lastUpdated).toLocaleDateString()}
            </div>
          </button>
        ))}
      </div>
    </div>
  );
} 