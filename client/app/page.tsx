"use client";

import dynamic from "next/dynamic";

const RobotVisualizer = dynamic(() => import("./components/RobotVisualizer"), {
  ssr: false,
});

export default function Home() {
  return (
    <main className="flex min-h-screen bg-primary text-white">
      <RobotVisualizer />
    </main>
  );
}
