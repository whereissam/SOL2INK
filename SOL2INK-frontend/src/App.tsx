import { Link, Outlet } from "@tanstack/react-router";
import { Code, Github, BookOpen } from "lucide-react";
import ChatBotDialog from "./components/ChatBotDialog";

export function App() {
  return (
    <div className="min-h-screen bg-background">
      <nav className="border-b border-border bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
        <div className="container mx-auto px-4 py-3">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-6">
              <Link to="/" className="flex items-center gap-2 text-xl font-bold">
                <div className="p-1.5 bg-gradient-to-r from-blue-500 to-purple-600 rounded-lg">
                  <Code className="w-5 h-5 text-white" />
                </div>
                SOL2INK
              </Link>
              <div className="hidden md:flex items-center gap-4 text-sm">
                <Link 
                  to="/" 
                  className="text-foreground/60 hover:text-foreground transition-colors"
                >
                  Migration Assistant
                </Link>
                <Link 
                  to="/about" 
                  className="text-foreground/60 hover:text-foreground transition-colors"
                >
                  About
                </Link>
              </div>
            </div>
            <div className="flex items-center gap-3">
              <a
                href="https://github.com/your-repo/sol2ink"
                target="_blank"
                rel="noopener noreferrer"
                className="p-2 rounded-md hover:bg-accent transition-colors"
                title="View on GitHub"
              >
                <Github className="w-4 h-4" />
              </a>
              <a
                href="https://use.ink/"
                target="_blank"
                rel="noopener noreferrer"
                className="p-2 rounded-md hover:bg-accent transition-colors"
                title="ink! Documentation"
              >
                <BookOpen className="w-4 h-4" />
              </a>
            </div>
          </div>
        </div>
      </nav>
      <main>
        <Outlet />
      </main>
      <ChatBotDialog />
    </div>
  );
}

export default App;
