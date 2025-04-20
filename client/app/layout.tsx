import "./globals.css";
import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "Robot Visualizer",
  description: "A high-tech robot visualization and programming interface",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
