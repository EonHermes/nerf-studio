import { useState, useEffect } from 'react'
import { scenes, Scene } from '../api/client'
import CreateSceneModal from '../components/CreateSceneModal'

export default function ScenesPage() {
  const [scenes, setScenes] = useState<Scene[]>([])
  const [loading, setLoading] = useState(true)
  const [showCreateModal, setShowCreateModal] = useState(false)

  useEffect(() => {
    loadScenes()
  }, [])

  const loadScenes = async () => {
    try {
      const response = await scenes.list()
      setScenes(response.data)
    } catch (error) {
      console.error('Failed to load scenes:', error)
    } finally {
      setLoading(false)
    }
  }

  const handleDelete = async (id: string) => {
    if (!confirm('Are you sure you want to delete this scene?')) return
    
    try {
      await scenes.delete(id)
      setScenes(scenes.filter((s) => s.id !== id))
    } catch (error) {
      console.error('Failed to delete scene:', error)
    }
  }

  if (loading) {
    return <div className="empty-state">Loading scenes...</div>
  }

  return (
    <>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '2rem' }}>
        <div>
          <h2 style={{ fontSize: '1.75rem', fontWeight: 600 }}>Neural Radiance Fields Studio</h2>
          <p style={{ color: 'var(--text-secondary)', marginTop: '0.5rem' }}>
            Create photorealistic 3D scenes from photo collections
          </p>
        </div>
        <button className="btn btn-primary" onClick={() => setShowCreateModal(true)}>
          + New Scene
        </button>
      </div>

      {scenes.length === 0 ? (
        <div className="empty-state">
          <div className="empty-icon">📷</div>
          <h3 className="empty-title">No scenes yet</h3>
          <p className="empty-description">
            Create your first NeRF scene by uploading a collection of photos taken from different angles.
          </p>
        </div>
      ) : (
        <div className="scene-grid">
          {scenes.map((scene) => (
            <SceneCard key={scene.id} scene={scene} onDelete={handleDelete} />
          ))}
        </div>
      )}

      {showCreateModal && (
        <CreateSceneModal
          onClose={() => setShowCreateModal(false)}
          onSuccess={() => {
            setShowCreateModal(false)
            loadScenes()
          }}
        />
      )}
    </>
  )
}

function SceneCard({ scene, onDelete }: { scene: Scene; onDelete: (id: string) => void }) {
  const statusClass = `status-badge status-${scene.status}`
  
  return (
    <div className="scene-card">
      <div className="scene-thumbnail">🎨</div>
      <div className="scene-info">
        <h3 className="scene-name">{scene.name}</h3>
        {scene.description && <p className="scene-description">{scene.description}</p>}
        <div className="scene-meta">
          <span>{scene.image_count} images</span>
          <span className={statusClass}>{scene.status}</span>
        </div>
        <div style={{ marginTop: '1rem', display: 'flex', gap: '0.5rem' }}>
          {scene.status === 'ready' && (
            <a href={`/render/${scene.id}`} className="btn btn-primary" style={{ flex: 1, justifyContent: 'center' }}>
              Render
            </a>
          )}
          <button 
            className="btn btn-secondary" 
            onClick={() => onDelete(scene.id)}
            style={{ flex: 1 }}
          >
            Delete
          </button>
        </div>
      </div>
    </div>
  )
}
