import { createRouter, createWebHistory } from 'vue-router'
import Home from '../views/Home.vue'
import Announcement from '../views/Announcement.vue'
import Tasks from '../views/Tasks.vue'
import Extensions from '../views/Extensions.vue'
import Console from '../views/Console.vue'
import Settings from '../views/Settings.vue'

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home
  },
  {
    path: '/announcement',
    name: 'Announcement',
    component: Announcement
  },
  {
    path: '/tasks',
    name: 'Tasks',
    component: Tasks
  },
  {
    path: '/extensions',
    name: 'Extensions',
    component: Extensions
  },
  {
    path: '/console',
    name: 'Console',
    component: Console
  },
  {
    path: '/settings',
    name: 'Settings',
    component: Settings
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router