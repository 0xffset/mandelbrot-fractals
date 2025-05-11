import React from 'react';
import './Controls.css';

const Controls = ({ params, onParamChange, paintModes, isRendering }) => {
  // Convert PaintMode enum to array of options for select
  const paintModeOptions = Object.keys(paintModes).map(key => ({
    value: paintModes[key],
    label: key
  }));

  const handleInputChange = (e) => {
    const { name, value, type } = e.target;
    
    // Convert string values to correct types
    let parsedValue;
    if (type === 'number' || type === 'range') {
      parsedValue = name === 'threads' || name === 'iterations' || name === 'samples' || 
                    name === 'width' || name === 'height' 
                    ? parseInt(value, 10) 
                    : parseFloat(value);
    } else if (name === 'paintMode') {
      parsedValue = parseInt(value, 10);
    } else {
      parsedValue = value;
    }
    
    onParamChange(name, parsedValue);
  };

  return (
    <div className="controls">
      <h2>Controls</h2>
      
      <div className="control-group">
        <label>
          Position X:
          <div className="input-with-value">
            <input
              type="range"
              name="posX"
              min={-1}
              max={1}
              step="0.05"
              value={params.posX}
              onChange={handleInputChange}
              disabled={isRendering}
            />
            <input
              type="number"
              name="posX"
              value={params.posX}
              onChange={handleInputChange}
              step="0.05"
              disabled={isRendering}
            />
          </div>
        </label>
      </div>
      
      <div className="control-group">
        <label>
          Position Y:
          <div className="input-with-value">
            <input
              type="range"
              name="posY"
              min={-1}
              max={1}
              step="0.05"
              value={params.posY}
              onChange={handleInputChange}
              disabled={isRendering}
            />
            <input
              type="number"
              name="posY"
              value={params.posY}
              onChange={handleInputChange}
              step="0.05"
              disabled={isRendering}
            />
          </div>
        </label>
      </div>
      
      <div className="control-group">
        <label>
          Size:
          <div className="input-with-value">
            <input
              type="range"
              name="size"
              min="0.01"
              max="6"
              step="0.01"
              value={params.size}
              onChange={handleInputChange}
              disabled={isRendering}
            />
            <input
              type="number"
              name="size"
              value={params.size}
              onChange={handleInputChange}
              step="0.01"
              min="0.01"
              disabled={isRendering}
            />
          </div>
        </label>
      </div>
      
      <div className="control-group">
        <label>
          Iterations:
          <div className="input-with-value">
            <input
              type="range"
              name="iterations"
              min="100"
              max="5000"
              step="100"
              value={params.iterations}
              onChange={handleInputChange}
              disabled={isRendering}
            />
            <input
              type="number"
              name="iterations"
              value={params.iterations}
              onChange={handleInputChange}
              min="100"
              step="100"
              disabled={isRendering}
            />
          </div>
        </label>
      </div>
      
      <div className="control-group">
        <label>
          Samples:
          <div className="input-with-value">
            <input
              type="range"
              name="samples"
              min="1"
              max="10"
              value={params.samples}
              onChange={handleInputChange}
              disabled={isRendering}
            />
            <input
              type="number"
              name="samples"
              value={params.samples}
              onChange={handleInputChange}
              min="1"
              max="10"
              disabled={isRendering}
            />
          </div>
        </label>
      </div>
      
      <div className="control-group">
        <label>
          Width:
          <div className="input-with-value">
            <input
              type="range"
              name="width"
              min="128"
              max="1920"
              step="64"
              value={params.width}
              onChange={handleInputChange}
              disabled={isRendering}
            />
            <input
              type="number"
              name="width"
              value={params.width}
              onChange={handleInputChange}
              min="128"
              step="64"
              disabled={isRendering}
            />
          </div>
        </label>
      </div>
      
      <div className="control-group">
        <label>
          Height:
          <div className="input-with-value">
            <input
              type="range"
              name="height"
              min="128"
              max="1080"
              step="64"
              value={params.height}
              onChange={handleInputChange}
              disabled={isRendering}
            />
            <input
              type="number"
              name="height"
              value={params.height}
              onChange={handleInputChange}
              min="128"
              step="64"
              disabled={isRendering}
            />
          </div>
        </label>
      </div>
      
      <div className="control-group">
        <label>
          Threads:
          <div className="input-with-value">
            <input
              type="range"
              name="threads"
              min="1"
              max="32"
              value={params.threads}
              onChange={handleInputChange}
              disabled={isRendering}
            />
            <input
              type="number"
              name="threads"
              value={params.threads}
              onChange={handleInputChange}
              min="1"
              max="32"
              disabled={isRendering}
            />
          </div>
        </label>
      </div>
      
      <div className="control-group">
        <label>
          Paint Mode:
          <select
            name="paintMode"
            value={params.paintMode}
            onChange={handleInputChange}
            disabled={isRendering}
          >
            {paintModeOptions.map(option => (
              <option key={option.label} value={option.value}>
                {option.label}
              </option>
            ))}
          </select>
        </label>
      </div>
    </div>
  );
};

export default Controls;