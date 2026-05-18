<template>
  <div class="min-h-screen bg-gray-100">
    <!-- Header -->
    <header class="bg-white shadow-sm">
      <nav class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4">
        <div class="flex justify-between items-center">
          <router-link to="/" class="text-2xl font-bold text-indigo-600">ScrapeFlux</router-link>
          <div class="flex items-center space-x-4">
            <router-link to="/dashboard" class="text-gray-600 hover:text-gray-900">Dashboard</router-link>
            <button @click="logout" class="text-gray-600 hover:text-gray-900">Logout</button>
          </div>
        </div>
      </nav>
    </header>

    <main class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <h1 class="text-3xl font-bold text-gray-900 mb-8">Profile Settings</h1>

      <!-- Profile Form -->
      <div class="bg-white rounded-lg shadow p-6 mb-8">
        <h2 class="text-xl font-semibold text-gray-900 mb-4">Personal Information</h2>
        <form class="space-y-4">
          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-medium text-gray-700">First Name</label>
              <input type="text" v-model="form.first_name" class="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2" />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700">Last Name</label>
              <input type="text" v-model="form.last_name" class="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2" />
            </div>
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700">Email</label>
            <input type="email" v-model="form.email" class="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2" readonly />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700">Company</label>
            <input type="text" v-model="form.company" class="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2" />
          </div>
          <button type="submit" class="bg-indigo-600 text-white px-4 py-2 rounded-md hover:bg-indigo-700">
            Save Changes
          </button>
        </form>
      </div>

      <!-- Subscription -->
      <div class="bg-white rounded-lg shadow p-6 mb-8">
        <h2 class="text-xl font-semibold text-gray-900 mb-4">Subscription</h2>
        <div class="flex justify-between items-center">
          <div>
            <p class="text-lg font-semibold">{{ subscription.plan }}</p>
            <p class="text-gray-500">${{ subscription.price }}/month</p>
          </div>
          <router-link to="/pricing" class="bg-indigo-600 text-white px-4 py-2 rounded-md hover:bg-indigo-700">
            Upgrade Plan
          </router-link>
        </div>
      </div>

      <!-- Change Password -->
      <div class="bg-white rounded-lg shadow p-6">
        <h2 class="text-xl font-semibold text-gray-900 mb-4">Change Password</h2>
        <form class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700">Current Password</label>
            <input type="password" v-model="passwordForm.current" class="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2" />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700">New Password</label>
            <input type="password" v-model="passwordForm.new" class="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2" />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700">Confirm New Password</label>
            <input type="password" v-model="passwordForm.confirm" class="mt-1 block w-full border border-gray-300 rounded-md px-3 py-2" />
          </div>
          <button type="submit" class="bg-indigo-600 text-white px-4 py-2 rounded-md hover:bg-indigo-700">
            Update Password
          </button>
        </form>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const router = useRouter()
const authStore = useAuthStore()

const form = ref({
  first_name: authStore.user?.first_name || '',
  last_name: authStore.user?.last_name || '',
  email: authStore.user?.email || '',
  company: ''
})

const subscription = ref({
  plan: 'Starter',
  price: 29
})

const passwordForm = ref({
  current: '',
  new: '',
  confirm: ''
})

function logout() {
  authStore.logout()
  router.push('/login')
}
</script>