import React, { useState, useEffect, useRef } from 'react';
import Controls from './Controls';
import './App.css';

const App = ({ wasmModule }) => {
  const canvasRef = useRef(null);
  const mandelbrotRef = useRef(null);

  const [params, setParams] = useState({
    posX: 0,
    posY: 0,
    size: 4.0,
    iterations: 1000,
    samples: 5,
    width: 512,
    height: 512,
    threads: 16,
    paintMode: wasmModule.PaintMode.HSL3
  });

  const [renderTime, setRenderTime] = useState(0);
  const [isRendering, setIsRendering] = useState(false);
  const [mouseDown, setMouseDown] = useState(false);
  const [zoomStart, setZoomStart] = useState(null);
  const [zoomRect, setZoomRect] = useState(null);

  useEffect(() => {
    if (wasmModule && canvasRef.current) {
      try {
        mandelbrotRef.current = new wasmModule.Mandelbrot(
          "mandelbrot-canvas",
          params.width,
          params.height,
          params.posX,
          params.posY,
          params.size,
          params.iterations,
          params.samples,
          params.paintMode
        );
        handleRender();
      } catch (err) {
        console.error("Error initializing Mandelbrot:", err);
      }
    }
    return () => {
      mandelbrotRef.current = null;
    };
  }, [wasmModule]);

  const handleParamChange = (name, value) => {
    setParams(prev => ({ ...prev, [name]: value }));
    if (mandelbrotRef.current) {
      switch (name) {
        case 'posX': mandelbrotRef.current.set_pos_x(value); break;
        case 'posY': mandelbrotRef.current.set_pos_y(value); break;
        case 'size': mandelbrotRef.current.set_size(value); break;
        case 'iterations': mandelbrotRef.current.set_max_iterations(value); break;
        case 'samples': mandelbrotRef.current.set_samples(value); break;
        case 'width': mandelbrotRef.current.set_width(value); break;
        case 'height': mandelbrotRef.current.set_height(value); break;
        case 'paintMode': mandelbrotRef.current.set_paint_mode(value); break;
        default: break;
      }
    }
  };

  const handleRender = async () => {
    if (!mandelbrotRef.current) return;
    setIsRendering(true);
    try {
      await mandelbrotRef.current.render_parallel(params.threads);
      setRenderTime(mandelbrotRef.current.get_render_time());
    } catch (err) {
      console.error("Error rendering Mandelbrot:", err);
    } finally {
      setIsRendering(false);
    }
  };

  const handleSaveImage = () => {
    if (!mandelbrotRef.current) return;
    try {
      mandelbrotRef.current.save_to_png();
    } catch (err) {
      console.error("Error saving image:", err);
    }
  };

  const handleMouseDown = (e) => {
    const rect = canvasRef.current.getBoundingClientRect();
    setZoomStart({
      x: e.clientX - rect.left,
      y: e.clientY - rect.top
    });
    setMouseDown(true);
  };

  const handleMouseMove = (e) => {
    if (!mouseDown || !zoomStart) return;
    const rect = canvasRef.current.getBoundingClientRect();
    const currentX = e.clientX - rect.left;
    const currentY = e.clientY - rect.top;
    setZoomRect({
      x: zoomStart.x,
      y: zoomStart.y,
      width: currentX - zoomStart.x,
      height: currentY - zoomStart.y
    });
  };

  const handleMouseUp = () => {
    if (!zoomRect || !canvasRef.current) {
      setMouseDown(false);
      setZoomRect(null);
      return;
    }

    const { x, y, width, height } = zoomRect;
    if (Math.abs(width) < 5 || Math.abs(height) < 5) {
      setMouseDown(false);
      setZoomRect(null);
      return;
    }

    // Calculate center of the zoom box in canvas coordinates
    const cx = x + width / 2;
    const cy = y + height / 2;

    // Compute scaling factor based on zoom box relative to canvas
    const scaleX = Math.abs(width) / params.width;
    const scaleY = Math.abs(height) / params.height;
    const scale = Math.max(scaleX, scaleY); // Use max to fit the zoom box entirely

    const newSize = params.size * scale;
    const aspect = params.height / params.width;

    const newPosX = params.posX + (cx / params.width - 0.5) * params.size;
    const newPosY = params.posY + (cy / params.height - 0.5) * params.size * aspect;


    setParams(prev => ({
      ...prev,
      posX: newPosX,
      posY: newPosY,
      size: newSize
    }));

    if (mandelbrotRef.current) {
      mandelbrotRef.current.set_pos_x(newPosX);
      mandelbrotRef.current.set_pos_y(newPosY);
      mandelbrotRef.current.set_size(newSize);
      handleRender();
    }

    setMouseDown(false);
    setZoomRect(null);
  };

  return (
    <div className="container">
      <h1>Mandelbrot Set Z = Z^2 + c</h1>
      <div className="app-layout">
          <Controls
          params={params}
          onParamChange={handleParamChange}
          paintModes={wasmModule.PaintMode}
          isRendering={isRendering}
        />
        <div className="canvas-container" style={{ position: 'relative', width: params.width, height: params.height }}>
          <canvas
            id="mandelbrot-canvas"
            ref={canvasRef}
            width={params.width}
            height={params.height}
            onMouseDown={handleMouseDown}
            onMouseMove={handleMouseMove}
            onMouseUp={handleMouseUp}
            style={{ cursor: 'crosshair', display: 'block' }}
          />

          {zoomRect && (
            <div
              style={{
                position: 'absolute',
                border: '1px solid red',
                backgroundColor: 'rgba(255, 0, 0, 0.2)',
                left: Math.min(zoomRect.x, zoomRect.x + zoomRect.width),
                top: Math.min(zoomRect.y, zoomRect.y + zoomRect.height),
                width: Math.abs(zoomRect.width),
                height: Math.abs(zoomRect.height),
                pointerEvents: 'none'
              }}
            />
          )}

          <div className="render-info">
            {isRendering ? <p>Rendering...</p> : <p>Render time: {renderTime.toFixed(3)} seconds</p>}
          </div>

          <div className="action-buttons">
            <button onClick={handleRender} disabled={isRendering}>Render</button>
            <button onClick={handleSaveImage} disabled={isRendering}>Save as PNG</button>
          </div>
        </div>

      
      </div>
    </div>
  );
};

export default App;