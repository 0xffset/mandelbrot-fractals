import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';

// Import our WebAssembly module
import("mandelbrot-wasm").then(module => {
  // Initialize the React application
  const root = ReactDOM.createRoot(document.getElementById('app'));
  root.render(
    <React.StrictMode>
      <App wasmModule={module} />
    </React.StrictMode>
  );
}).catch(e => console.error("Error importing WASM module:", e));