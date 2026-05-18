<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Header -->
    <header class="bg-white shadow-sm">
      <nav class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4">
        <div class="flex justify-between items-center">
          <router-link to="/" class="text-2xl font-bold text-indigo-600">ScrapeFlux</router-link>
          <div class="flex items-center space-x-4">
            <router-link to="/login" class="text-gray-600 hover:text-gray-900">Sign in</router-link>
            <router-link to="/register" class="bg-indigo-600 text-white px-4 py-2 rounded-md hover:bg-indigo-700">
              Get Started
            </router-link>
          </div>
        </div>
      </nav>
    </header>

    <!-- Pricing Header -->
    <div class="bg-indigo-600 py-16">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
        <h1 class="text-4xl font-extrabold text-white">Choose Your Plan</h1>
        <p class="mt-4 text-xl text-indigo-100">
          Scale your web scraping with flexible pricing that grows with your needs
        </p>
      </div>
    </div>

    <!-- Pricing Cards -->
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
        <!-- Free Plan -->
        <div class="bg-white rounded-lg shadow-lg p-8">
          <h3 class="text-xl font-semibold text-gray-900">Free</h3>
          <p class="mt-2 text-gray-500">Get started with basic scraping</p>
          <div class="mt-4">
            <span class="text-4xl font-extrabold text-gray-900">$0</span>
            <span class="text-gray-500">/month</span>
          </div>
          <ul class="mt-6 space-y-4">
            <li class="flex items-center">
              <svg class="w-5 h-5 text-green-500 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
              </svg>
              100 scrapes/month
            </li>
            <li class="flex items-center">
              <svg class="w-5 h-5 text-green-500 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
              </svg>
              10 emails/month
            </li>
            <li class="flex items-center">
              <svg class="w-5 h-5 text-green-500 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
              </svg>
              1 team member
            </li>
            <li class="flex items-center text-gray-400">
              <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
              </svg>
              API access
            </li>
          </ul>
          <button
            @click="subscribe('free')"
            class="mt-8 w-full bg-gray-100 text-gray-900 py-2 px-4 rounded-md hover:bg-gray-200"
          >
            Start Free
          </button>
        </div>

        <!-- Starter Plan -->
        <div class="bg-white rounded-lg shadow-lg p-8 border-2 border-indigo-600">
          <div class="flex items-center justify-between">
            <h3 class="text-xl font-semibold text-gray-900">Starter</h3>
            <span class="bg-indigo-100 text-indigo-800 text-xs font-semibold px-2 py-1 rounded">Popular</span>
          </div>
          <p class="mt-2 text-gray-500">Perfect for small projects</p>
          <div class="mt-4">
            <span class="text-4xl font-extrabold text-gray-900">$29</span>
            <span class="text-gray-500">/month</span>
          </div>
          <ul class="mt-6 space-y-4">
            <li class="flex items-center">
              <svg class="w-5 h-5 text-green-500 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
              </svg>
              1,000 scrapes/month
            </li>
            <li class="flex items-center">
              <svg class="w-5 h-5 text-green-500 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
              </svg>
              500 emails/month
            </li>
            <li class="flex items-center">
              <svg class="w-5 h-5 text-green-500 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
              </svg>
              3 team members
            </li>
            <li class="flex items-center">
              <svg class="w-5 h-5 text-green-500 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
              </svg>
              API access
            </li>
          </ul>
          <button
            @click="subscribe('starter')"
            class="mt-8 w-full bg-indigo-600 text-white py-2 px-4 rounded-md hover:bg-indigo-700"
          >
            Get Started
          </button>
        </div>

        <!-- Professional Plan -->
        <div class="bg-white rounded-lg shadow-lg p-8">
          <h3 class="text-xl font-semibold text-gray-900">Professional</h3>
          <p class="mt-2 text-gray-500">For growing businesses</p>
          <div class="mt-4">
            <span class="text-4xl font-extrabold text-gray-900">$99</span>
            <span class="text-gray-500">/month</span>
          </div>
          <ul class="mt-6 space-y-4">
            <li class="flex items-center">
              <svg class="w-5 h-5 text-green-500 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
              </svg>
              10,000 scrapes/month
            </li>
            <li class="flex items-center">
              <svg class="w-5 h-5 text-green-500 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
              </svg>
              5,000 emails/month
            </li>
            <li class="flex items-center">
              <svg class="w-5 h-5 text-green-500 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
              </svg>
              10 team members
            </li>
            <li class="flex items-center">
              <svg class="w-5 h-5 text-green-500 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
              </svg>
              Multi-IP rotation
            </li>
            <li class="flex items-center">
              <svg class="w-5 h-5 text-green-500 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
              </svg>
              Priority support
            </li>
          </ul>
          <button
            @click="subscribe('professional')"
            class="mt-8 w-full bg-indigo-600 text-white py-2 px-4 rounded-md hover:bg-indigo-700"
          >
            Get Started
          </button>
        </div>

        <!-- Enterprise Plan -->
        <div class="bg-white rounded-lg shadow-lg p-8">
          <h3 class="text-xl font-semibold text-gray-900">Enterprise</h3>
          <p class="mt-2 text-gray-500">For large scale operations</p>
          <div class="mt-4">
            <span class="text-4xl font-extrabold text-gray-900">$299</span>
            <span class="text-gray-500">/month</span>
          </div>
          <ul class="mt-6 space-y-4">
            <li class="flex items-center">
              <svg class="w-5 h-5 text-green-500 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
              </svg>
              Unlimited scrapes
            </li>
            <li class="flex items-center">
              <svg class="w-5 h-5 text-green-500 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
              </svg>
              Unlimited emails
            </li>
            <li class="flex items-center">
              <svg class="w-5 h-5 text-green-500 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
              </svg>
              Unlimited team members
            </li>
            <li class="flex items-center">
              <svg class="w-5 h-5 text-green-500 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
              </svg>
              Custom integrations
            </li>
            <li class="flex items-center">
              <svg class="w-5 h-5 text-green-500 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
              </svg>
              Dedicated support
            </li>
          </ul>
          <button
            @click="subscribe('enterprise')"
            class="mt-8 w-full bg-indigo-600 text-white py-2 px-4 rounded-md hover:bg-indigo-700"
          >
            Contact Sales
          </button>
        </div>
      </div>
    </div>

    <!-- FAQ Section -->
    <div class="bg-gray-100 py-16">
      <div class="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8">
        <h2 class="text-3xl font-extrabold text-center text-gray-900">Frequently Asked Questions</h2>
        <div class="mt-8 space-y-6">
          <div>
            <h3 class="text-lg font-semibold text-gray-900">Can I change plans later?</h3>
            <p class="mt-2 text-gray-600">Yes, you can upgrade or downgrade your plan at any time. Changes take effect immediately.</p>
          </div>
          <div>
            <h3 class="text-lg font-semibold text-gray-900">What happens if I exceed my limits?</h3>
            <p class="mt-2 text-gray-600">You'll be notified when you're approaching your limits. You can upgrade or wait for the next billing cycle.</p>
          </div>
          <div>
            <h3 class="text-lg font-semibold text-gray-900">Do you offer refunds?</h3>
            <p class="mt-2 text-gray-600">We offer a 30-day money-back guarantee for all paid plans.</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'
import { useSubscriptionStore } from '@/stores/subscription'

const router = useRouter()
const subscriptionStore = useSubscriptionStore()

function subscribe(plan: string) {
  // For now, redirect to register/login
  if (plan === 'free') {
    router.push('/register?plan=free')
  } else {
    router.push('/register?plan=' + plan)
  }
}
</script>