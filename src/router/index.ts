import { createRouter, createWebHistory } from 'vue-router'
import RolePage from '@/pages/sidebar/RolePage.vue'
import ItemsPage from '@/pages/sidebar/ItemsPage.vue'
import EventsPage from '@/pages/sidebar/EventsPage.vue'
import TradingPage from '@/pages/sidebar/TradingPage.vue'
import MapPage from '@/pages/sidebar/MapPage.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      redirect: '/role',
    },
    {
      path: '/role',
      name: 'Role',
      component: RolePage,
      meta: { title: 'Role' },
    },
    {
      path: '/items',
      name: 'Items',
      component: ItemsPage,
      meta: { title: 'Items' },
    },
    {
      path: '/events',
      name: 'Events',
      component: EventsPage,
      meta: { title: 'Events' },
    },
    {
      path: '/trading',
      name: 'Trading',
      component: TradingPage,
      meta: { title: 'Trading' },
    },
    {
      path: '/map',
      name: 'Map',
      component: MapPage,
      meta: { title: 'Map' },
    },
  ],
})

export default router
