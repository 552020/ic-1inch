import React, { useState } from "react";
import { backend } from "../../declarations/backend";

function App() {
  const [greeting, setGreeting] = useState("");
  const [name, setName] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);
    try {
      const result = await backend.greet(name);
      setGreeting(result);
    } catch (err) {
      console.error("Error calling backend:", err);
      setError(
        "Failed to connect to backend. Make sure dfx is running and deployed."
      );
    } finally {
      setLoading(false);
    }
  };

  return (
    <main className="container mx-auto p-8">
      <div className="text-center">
        <h1 className="text-4xl font-bold mb-8">
          IC-1inch Limit Order Protocol
        </h1>
        <p className="text-gray-600 mb-8">
          MVP Implementation on Internet Computer
        </p>

        {error && (
          <div className="mb-8 p-4 bg-red-100 border border-red-400 text-red-700 rounded">
            {error}
          </div>
        )}

        <form
          onSubmit={(e) => {
            void handleSubmit(e);
          }}
          className="max-w-md mx-auto"
        >
          <div className="mb-4">
            <label htmlFor="name" className="block text-sm font-medium mb-2">
              Enter your name:
            </label>
            <input
              id="name"
              type="text"
              value={name}
              onChange={(e) => {
                setName(e.target.value);
              }}
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="Your name"
              disabled={loading}
            />
          </div>
          <button
            type="submit"
            disabled={loading}
            className="w-full bg-blue-500 text-white py-2 px-4 rounded-md hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50"
          >
            {loading ? "Loading..." : "Greet"}
          </button>
        </form>

        {greeting && (
          <div className="mt-8 p-4 bg-green-100 border border-green-400 text-green-700 rounded">
            {greeting}
          </div>
        )}
      </div>
    </main>
  );
}

export default App;
