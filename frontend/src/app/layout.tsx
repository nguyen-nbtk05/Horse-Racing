import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "Horse Racing",
  description: "Horse Racing Tournament Management System",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="vi">
      <body>{children}</body>
    </html>
  );
}