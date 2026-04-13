import { useState, useEffect } from 'react'
import { useParams } from 'react-router-dom'
import { Canvas } from '@react-three/fiber'
import { OrbitControls, Environment } from '@react-three/drei'
import { scenes, render as renderApi } from '../api/client'

export default function RenderPage() {
  const { sceneId } = useParams<{ sceneId: string }>()
  
  const [scene, setScene] = useState<any>(null)
  const [cameraPosition, setCameraPosition] = useState<[number, number, number]>([0, 0, 3])
  const [cameraRotation, setCameraRotation] = useState<[number, number]>([0, 0])
  const [rendering, setRendering] = useState(false)
  const [lastRender, setLastRender] = useState<any>(null)

  useEffect(() => {
    if (sceneId) {
      loadScene()
    }
  }, [sceneId])

  const loadScene = async () => {
    if (!sceneId) return
    try {
      const response = await scenes.get(sceneId)
      setScene(response.data)
    } catch (error) {
      console.error('Failed to load scene:', error)
    }
  }

  const handleRender = async () => {
    if (!sceneId) return
    
    setRendering(true)
    try {
      const response = await renderApi.create({
        scene_id: sceneId,
        camera_position: cameraPosition,
        camera_rotation: cameraRotation,
        width: 512,
        height: 512,
      })
      setLastRender(response.data)
    } catch (error) {
      console.error('Failed to render:', error)
    } finally {
      setRendering(false)
    }
  }

  if (!scene) {
    return <div className="empty-state">Loading...</div>
  }

  return (
    <>
      <button 
        className="btn btn-secondary" 
        onClick={() => window.history.back()}
        style={{ marginBottom: '1.5rem' }}
      >
        ← Back to Scene
      </button>

      <h2 style={{ fontSize: '1.75rem', fontWeight: 600, marginBottom: '1rem' }}>
        Render Novel View - {scene.name}
      </h2>

      <div className="canvas-container">
        <Canvas camera={{ position: cameraPosition, fov: 50 }}>
          <ambientLight intensity={0.5} />
          <pointLight position={[10, 10, 10]} />
          <Environment preset="city" />
          
          {/* Placeholder 3D scene - in production would show NeRF reconstruction */}
          <mesh>
            <boxGeometry args={[2, 2, 2]} />
            <meshStandardMaterial color="#58a6ff" wireframe />
          </mesh>
          
          <OrbitControls 
            enableZoom={false}
            onChange={() => {}}
          />
        </Canvas>
      </div>

      <div className="controls-panel">
        <div className="control-group">
          <label className="control-label">Camera X</label>
          <input
            type="number"
            step="0.1"
            className="control-input"
            value={cameraPosition[0]}
            onChange={(e) => setCameraPosition([parseFloat(e.target.value), cameraPosition[1], cameraPosition[2]])}
          />
        </div>
        
        <div className="control-group">
          <label className="control-label">Camera Y</label>
          <input
            type="number"
            step="0.1"
            className="control-input"
            value={cameraPosition[1]}
            onChange={(e) => setCameraPosition([cameraPosition[0], parseFloat(e.target.value), cameraPosition[2]])}
          />
        </div>
        
        <div className="control-group">
          <label className="control-label">Camera Z</label>
          <input
            type="number"
            step="0.1"
            className="control-input"
            value={cameraPosition[2]}
            onChange={(e) => setCameraPosition([cameraPosition[0], cameraPosition[1], parseFloat(e.target.value)])}
          />
        </div>

        <button 
          className="btn btn-primary"
          onClick={handleRender}
          disabled={rendering}
          style={{ marginLeft: 'auto' }}
        >
          {rendering ? 'Rendering...' : '🎨 Render View'}
        </button>
      </div>

      {lastRender && (
        <div className="card" style={{ marginTop: '1rem' }}>
          <h3 style={{ marginBottom: '1rem' }}>Render Result</h3>
          <p style={{ color: 'var(--text-secondary)', marginBottom: '0.5rem' }}>
            Rendered in {lastRender.render_time_ms?.toFixed(2)}ms
          </p>
          <div style={{ 
            width: '100%', 
            maxWidth: '400px', 
            aspectRatio: '1/1', 
            backgroundColor: 'var(--bg-tertiary)',
            borderRadius: '8px',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center'
          }}>
            <span style={{ color: 'var(--text-secondary)' }}>Render preview</span>
          </div>
        </div>
      )}

      <div className="card" style={{ marginTop: '1rem' }}>
        <h3 style={{ marginBottom: '1rem' }}>Tips for Best Results</h3>
        <ul style={{ color: 'var(--text-secondary)', paddingLeft: '1.5rem', lineHeight: 1.8 }}>
          <li>Move the camera around to explore different viewpoints</li>
          <li>The NeRF model will synthesize novel views from your training photos</li>
          <li>For photorealistic results, train with 20-50 overlapping photos</li>
          <li>Capture your subject from all angles for complete coverage</li>
        </ul>
      </div>
    </>
  )
}
