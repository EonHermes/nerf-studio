import { useState, useEffect } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { scenes, images, Scene, SceneImage } from '../api/client'

export default function SceneDetailPage() {
  const { id } = useParams<{ id: string }>()
  const navigate = useNavigate()
  
  const [scene, setScene] = useState<Scene | null>(null)
  const [images, setImages] = useState<SceneImage[]>([])
  const [uploading, setUploading] = useState(false)

  useEffect(() => {
    if (id) {
      loadScene()
      loadImages()
    }
  }, [id])

  const loadScene = async () => {
    if (!id) return
    try {
      const response = await scenes.get(id)
      setScene(response.data)
    } catch (error) {
      console.error('Failed to load scene:', error)
    }
  }

  const loadImages = async () => {
    if (!id) return
    try {
      const response = await scenes.getImages(id)
      setImages(response.data)
    } catch (error) {
      console.error('Failed to load images:', error)
    }
  }

  const handleFileSelect = async (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!id || !e.target.files?.length) return
    
    setUploading(true)
    try {
      await images.upload(id, Array.from(e.target.files))
      await loadImages()
      await loadScene()
    } catch (error) {
      console.error('Failed to upload images:', error)
    } finally {
      setUploading(false)
    }
  }

  if (!scene) {
    return <div className="empty-state">Loading...</div>
  }

  const statusClass = `status-badge status-${scene.status}`

  return (
    <>
      <button 
        className="btn btn-secondary" 
        onClick={() => navigate('/')}
        style={{ marginBottom: '1.5rem' }}
      >
        ← Back to Scenes
      </button>

      <div className="card">
        <div className="card-header">
          <h2 className="card-title">{scene.name}</h2>
          <span className={statusClass}>{scene.status}</span>
        </div>
        
        {scene.description && (
          <p style={{ color: 'var(--text-secondary)', marginBottom: '1rem' }}>
            {scene.description}
          </p>
        )}

        <div style={{ display: 'flex', gap: '2rem', flexWrap: 'wrap' }}>
          <div>
            <strong>Images:</strong> {scene.image_count}
          </div>
          <div>
            <strong>Created:</strong> {new Date(scene.created_at).toLocaleDateString()}
          </div>
        </div>

        {scene.status === 'ready' && (
          <button 
            className="btn btn-primary" 
            onClick={() => navigate(`/render/${id}`)}
            style={{ marginTop: '1rem' }}
          >
            🎨 Start Rendering
          </button>
        )}
      </div>

      <div className="card">
        <h3 className="card-title" style={{ marginBottom: '1rem' }}>Upload Images</h3>
        
        <label className="upload-zone">
          <input 
            type="file" 
            multiple 
            accept="image/*"
            onChange={handleFileSelect}
            style={{ display: 'none' }}
          />
          <div className="upload-icon">📷</div>
          <div className="upload-text">
            {uploading ? 'Uploading...' : 'Click to upload images'}
          </div>
          <div className="upload-hint">
            Upload photos taken from different angles for best results
          </div>
        </label>
      </div>

      {images.length > 0 && (
        <div className="card">
          <h3 className="card-title" style={{ marginBottom: '1rem' }}>
            Uploaded Images ({images.length})
          </h3>
          <div className="image-grid">
            {images.map((img) => (
              <div key={img.id} className="image-item">
                <img src={img.thumbnail_url} alt={img.original_name} />
              </div>
            ))}
          </div>
        </div>
      )}

      {scene.image_count >= 5 && scene.status !== 'ready' && (
        <div className="card" style={{ borderColor: 'var(--accent-primary)' }}>
          <h3 className="card-title">Ready to Train</h3>
          <p style={{ color: 'var(--text-secondary)', marginBottom: '1rem' }}>
            You have {scene.image_count} images. For best results, use 10-50 photos taken from different angles around your subject.
          </p>
          <button className="btn btn-success">
            Start Training (Coming Soon)
          </button>
        </div>
      )}
    </>
  )
}
