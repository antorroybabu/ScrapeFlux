<template>
  <div class="min-h-screen bg-gray-100">
    <!-- Header -->
    <header class="bg-white shadow-sm">
      <nav class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4">
        <div class="flex justify-between items-center">
          <router-link to="/" class="text-2xl font-bold text-indigo-600">ScrapeFlux</router-link>
          <div class="flex items-center space-x-4">
            <span class="text-gray-600">{{ user?.email }}</span>
            <button @click="logout" class="text-gray-600 hover:text-gray-900">Logout</button>
          </div>
        </div>
      </nav>
    </header>

    <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <h1 class="text-3xl font-bold text-gray-900 mb-8">Dashboard</h1>

      <!-- Stats -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
        <div class="bg-white rounded-lg shadow p-6">
          <p class="text-gray-500 text-sm">Scrapes Used</p>
          <p class="text-3xl font-bold text-gray-900">{{ stats.scrapes_used }} / {{ stats.scrapes_limit }}</p>
          <div class="mt-2 w-full bg-gray-200 rounded-full h-2">
            <div class="bg-indigo-600 h-2 rounded-full" :style="{ width: stats.scrapes_percent + '%' }"></div>
          </div>
        </div>
        <div class="bg-white rounded-lg shadow p-6">
          <p class="text-gray-500 text-sm">Emails Sent</p>
          <p class="text-3xl font-bold text-gray-900">{{ stats.emails_sent }} / {{ stats.emails_limit }}</p>
          <div class="mt-2 w-full bg-gray-200 rounded-full h-2">
            <div class="bg-green-600 h-2 rounded-full" :style="{ width: stats.emails_percent + '%' }"></div>
          </div>
        </div>
        <div class="bg-white rounded-lg shadow p-6">
          <p class="text-gray-500 text-sm">Current Plan</p>
          <p class="text-3xl font-bold text-indigo-600">{{ stats.plan }}</p>
        </div>
      </div>

      <!-- Quick Actions -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
        <router-link to="/scrape" class="bg-indigo-600 text-white rounded-lg shadow p-6 hover:bg-indigo-700">
          <svg class="w-8 h-8 mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9"></path>
          </svg>
          <h3 class="text-lg font-semibold">New Scrape</h3>
          <p class="text-indigo-200">Start a new scraping job</p>
        </router-link>
        <router-link to="/contacts" class="bg-green-600 text-white rounded-lg shadow p-6 hover:bg-green-700">
          <svg class="w-8 h-8 mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"></path>
          </svg>
          <h3 class="text-lg font-semibold">Contacts</h3>
          <p class="text-green-200">Manage your contacts</p>
        </router-link>
        <router-link to="/emails" class="bg-purple-600 text-white rounded-lg shadow p-6 hover:bg-purple-700">
          <svg class="w-8 h-8 mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"></path>
          </svg>
          <h3 class="text-lg font-semibold">Email Campaigns</h3>
          <p class="text-purple-200">Create email campaigns</p>
        </router-link>
      </div>

      <!-- Recent Jobs -->
      <div class="bg-white rounded-lg shadow">
        <div class="px-6 py-4 border-b">
          <h2 class="text-xl font-semibold text-gray-900">Recent Scraping Jobs</h2>
        </div>
        <div class="p-6">
          <div v-if="jobs.length === 0" class="text-center text-gray-500 py-8">
            No jobs yet. Start your first scrape!
          </div>
          <div v-else class="space-y-4">
            <div v-for="job in jobs" :key="job.id" class="flex items-center justify-between py-3 border-b">
              <div>
                <p class="font-medium text-gray-900">{{ job.url }}</p>
                <p class="text-sm text-gray-500">{{ job.date }}</p>
              </div>
              <span :class="getStatusClass(job.status)" class="px-3 py-1 rounded-full text-xs font-semibold">
                {{ job.status }}
              </span>
            </div>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const router = useRouter()
const authStore = useAuthStore()

const user = ref(authStore.user)

const stats = ref({
  scrapes_used: 450,
  scrapes_limit: 1000,
  scrapes_percent: 45,
  emails_sent: 120,
  emails_limit: 500,
  emails_percent: 24,
  plan: 'Starter'
})

const jobs = ref([
  { id: 1, url: 'https://example.com/products', status: 'Completed', date: '2 hours ago' },
  { id: 2, url: 'https://shop.com/catalog', status: 'Running', date: '3 hours ago' },
  { id: 3, url: 'https://news.com/articles', status: 'Completed', date: '1 day ago' },
])

function getStatusClass(status: string) {
  const classes: Record<string, string> = {
    'Completed': 'bg-green-100 text-green-600',
    'Running': 'bg-blue-100 text-blue-600',
    'Failed': 'bg-red-100 text-red-600',
  }
  return classes[status] || 'bg-gray-100 text-gray-600'
}

function logout() {
  authStore.logout()
  router.push('/login')
}

onMounted(() => {
  if (!authStore.isAuthenticated) {
    router.push('/login')
  }
})
</script>