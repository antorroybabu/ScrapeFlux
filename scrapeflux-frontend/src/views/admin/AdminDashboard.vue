<template>
  <div class="min-h-screen bg-gray-100">
    <!-- Sidebar -->
    <aside class="fixed left-0 top-0 h-screen w-64 bg-gray-900 text-white">
      <div class="p-6">
        <h1 class="text-2xl font-bold">ScrapeFlux Admin</h1>
      </div>
      <nav class="mt-6">
        <router-link to="/admin" class="flex items-center px-6 py-3 bg-gray-800 text-white">
          <svg class="w-5 h-5 mr-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"></path>
          </svg>
          Dashboard
        </router-link>
        <router-link to="/admin/users" class="flex items-center px-6 py-3 hover:bg-gray-800 text-gray-300">
          <svg class="w-5 h-5 mr-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z"></path>
          </svg>
          Users
        </router-link>
        <router-link to="/admin/subscriptions" class="flex items-center px-6 py-3 hover:bg-gray-800 text-gray-300">
          <svg class="w-5 h-5 mr-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z"></path>
          </svg>
          Subscriptions
        </router-link>
        <router-link to="/admin/scraping" class="flex items-center px-6 py-3 hover:bg-gray-800 text-gray-300">
          <svg class="w-5 h-5 mr-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9"></path>
          </svg>
          Scraping Jobs
        </router-link>
        <router-link to="/admin/emails" class="flex items-center px-6 py-3 hover:bg-gray-800 text-gray-300">
          <svg class="w-5 h-5 mr-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"></path>
          </svg>
          Email Campaigns
        </router-link>
        <router-link to="/admin/analytics" class="flex items-center px-6 py-3 hover:bg-gray-800 text-gray-300">
          <svg class="w-5 h-5 mr-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"></path>
          </svg>
          Analytics
        </router-link>
        <router-link to="/admin/settings" class="flex items-center px-6 py-3 hover:bg-gray-800 text-gray-300">
          <svg class="w-5 h-5 mr-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
          </svg>
          Settings
        </router-link>
      </nav>
    </aside>

    <!-- Main Content -->
    <main class="ml-64 p-8">
      <!-- Header -->
      <div class="flex justify-between items-center mb-8">
        <h1 class="text-3xl font-bold text-gray-900">Admin Dashboard</h1>
        <div class="flex items-center">
          <span class="text-gray-500 mr-4">Welcome, Admin</span>
          <button @click="logout" class="bg-red-600 text-white px-4 py-2 rounded-md hover:bg-red-700">
            Logout
          </button>
        </div>
      </div>

      <!-- Stats Cards -->
      <div class="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
        <div class="bg-white rounded-lg shadow p-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-gray-500 text-sm">Total Users</p>
              <p class="text-3xl font-bold text-gray-900">1,234</p>
            </div>
            <div class="bg-blue-100 p-3 rounded-full">
              <svg class="w-6 h-6 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z"></path>
              </svg>
            </div>
          </div>
          <p class="text-green-600 text-sm mt-2">+12% from last month</p>
        </div>

        <div class="bg-white rounded-lg shadow p-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-gray-500 text-sm">Active Subscriptions</p>
              <p class="text-3xl font-bold text-gray-900">892</p>
            </div>
            <div class="bg-green-100 p-3 rounded-full">
              <svg class="w-6 h-6 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
              </svg>
            </div>
          </div>
          <p class="text-green-600 text-sm mt-2">+8% from last month</p>
        </div>

        <div class="bg-white rounded-lg shadow p-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-gray-500 text-sm">Total Scrapes</p>
              <p class="text-3xl font-bold text-gray-900">2.4M</p>
            </div>
            <div class="bg-purple-100 p-3 rounded-full">
              <svg class="w-6 h-6 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9"></path>
              </svg>
            </div>
          </div>
          <p class="text-green-600 text-sm mt-2">+25% from last month</p>
        </div>

        <div class="bg-white rounded-lg shadow p-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-gray-500 text-sm">Emails Sent</p>
              <p class="text-3xl font-bold text-gray-900">456K</p>
            </div>
            <div class="bg-yellow-100 p-3 rounded-full">
              <svg class="w-6 h-6 text-yellow-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"></path>
              </svg>
            </div>
          </div>
          <p class="text-green-600 text-sm mt-2">+15% from last month</p>
        </div>
      </div>

      <!-- Revenue Chart -->
      <div class="bg-white rounded-lg shadow p-6 mb-8">
        <h2 class="text-xl font-semibold text-gray-900 mb-4">Revenue Overview</h2>
        <div class="h-64 bg-gray-100 rounded flex items-center justify-center">
          <p class="text-gray-500">Revenue Chart (Integration with Chart.js)</p>
        </div>
      </div>

      <!-- Recent Activity -->
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
        <!-- Recent Users -->
        <div class="bg-white rounded-lg shadow p-6">
          <h2 class="text-xl font-semibold text-gray-900 mb-4">Recent Users</h2>
          <div class="space-y-4">
            <div v-for="user in recentUsers" :key="user.id" class="flex items-center justify-between py-3 border-b">
              <div class="flex items-center">
                <div class="w-10 h-10 bg-indigo-100 rounded-full flex items-center justify-center mr-3">
                  <span class="text-indigo-600 font-semibold">{{ user.name[0] }}</span>
                </div>
                <div>
                  <p class="font-medium text-gray-900">{{ user.name }}</p>
                  <p class="text-sm text-gray-500">{{ user.email }}</p>
                </div>
              </div>
              <span :class="getPlanClass(user.plan)" class="px-3 py-1 rounded-full text-xs font-semibold">
                {{ user.plan }}
              </span>
            </div>
          </div>
        </div>

        <!-- Recent Scraping Jobs -->
        <div class="bg-white rounded-lg shadow p-6">
          <h2 class="text-xl font-semibold text-gray-900 mb-4">Recent Scraping Jobs</h2>
          <div class="space-y-4">
            <div v-for="job in recentJobs" :key="job.id" class="flex items-center justify-between py-3 border-b">
              <div>
                <p class="font-medium text-gray-900">{{ job.url }}</p>
                <p class="text-sm text-gray-500">{{ job.timestamp }}</p>
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
import { ref } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()

