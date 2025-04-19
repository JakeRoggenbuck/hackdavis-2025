import { Message } from '../types/chat';
import ReactMarkdown from 'react-markdown';
import { motion } from 'framer-motion';

interface ChatMessageProps {
  message: Message;
}

export default function ChatMessage({ message }: ChatMessageProps) {
  const isUser = message.role === 'user';

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      className={`flex gap-4 p-4 ${
        isUser ? 'bg-chat-bg' : 'bg-chat-input'
      }`}
    >
      <div className="w-8 h-8 rounded-full bg-gray-600 flex items-center justify-center">
        {isUser ? 'U' : 'AI'}
      </div>
      <div className="flex-1 prose prose-invert max-w-none">
        <ReactMarkdown
          components={{
            code: ({ node, ...props }) => (
              <code
                className="bg-gray-700 rounded px-1 py-0.5"
                {...props}
              />
            ),
            pre: ({ node, ...props }) => (
              <pre
                className="bg-gray-800 rounded p-4 my-2 overflow-x-auto"
                {...props}
              />
            ),
          }}
        >
          {message.content}
        </ReactMarkdown>
      </div>
    </motion.div>
  );
} 