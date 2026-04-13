import axios from 'axios'

const api = axios.create({
  baseURL: '/api/v1',
  headers: {
    'Content-Type': 'application/json',
  },
})

export interface Scene {
  id: string
  name: string
  description?: string
  status: 'pending' | 'training' | 'ready' | 'error'
  image_count: number
  training_progress?: number
  created_at: string
  updated_at: string
}

export interface SceneImage {
  id: string
  scene_id: string
  filename: string
  original_name: string
  width: number
  height: number
  url: string
  thumbnail_url: string
  uploaded_at: string
}

export const scenes = {
  list: () => api.get<Scene[]>('/scenes'),
  create: (data: { name: string; description?: string }) => api.post<Scene>('/scenes', data),
  get: (id: string) => api.get<Scene>(`/scenes/${id}`),
  update: (id: string, data: { name?: string; description?: string }) => 
    api.put<Scene>(`/scenes/${id}`, data),
  delete: (id: string) => api.delete(`/scenes/${id}`),
  getImages: (id: string) => api.get<SceneImage[]>(`/scenes/${id}/images`),
}

export const images = {
  upload: (sceneId: string, files: File[]) => {
    const formData = new FormData()
    files.forEach((file) => {
      formData.append('files', file)
    })
    return api.post('/images/upload', formData, {
      headers: { 'Content-Type': 'multipart/form-data' },
    })
  },
}

export const render = {
  create: (data: {
    scene_id: string
    camera_position: [number, number, number]
    camera_rotation: [number, number]
    width?: number
    height?: number
  }) => api.post('/render', data),
}

export const health = () => api.get('/health')
