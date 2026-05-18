import { defineStore } from 'pinia'
import axios from 'axios'

export interface Plan {
  id: string
  name: string
  monthly_price: number
  yearly_price: number
  features: string[]
  scrapes_per_month: number
  emails_per_month: number
  team_members: number
  api_access: boolean
  multi_ip: boolean
  priority_support: boolean
}

export const useSubscriptionStore = defineStore('subscription', {
  state: () => ({
    plans: [] as Plan[],
    currentSubscription: null as any,
    loading: false
  }),

  actions: {
    async fetchPlans() {
      this.loading = true
      try {
        const response = await axios.get('/api/subscriptions/plans')
        this.plans = response.data
      } finally {
        this.loading = false
      }
    },

    async subscribe(planId: string, paymentToken: string) {
      this.loading = true
      try {
        const response = await axios.post('/api/subscriptions/subscribe', {
          plan_id: planId,
          payment_token: paymentToken
        })
        this.currentSubscription = response.data
        return true
      } catch {
        return false
      } finally {
        this.loading = false
      }
    },

    async cancelSubscription() {
      this.loading = true
      try {
        await axios.post('/api/subscriptions/cancel')
        this.currentSubscription = null
        return true
      } catch {
        return false
      } finally {
        this.loading = false
      }
    }
  }
})