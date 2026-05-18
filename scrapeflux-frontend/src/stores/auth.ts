import { defineStore } from 'pinia'
import axios from 'axios'

export interface User {
  id: string
  email: string
  first_name: string
  last_name: string
  role: 'free' | 'starter' | 'professional' | 'enterprise' | 'admin'
  subscription?: {
    plan: string
    status: string
    expires_at: string
  }
}

export const useAuthStore = defineStore('auth', {
  state: () => ({
    user: null as User | null,
    token: localStorage.getItem('token') || null,
    loading: false,
    error: null as string | null
  }),

  getters: {
    isAuthenticated: (state) => !!state.token,
    isAdmin: (state) => state.user?.role === 'admin',
    fullName: (state) => state.user ? `${state.user.first_name} ${state.user.last_name}` : ''
  },

  actions: {
    async login(email: string, password: string) {
      this.loading = true
      this.error = null
      try {
        const response = await axios.post('/api/auth/login', { email, password })
        this.token = response.data.token
        this.user = response.data.user
        localStorage.setItem('token', this.token!)
        axios.defaults.headers.common['Authorization'] = `Bearer ${this.token}`
        return true
      } catch (e: any) {
        this.error = e.response?.data?.message || 'Login failed'
        return false
      } finally {
        this.loading = false
      }
    },

    async register(data: { email: string; password: string; first_name: string; last_name: string }) {
      this.loading = true
      this.error = null
      try {
        const response = await axios.post('/api/auth/register', data)
        this.token = response.data.token
        this.user = response.data.user
        localStorage.setItem('token', this.token!)
        axios.defaults.headers.common['Authorization'] = `Bearer ${this.token}`
        return true
      } catch (e: any) {
        this.error = e.response?.data?.message || 'Registration failed'
        return false
      } finally {
        this.loading = false
      }
    },

    logout() {
      this.user = null
      this.token = null
      localStorage.removeItem('token')
      delete axios.defaults.headers.common['Authorization']
    },

    async fetchUser() {
      if (!this.token) return
      try {
        const response = await axios.get('/api/auth/me')
        this.user = response.data
      } catch {
        this.logout()
      }
    }
  }
})