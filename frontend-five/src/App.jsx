import { useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'

function App() {
  return (
    <div className="min-h-screen bg-gray-100 py-6 flex flex-col justify-center sm:py-12">
      <div className="relative py-3 sm:max-w-xl sm:mx-auto">
        <div className="relative px-4 py-10 bg-white shadow-lg sm:rounded-3xl sm:p-20">
          <div className="max-w-md mx-auto">
            <div className="divide-y divide-gray-200">
              <div className="py-8 text-base leading-6 space-y-4 text-gray-700 sm:text-lg sm:leading-7">
                <h1 className="text-3xl font-bold text-center text-indigo-600 mb-8">
                  Welcome to React + Tailwind
                </h1>
                <p className="text-center">
                  This is a simple React application styled with Tailwind CSS.
                </p>
                <div className="mt-8 flex justify-center">
                  <button className="px-4 py-2 bg-indigo-600 text-white rounded-md hover:bg-indigo-700 transition-colors">
                    Click Me
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}

export default App