const recentUsers = ref([
  { id: 1, name: 'John Doe', email: 'john@example.com', plan: 'Professional' },
  { id: 2, name: 'Jane Smith', email: 'jane@example.com', plan: 'Starter' },
  { id: 3, name: 'Bob Wilson', email: 'bob@example.com', plan: 'Enterprise' },
  { id: 4, name: 'Alice Brown', email: 'alice@example.com', plan: 'Free' },
])

const recentJobs = ref([
  { id: 1, url: 'https://example.com/products', status: 'Completed', timestamp: '2 mins ago' },
  { id: 2, url: 'https://shop.com/catalog', status: 'Running', timestamp: '5 mins ago' },
  { id: 3, url: 'https://news.com/articles', status: 'Failed', timestamp: '10 mins ago' },
  { id: 4, url: 'https://blog.com/posts', status: 'Completed', timestamp: '15 mins ago' },
])

function getPlanClass(plan: string) {
  const classes: Record<string, string> = {
    'Free': 'bg-gray-100 text-gray-600',
    'Starter': 'bg-blue-100 text-blue-600',
    'Professional': 'bg-purple-100 text-purple-600',
    'Enterprise': 'bg-yellow-100 text-yellow-600',
  }
  return classes[plan] || 'bg-gray-100 text-gray-600'
}

function getStatusClass(status: string) {
  const classes: Record<string, string> = {
    'Completed': 'bg-green-100 text-green-600',
    'Running': 'bg-blue-100 text-blue-600',
    'Failed': 'bg-red-100 text-red-600',
    'Pending': 'bg-yellow-100 text-yellow-600',
  }
  return classes[status] || 'bg-gray-100 text-gray-600'
}

function logout() {
  localStorage.removeItem('token')
  router.push('/login')
}
</script>